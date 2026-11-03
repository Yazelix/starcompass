#!/usr/bin/env bash
# Prove hooks/after-compact.sh: per-session counts, the cap messages, and a
# reminder without a count for missing or unsafe session IDs or a counter it
# cannot write. Uses a temporary state directory.
set -euo pipefail

hook=$(cd "$(dirname "$0")" && pwd)/after-compact.sh
state=$(mktemp -d)
trap 'chmod -R u+w "$state"; rm -rf "$state"' EXIT

failed=0
expect() {
  local want=$1 input=$2 unwanted=${3:-} out
  out=$(printf '%s' "$input" | XDG_STATE_HOME=$state "$hook") || {
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
expect "last compaction" "$a"
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

if [ "$failed" = 0 ]; then
  echo "hooks: all cases pass"
fi
exit "$failed"
