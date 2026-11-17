#!/usr/bin/env bash
# Prove hooks/after-compact.sh: per-session counts, the cap messages, and a
# reminder without a count for missing or unsafe session IDs or a counter it
# cannot write; bounded decisions replay and child isolation. Then prove
# hooks/block-wide-search.py's decisions by piping
# tool-call JSON into it under a temporary HOME; no case runs a search. Uses a
# temporary state directory.
set -euo pipefail

hooks=$(cd "$(dirname "$0")" && pwd)
hook=$hooks/after-compact.sh
state=$(mktemp -d)
trap 'chmod -R u+w "$state"; rm -rf "$state"' EXIT
unset STARCOMPASS_STATE_DIR
replay_dir=

run_compact() {
  printf '%s' "$1" | STARCOMPASS_STATE_DIR=$replay_dir XDG_STATE_HOME=$state "$hook"
}

failed=0
expect() {
  local want=$1 input=$2 unwanted=${3:-} out
  out=$(run_compact "$input") || {
    echo "hook exited $? for: $input"
    failed=1
    return
  }
  if [[ $out != *"$want"* ]] || { [ -n "$unwanted" ] && [[ $out == *"$unwanted"* ]]; }; then
    echo "expected \"$want\"${unwanted:+ without \"$unwanted\"} for: $input"
    echo "got: $out"
    failed=1
  fi
}

a='{"session_id":"abc-123","hook_event_name":"SessionStart","source":"compact"}'
for n in 1 2 3 4; do
  expect "compaction $n of 5" "$a" "hand off"
done
expect "last compaction AP-SESSION-001 allows: hand off now" "$a" "300k"
expect "past AP-SESSION-001's cap of 5" "$a"
expect "compaction 1 of 5" '{"source": "compact", "session_id": "other_9"}'

uncounted="Context was compacted. Follow"
expect "$uncounted" '{"session_id":"../escape","source":"compact"}'
expect "$uncounted" '{"source":"compact"}'
expect "$uncounted" ''
if [ -e "$state/starcompass/escape" ]; then
  echo "an unsafe session ID wrote outside the counter folder"
  failed=1
fi

# Every invocation overrides the launch environment; no live decisions are read.
expect 'STARCOMPASS_STATE_DIR is not set' '{"session_id":"no-replay"}'
replay_dir="$state/lead state"
mkdir -p "$replay_dir"
decisions=$replay_dir/standing-decisions.md
block=$'# Standing decisions\n2026-11-17: "continue means go"\nLast decision.'
printf '%s\n' "$block" >"$decisions"
expect "Standing decisions from $decisions:" '{"session_id":"replay"}'
expect "$block" '{"session_id":"replay"}'
expect "$block" '{"session_id":"../escape"}'
expect "$block" ''
# Counter creation failure must still replay, even when running as root.
state_before=$state
state=$state/counter-is-a-file
printf x >"$state"
expect "$block" '{"session_id":"counter-failure"}'
state=$state_before
out=$(printf '%s' '{"session_id":"child"}' |
  STARCOMPASS_STATE_DIR=$replay_dir XDG_STATE_HOME=$state env -u STARCOMPASS_STATE_DIR "$hook")
if [[ $out != *'compaction 1 of 5'* || $out != *'STARCOMPASS_STATE_DIR is not set'* || $out == *"$block"* ]]; then
  echo 'child did not retain its counter while clearing decisions replay'
  failed=1
fi

# The header follows the counter; the entire small block appears, not a summary.
out=$(run_compact '{"session_id":"ordering"}')
if [[ $out != "Context was compacted: compaction 1 of 5"*$'\n'"Standing decisions from $decisions:"$'\n'"$block" ]]; then
  echo 'decisions replay was incomplete or preceded the counter'
  failed=1
fi
python3 -I -c 'import sys; sys.stdout.write("x" * 8192)' >"$decisions"
at_limit=$(cat "$decisions")
expect "$at_limit" '{"session_id":"exact-limit"}' 'truncated'
# The first omitted byte is a newline: command substitution must not hide it.
printf '\nOMITTED\n' >>"$decisions"
expect "truncated at 8192 bytes; read $decisions in full" '{"session_id":"over-limit"}' 'OMITTED'
out=$(run_compact '{"session_id":"bounded"}')
if [[ $out != *"Standing decisions from $decisions:"$'\n'"$at_limit"$'\n'"Standing decisions truncated at 8192 bytes; read $decisions in full." ]]; then
  echo 'decisions replay did not preserve exactly the first 8192 bytes'
  failed=1
