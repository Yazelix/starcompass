#!/usr/bin/env bash
# Prove the Starcompass dcg pack: dcg, with this pack added to its default
# packs and the POSIX dialect the Bash hook uses, must give every case below its
# expected decision.
set -euo pipefail

pack=$(cd "$(dirname "$0")" && pwd)/starcompass-waits.yaml
config=$(mktemp)
trap 'rm -f "$config"' EXIT
printf '[packs]\ncustom_paths = ["%s"]\n' "$pack" >"$config"

failed=0
expect() {
  local want=$1 command=$2 out status=0 got
  out=$(DCG_CONFIG=$config dcg test --robot --dialect posix "$command" 2>/dev/null) || status=$?
  case $status in
    0) got=allow ;;
    1) got=block ;;
    *) got="error $status" ;;
  esac
  if [ "$got" = block ] && [[ $out != *starcompass.waits:self-matching-wait* ]]; then
    got="block by another rule"
  fi
  if [ "$got" != "$want" ]; then
    echo "expected $want, got $got: $command"
    failed=1
  fi
}

# Self-matching waits, including the recorded incidents (ap-ouv, ap-lll).
expect block "until ! (ss -ltn | grep -q ':5174 ') && ! pgrep -f 'blueprinter-x8x9.*playwright' >/dev/null; do sleep 2; done; echo free"
expect block "until ! pgrep -f 'nova build'; do sleep 5; done"
expect block "bash -c 'until ! pgrep -f xyz; do sleep 1; done'"
expect block "bash -lc 'until ! pgrep -f xyz; do sleep 1; done'"
expect block "sh -ec \"while pgrep -f xyz; do sleep 1; done\""
expect block "timeout 60 bash -c 'until ! pgrep -f xyz; do sleep 1; done'"
expect block "while pgrep --full 'cargo test' >/dev/null; do sleep 2; done"
expect block "while true; do pkill -0 -f server || break; sleep 1; done"
expect block "while pgrep -u lucca -f x; do :; done"
expect block "until test -f 'done' || ! pgrep -f worker; do sleep 1; done"
expect block "while true; do pgrep -f worker || break; test -f 'done'; sleep 1; done"
expect block "while pgrep -f 'worker -A'; do sleep 1; done"
expect block "while pgrep -f 'worker.*[x]'; do sleep 1; done"

# Correct waits, safe selectors, and lookups that are not waits.
expect allow "timeout 120 bash -c 'while kill -0 1234; do sleep 1; done'"
expect allow "timeout 60 bash -c 'until ss -ltn | grep -q :5174; do sleep 1; done'"
expect allow "timeout 60 bash -c 'while pgrep -Af worker >/dev/null; do sleep 1; done'"
expect allow "while pgrep -fx worker; do sleep 1; done"
expect allow "while pgrep --full --exact worker; do sleep 1; done"
expect allow "while pgrep -f '[w]orker'; do sleep 1; done"
expect allow "while pgrep -f \"[w]orker\"; do sleep 1; done"
expect allow "until docker ps | grep -q db; do sleep 1; done"
expect allow "pgrep -af foo"
expect allow "pgrep -f 'cargo build'"
expect allow "while read -r p; do pgrep -f \"\$p\"; done < patterns.txt"
expect allow "while read -r f; do wc -l \"\$f\"; done < list; pgrep -f server"

# Commands that only quote a wait loop as data.
loop="until ! pgrep -f 'nova build'; do sleep 5; done"
expect allow "br comments add ap-x \"$loop\" && br sync --flush-only"
expect allow "dcg explain \"$loop\" 2>&1 | grep -E 'Decision'"
expect allow "git commit -m \"$loop\" && git log -1"

if [ "$failed" = 0 ]; then
  echo "dcg pack: all cases pass"
fi
exit "$failed"
