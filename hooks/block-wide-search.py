#!/usr/bin/env python3
"""Claude Code PreToolUse hook (matcher "Grep|Glob|Bash") for AP-SEARCH-001.

Denies a recursive search rooted exactly at a blocked root: grep -r, ugrep -r,
rg, find and fd without a depth limit of 2 or less, any du (its -d limits only
output), the Grep tool, and the Glob tool with a ** pattern. A search below a
root, such as rg in a repository under ~, passes. It walks the command in
order, following cd, root/* globs, scripts passed to sh -c and its kin, and
wrappers such as nice, ionice, timeout, env, time and xargs.

STARCOMPASS_BLOCKED_SEARCH_ROOTS lists the roots, colon-separated, with ~ and
$HOME expanded; it replaces the default, / and ~. Never runs the command, and
any bad input or internal error allows the call. A heuristic, not a sandbox.
"""
import json
import os
import re
import shlex
import sys

HOME = os.path.expanduser("~")
REASON = (
    "Blocked by AP-SEARCH-001: recursive search rooted at {where}. One sweep "
    "there can saturate a shared disk and stall the machine. Search one named "
    "folder below it, grep named files non-recursively (grep -l pat dir/*), or "
    "bound the depth (find -maxdepth 2)."
)

SEARCH = {"grep", "egrep", "fgrep", "ugrep", "rg", "find", "fd", "du"}
SHELLS = {"bash", "sh", "zsh", "dash", "nu"}
# Wrappers, each with its options whose value is the next token.
WRAPPERS = {
    "sudo": {"-u", "-g", "-C", "-D", "-h", "-p", "-r", "-t", "-U", "--user", "--group"},
    "env": {"-u", "-C", "--unset", "--chdir"},
    "nice": {"-n", "--adjustment"},
    "ionice": {"-c", "-n", "-p", "--class", "--classdata", "--pid"},
    "timeout": {"-s", "-k", "--signal", "--kill-after"},
    "nohup": set(),
    "stdbuf": {"-i", "-o", "-e", "--input", "--output", "--error"},
    "time": {"-f", "-o", "--format", "--output"},
    "xargs": {"-I", "-d", "-P", "-L", "-n", "-s", "-a", "-E", "--max-args",
              "--max-procs", "--delimiter", "--arg-file"},
    "command": set(),
    "exec": {"-a"},
}
# Search options whose value is the next token, so it is neither pattern nor path.
CONTEXT = {"-A", "-B", "-C", "-m", "--context", "--after-context", "--before-context",
           "--max-count"}
GREP_VALUES = CONTEXT | {"-e", "-f", "-d", "-D", "--regexp", "--file", "--include",
                         "--exclude", "--exclude-dir", "--binary-files"}
VALUES = {
    "rg": CONTEXT | {"-e", "-f", "-g", "-t", "-T", "-M", "-j", "-r", "-E", "-d", "--regexp",
                     "--file", "--glob", "--iglob", "--type", "--type-not", "--max-columns",
                     "--threads", "--replace", "--encoding", "--max-depth", "--maxdepth",
                     "--max-filesize", "--sort", "--sortr", "--color", "--colors"},
    "fd": {"-e", "-t", "-E", "-d", "-j", "-S", "-x", "-X", "--extension", "--type",
           "--exclude", "--max-depth", "--maxdepth", "--min-depth", "--exact-depth",
           "--threads", "--size", "--max-results"},
    "du": {"-d", "-B", "-t", "-X", "--max-depth", "--block-size", "--threshold",
           "--exclude", "--exclude-from"},
}
PATTERN_OPTIONS = {"-e", "-f", "--regexp", "--file"}
DEPTH = {"rg": {"-d", "--max-depth", "--maxdepth"},
         "fd": {"-d", "--max-depth", "--maxdepth", "--exact-depth"}}
# Shell keywords that start a command without being one.
KEYWORDS = {"do", "then", "else", "elif", "if", "while", "until", "{", "}", "!"}


def normalize(p):
    p = os.path.normpath(p)
    return "/" + p.lstrip("/") if p.startswith("//") else p