fi
# Exercise the byte limit under a UTF-8 locale, including a split character.
if [ "$(LC_ALL=C.UTF-8 locale charmap 2>/dev/null)" = UTF-8 ]; then
  printf '%s\303\251\n' "${at_limit:1}" >"$decisions"
  out=$(LC_ALL=C.UTF-8 run_compact '{"session_id":"utf8-boundary"}')
  prefix=${at_limit:1}$'\303'
  if [[ $out != *"Standing decisions from $decisions:"$'\n'"$prefix"$'\n'"Standing decisions truncated at 8192 bytes; read $decisions in full." ]]; then
    echo 'UTF-8 replay did not respect the byte limit'
    failed=1
  fi
fi
: >"$decisions"
expect 'compaction 1 of 5' '{"session_id":"empty"}' 'Standing decisions from'
rm "$decisions"
expect "Standing decisions unavailable at $decisions" '{"session_id":"missing-file"}' 'Standing decisions from'
mkdir "$decisions"
expect 'compaction 1 of 5' '{"session_id":"directory"}' 'Standing decisions from'
rmdir "$decisions"
printf '%s\n' "$block" >"$decisions"
if [ "$(id -u)" != 0 ]; then
  chmod a-r "$decisions"
  expect 'compaction 1 of 5' '{"session_id":"unreadable"}' 'Standing decisions from'
  chmod u+r "$decisions"
fi
replay_dir=

counters=$state/starcompass/compactions
echo junk >"$counters/corrupt"
expect "compaction 1 of 5" '{"session_id":"corrupt"}'
echo 08 >"$counters/octal"
expect "compaction 9 of 5" '{"session_id":"octal"}'

if [ "$(id -u)" != 0 ]; then # root writes through mode bits
  chmod a-w "$counters"
  expect "$uncounted" '{"session_id":"unwritable"}'
  chmod u+w "$counters"
fi

# block-wide-search.py: "deny" or "allow" for one tool call.
home=$state/home
mkdir -p "$home/.cache/eon-agents/eon-lead" "$home/repo/src"
roots='/:~:~/.cache:~/.cache/eon-agents'
decide() {
  local want=$1 payload=$2 out got=allow
  out=$(printf '%s' "$payload" |
    HOME=$home STARCOMPASS_BLOCKED_SEARCH_ROOTS=$roots python3 -I "$hooks/block-wide-search.py") || {
    echo "guard exited $? for: $payload"
    failed=1
    return
  }
  if [[ $out == *'"permissionDecision": "deny"'* ]]; then
    got=deny
  elif [ -n "$out" ]; then
    got="output: $out"
  fi
  if [ "$got" != "$want" ]; then
    echo "guard: expected $want, got $got for: $payload"
    failed=1
  fi
}
tool_call() { # tool cwd key value
  python3 -I -c 'import json, sys
t, cwd, k, v = sys.argv[1:]
print(json.dumps({"tool_name": t, "cwd": cwd, "tool_input": {k: v}}))' "$@"
}
bash_case() { decide "$1" "$(tool_call Bash "${3:-$home/repo}" command "$2")"; }

while IFS='|' read -r want command; do
  bash_case "$want" "$command"
