#!/usr/bin/env python3
"""List the dependencies and pub/export lines added since a base revision.

Usage: new_surface.py <base>

Covers `git diff <base>` plus untracked files. Prints one finding per line and
nothing when there is none. The list is review input, not a gate.
"""

import fnmatch
import json
import os
import re
import subprocess
import sys
import tomllib

PUBLIC = re.compile(r"\s*(pub|export)\b")
NAME = re.compile(r"\s*([A-Za-z0-9][A-Za-z0-9._-]*)")


def git(*args, check=True):
    """A git command's stdout; with check=False, "" on a quiet failure."""
    quiet = None if check else subprocess.DEVNULL
    run = subprocess.run(["git", *args], stdout=subprocess.PIPE, stderr=quiet, text=True, check=check)
    return run.stdout if run.returncode == 0 else ""


def names(node):
    """Dependency names under every key that names dependencies."""
    found = set()
    if isinstance(node, dict):
        for key, value in node.items():
            if "dependenc" in key.lower() or key == "requires":
                found |= listed(value)
            else:
                found |= names(value)
    return found


def listed(value):
    if isinstance(value, list):  # PEP 508 strings
        return {m[1] for v in value if isinstance(v, str) and (m := NAME.match(v))}
    if isinstance(value, dict):  # name tables, or groups of PEP 508 lists
        return set().union(*(listed(v) if isinstance(v, list) else {k} for k, v in value.items()))
    return set()


def dependencies(path, text):
    if not text:
        return set()
    if path.endswith(".txt"):
        lines = (line.split("#")[0] for line in text.splitlines())
        return {m[1] for line in lines if not line.lstrip().startswith("-") and (m := NAME.match(line))}
    return names(json.loads(text) if path.endswith(".json") else tomllib.loads(text))


def read(path):
    try:
        with open(path, encoding="utf-8", errors="replace") as file:
            return file.read()
    except (FileNotFoundError, IsADirectoryError):  # deleted, or a nested repository
        return ""


def main(base):
    os.chdir(git("rev-parse", "--show-toplevel").strip())
    diff = git("diff", "--no-color", "--no-ext-diff", "--no-prefix", "-U0", base)
    untracked = git("ls-files", "-z", "--others", "--exclude-standard").split("\0")[:-1]
    changed = git("diff", "--name-only", "-z", base).split("\0")[:-1] + untracked

    for path in changed:
        name = os.path.basename(path)
        if name in ("Cargo.toml", "package.json", "pyproject.toml") or fnmatch.fnmatch(name, "requirements*.txt"):
            before = dependencies(path, git("show", f"{base}:{path}", check=False))
            for dep in sorted(dependencies(path, read(path)) - before):
                print(f"{path}: new dependency {dep}")

    path, line = None, 0
    for row in diff.splitlines():
        if row.startswith("diff --git "):
            path = None
        elif path is None and row.startswith("+++ "):
            path = row[4:].rstrip("\t")  # git appends a tab when the path has a space
        elif row.startswith("@@ "):
            line = int(re.match(r"@@ -\S+ \+(\d+)", row)[1])
        elif path and row.startswith("+"):
            if PUBLIC.match(row[1:]):
                print(f"{path}:{line}: {row[1:].strip()}")
            line += 1
    for path in untracked:
        text = read(path)
        if "\0" in text:  # binary
            continue
        for number, row in enumerate(text.splitlines(), 1):
            if PUBLIC.match(row):
                print(f"{path}:{number}: {row.strip()}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("usage: new_surface.py <base>")
    main(sys.argv[1])