def expand(tok):
    return os.path.expanduser(tok.replace("${HOME}", HOME).replace("$HOME", HOME))


def blocked_roots():
    roots = set()
    for entry in os.environ.get("STARCOMPASS_BLOCKED_SEARCH_ROOTS", "").split(":"):
        p = expand(entry.strip())
        if os.path.isabs(p):
            roots.add(normalize(p))
    return roots or {"/", normalize(HOME)}


BLOCKED = blocked_roots()


def resolve(tok, cwd):
    """Absolute path of a token, or None when it is relative to an unknown cwd."""
    p = expand(tok)
    if not os.path.isabs(p):
        if cwd is None:
            return None
        p = os.path.join(cwd, p)
    return normalize(p)


def blocked_root(tok, cwd):
    """Return the blocked root a path token names exactly, or None."""
    for t in (tok, re.sub(r"(/\.?\*+)+/?$", "", tok.rstrip("/")) or "/"):
        p = resolve(t, cwd)  # the second form covers root/* and root/.* globs
        if p in BLOCKED:
            return p
    return None


def deny(where):
    print(json.dumps({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": REASON.format(where=where),
        }
    }))
    sys.exit(0)


def parse(tool, args):
    """The path operands of a search command and its last depth limit, skipping
    options, their values, and the pattern."""
    if tool == "find":  # paths come after -H/-L/-P and before the expression
        rest = list(args)
        while rest and re.fullmatch(r"-[HLP]|-O\d*", rest[0]):
            rest.pop(0)
        paths = []
        while rest and not rest[0].startswith("-") and rest[0] not in ("(", "!", ","):
            paths.append(rest.pop(0))
        depth = None
        for i, a in enumerate(rest[:-1]):
            if a == "-maxdepth":
                depth = rest[i + 1]
        return paths, depth
    values, depth_flags = VALUES.get(tool, GREP_VALUES), DEPTH.get(tool, set())
    pattern = tool != "du" and not (tool == "rg" and "--files" in args)
    positional, depth, it, opts = [], None, iter(args), True
    for a in it:
        if not opts or not a.startswith("-") or a == "-":
            positional.append(a)
            continue
        if a == "--":
            opts = False
            continue
        name = val = None
        if a.startswith("--"):
            name, eq, val = a.partition("=")
            if not eq and name in values:
                val = next(it, "")
        else:  # short options: the first that takes a value takes the rest or the next token
            for i, ch in enumerate(a[1:], 1):
                if "-" + ch in values:
                    name, val = "-" + ch, a[i + 1:] or next(it, "")
                    break
        if name in depth_flags:
            depth = val
        if name in PATTERN_OPTIONS and tool != "fd":
            pattern = False  # -e/-f give the pattern
    return (positional[1:] if pattern else positional), depth


def recursive(tool, args, depth):
    if tool == "du":  # -d/--max-depth limits output only; du still walks everything
        return True
    if tool in ("rg", "find", "fd"):
        try:
            return int(depth) > 2
        except (TypeError, ValueError):
            return True
    for a in args:  # grep / egrep / fgrep / ugrep
        if a in ("--recursive", "--dereference-recursive"):
            return True
        if a.startswith("--directories=recurse") or a == "recurse":
            return True
        if re.fullmatch(r"-[A-Za-z]*[rR][A-Za-z]*", a):
            return True
    return False


def segments(cmd):
    """Split a shell command into simple commands, honouring quotes. Yields each
    command's tokens, without redirects, and whether a pipe feeds its stdin. A <
    redirect does not count: rg given </dev/null or <&- still searches the cwd."""
    cmd = cmd.replace("\\\n", " ").replace("\n", ";")
    try:
        lex = shlex.shlex(cmd, posix=True, punctuation_chars=";&|()<>")
        lex.whitespace_split = True
        lex.commenters = "#"
        toks = list(lex)
    except ValueError:
        toks = cmd.split()
    seg, piped, target = [], False, False
    for t in toks:
        if target:  # a redirect's file or descriptor
            target = False
        elif t and set(t) <= set("<>&") and set(t) & set("<>"):
            if seg and seg[-1].isdigit():  # the 2 of 2>/dev/null
                seg.pop()
            target = True
        elif t and set(t) <= set(";&|()<>"):
            if seg:
                yield seg, piped
            seg, piped = [], t in ("|", "|&")
        elif seg or t not in KEYWORDS:
            seg.append(t)
    if seg:
        yield seg, piped