done <<'CASES'
deny|bash -c "grep -r foo ~/.cache"
deny|sh -c 'rg x ~'
deny|du -sh ~/.cache
deny|du ~
deny|du -h --max-depth=3 ~/.cache/eon-agents
deny|grep -rIl boat ~/.cache/eon-agents
deny|cd ~/.cache/eon-agents && grep -rl .
deny|cd ~/.cache/eon-agents; for d in */; do grep -rIl -i boat "$d"; done
deny|grep -r foo ~/.cache/eon-agents/*
deny|find ~/.cache/eon-agents -name x
deny|ionice -c3 grep -r x ~/.cache
deny|rg foo ~/.cache/eon-agents/ | head
deny|du -h -d 1 ~/.cache
deny|du -d 1 -h ~/.cache
deny|du --max-depth=2 ~
deny|rg foo //
deny|grep -rl x ~/.cache
deny|grep -rl x ~/.cache/eon-agents
deny|cd ~ && rg foo
allow|du -sh ~/.cache/eon-agents/eon-lead
allow|df -h ~
allow|bash -c "grep -r foo src"
allow|ps aux | grep claude
allow|grep -rn / src
allow|bash -lc "ls ~"
allow|grep -rn yazelix orbit/docs
allow|for c in 'ps aux | grep claude' 'grep -rn / src'; do echo "$c"; done
allow|git ls-files | xargs grep -Il yazelix
allow|cd ~/.cache/eon-agents; grep -l boat eon-lead/*.md
allow|find ~/.cache/eon-agents -maxdepth 1 -name x
allow|rg foo src/ && git status
allow|rg foo ~/repo/src
allow|cd ~ && rg foo repo
allow|cd ~/.cache/eon-agents && rg foo eon-lead
deny|rg --files ~/.cache/eon-agents
deny|rg --files /
deny|nice -n 19 rg foo ~
deny|ionice -c 3 rg foo ~
deny|timeout -s TERM 5 rg foo /
deny|time du -sh ~/.cache
deny|env LC_ALL=C grep -r x ~
deny|nohup rg foo ~ &
deny|grep -r x ~/.*
deny|cd /tmp && cd .. && rg foo
deny|cd /; bash -c "rg foo"
deny|rg --max-depth 2 --max-depth 10 foo /
allow|grep -rn -e '$HOME' src
allow|rg -t sh '$HOME' src
allow|rg -n -e / src
allow|grep -rn -A 3 / src
allow|rg -g "*.md" "~" src
allow|fd / src
allow|rg foo .; cd ~
allow|cd ~; cd ~/repo; rg foo
allow|rg -d2 foo /
allow|rg --maxdepth=2 foo /
allow|fd -d2 foo /
allow|command -v rg
deny|rg -efoo /
deny|grep -refoo /
deny|eval "cd /"; rg foo
deny|stdbuf -o L rg foo /
deny|time -p rg foo ~
deny|/usr/bin/time -v rg foo ~
deny|rg -e -d2 /
deny|fd -- -d2 /
deny|cd ~ && grep -rl x 2>/dev/null
deny|rg foo >out ~
allow|cd /; eval "cd /tmp"; rg foo
allow|rg --color never / src
allow|grep -rn --context 3 / src
CASES
bash_case deny $'rg -n pattern \\\n  ~/.cache/eon-agents'

# A root blocks only a search rooted exactly at it, wherever the agent stands.
bash_case deny 'rg foo .' "$home"
bash_case allow 'rg foo src/' "$home/repo"
bash_case allow 'rg foo repo/src' "$home"
bash_case allow 'rg foo $HOME/repo' "$home"
bash_case allow 'ps aux | rg claude' "$home"
bash_case deny 'cd ~ && rg foo </dev/null'
bash_case deny 'rg foo <&-' "$home"
bash_case deny 'rg foo 2>/dev/null' "$home"
bash_case deny 'git log | grep -r foo' "$home"
bash_case deny 'rg -e tmp' /
bash_case deny 'fd tmp' /
decide deny "$(tool_call Grep "$home" pattern x)"
decide deny "$(tool_call Grep "$home/repo" path "$home/.cache/")"
decide allow "$(tool_call Grep "$home" path repo)"
decide deny "$(tool_call Glob "$home" pattern '**/*.rs')"
decide allow "$(tool_call Glob "$home/repo" pattern '**/*.rs')"
decide allow "$(tool_call Glob "$home" pattern '*.md')"
decide deny "$(tool_call Glob "$home/repo" pattern "$home/.cache/eon-agents/**/*.rs")"
decide allow "$(tool_call Glob "$home" pattern 'repo/**/*.rs')"

# Bad input fails open.
decide allow '{bad'
decide allow '[1]'
decide allow '{"tool_name": "Bash", "tool_input": {"command": 7}}'

# The variable is the whole list; empty, the roots are / and ~.
roots=
bash_case deny 'rg x ~'
bash_case deny 'find / -name x'
bash_case allow 'rg x ~/.cache'
roots='~/.cache'
bash_case allow 'rg x ~'
bash_case deny 'rg x ~/.cache'

if [ "$failed" = 0 ]; then
  echo "hooks: all cases pass"
fi
exit "$failed"
