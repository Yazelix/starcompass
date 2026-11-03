#!/usr/bin/env bash
# Claude Code SessionStart hook (matcher "compact"). After each compaction it
# points the agent back to AP-SESSION-001 and counts this session's
# compactions against that protocol's cap, which an agent cannot count across
# summaries. Claude Code adds the hook's stdout to the model's context.
# Always exits 0: a broken counter must not stop the session.
set -u

cap=5 # AP-SESSION-001 v6: hand off after a session's fifth compaction.
base=${XDG_STATE_HOME:-${HOME:+$HOME/.local/state}}
state=${base:+$base/starcompass/compactions}
reread="reread AGENTS.md and the in-progress issue's latest checkpoint before continuing"
uncounted="Context was compacted. Follow AP-SESSION-001: $reread."

id=$(sed -n 's/.*"session_id"[[:space:]]*:[[:space:]]*"\([A-Za-z0-9_-]\{1,128\}\)".*/\1/p' | head -n 1)
if [ -z "$id" ] || [ -z "$state" ] || ! mkdir -p "$state" 2>/dev/null; then
  echo "$uncounted"
  exit 0
fi

find "$state" -type f -mtime +7 -delete 2>/dev/null
count=$(cat "$state/$id" 2>/dev/null)
case $count in '' | *[!0-9]*) count=0 ;; esac
count=$((10#$count + 1))
if ! { echo "$count" >"$state/$id"; } 2>/dev/null; then
  echo "$uncounted"
  exit 0
fi

echo "Context was compacted: compaction $count of $cap in this session. Follow AP-SESSION-001: $reread."
if [ "$count" -eq "$cap" ]; then
  echo "This was the last compaction AP-SESSION-001 allows: from 300k, hand off instead of compacting again."
elif [ "$count" -gt "$cap" ]; then
  echo "This session is past AP-SESSION-001's cap of $cap compactions: hand off now."
fi
exit 0