def strip_wrappers(toks):
    """Drop env assignments and wrapper commands with their options; report xargs."""
    via_xargs = False
    while toks:
        name = os.path.basename(toks[0])
        if re.match(r"^\w+=", toks[0]):
            toks = toks[1:]
        elif name == "command" and toks[1:2] in (["-v"], ["-V"]):
            return [], False  # only looks the command up
        elif name in WRAPPERS:
            via_xargs |= name == "xargs"
            toks = toks[1:]
            while toks and toks[0].startswith("-") and toks[0] != "-":
                opt, toks = toks[0], toks[1:]
                if opt == "--":
                    break
                if opt in WRAPPERS[name]:
                    toks = toks[1:]
            if name == "timeout" and toks and re.fullmatch(r"\d+(\.\d+)?[smhd]?", toks[0]):
                toks = toks[1:]
        else:
            break
    return toks, via_xargs


def check_bash(cmd, cwd, depth=0):
    """Walk the command in order and return the cwd it leaves; cwd is None once a
    cd goes somewhere unknown."""
    for toks, piped in segments(cmd):
        toks, via_xargs = strip_wrappers(toks)
        if not toks:
            continue
        tool, args = os.path.basename(toks[0]), toks[1:]
        if tool in ("cd", "pushd"):
            dirs = [a for a in args if not a.startswith("-")]
            target = dirs[0] if dirs else "~"
            cwd = None if "$" in expand(target) else resolve(target, cwd)
            continue
        if tool in SHELLS and depth < 3:  # bash -c "grep -r ..." runs in a child
            for i, a in enumerate(args):
                if re.fullmatch(r"-[A-Za-z]*c[A-Za-z]*", a) and i + 1 < len(args):
                    check_bash(args[i + 1], cwd, depth + 1)
                    break
            continue
        if tool == "eval" and depth < 3:  # eval runs in this shell
            cwd = check_bash(" ".join(args), cwd, depth + 1)
            continue
        if tool not in SEARCH:
            continue
        paths, limit = parse(tool, args)
        if not recursive(tool, args, limit):
            continue
        has_path = has_var = False
        for a in paths:
            r = blocked_root(a, cwd)
            if r:
                deny(r)
            if "$" in expand(a):
                has_var = True
            else:
                has_path = True
        # with no path it searches the cwd (rg reads a pipe instead; xargs adds paths)
        implicit = not has_path and not via_xargs and not (tool == "rg" and piped)
        if cwd in BLOCKED and (has_var or implicit):
            deny(cwd)
    return cwd


def check_glob(pattern, base):
    """Deny a ** pattern whose fixed leading directories end at a blocked root."""
    m = re.search(r"[*?\[{]", pattern)
    fixed = pattern[:m.start()] if m else pattern
    prefix = fixed[:fixed.rfind("/") + 1]
    r = blocked_root(prefix or ".", base)
    if r and "**" in pattern[len(prefix):]:
        deny(r)


def main():
    data = json.load(sys.stdin)
    tool = data.get("tool_name")
    inp = data.get("tool_input") or {}
    cwd = data.get("cwd") or os.getcwd()
    if tool == "Grep":
        r = blocked_root(inp.get("path") or cwd, cwd)
        if r:
            deny(r)
    elif tool == "Glob":
        check_glob(inp.get("pattern") or "", resolve(inp.get("path") or cwd, cwd))
    elif tool == "Bash":
        check_bash(inp.get("command", ""), cwd)


if __name__ == "__main__":
    try:
        main()
    except Exception:  # fail open: a broken guard must not stop the agent
        pass
