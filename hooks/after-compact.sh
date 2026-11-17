#!/usr/bin/env bash
# SessionStart hook (matcher "compact"). After each compaction it
# replays standing decisions, points back to AP-SESSION-001, and counts this session's
# compactions against that protocol's cap, which an agent cannot count across
# summaries. The host adds the hook's stdout to the model's context.
# Always exits 0: counter or replay failures must not stop the session.
set -u

# Isolate counter failures so decisions replay still runs.
count_compactions() (
  cap=5 # AP-SESSION-001 v7: hand off after a session's fifth compaction.
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
    echo "This was the last compaction AP-SESSION-001 allows: hand off now instead of compacting again."
  elif [ "$count" -gt "$cap" ]; then
    echo "This session is past AP-SESSION-001's cap of $cap compactions: hand off now."
  fi
)
count_compactions || :

replay_decisions() {
  local folder=${STARCOMPASS_STATE_DIR:-} file block= LC_ALL=C
  if [ -z "$folder" ]; then
    echo 'STARCOMPASS_STATE_DIR is not set: if this is a lead or desk session, your standing decisions were not replayed; read <state dir>/standing-decisions.md.'
    return
  fi
  file=$folder/standing-decisions.md
  if ! [ -f "$file" ] || ! [ -r "$file" ]; then
    printf 'Standing decisions unavailable at %s; read the full file when available.\n' "$file"
    return 0
  fi
  # One extra byte detects truncation without reading an unbounded file.
  { IFS= read -r -N 8193 block <"$file"; } 2>/dev/null || :
  [ -n "$block" ] || return 0
  printf 'Standing decisions from %s:\n%s\n' "$file" "${block:0:8192}"
  if [ "${#block}" -gt 8192 ]; then
    printf 'Standing decisions truncated at 8192 bytes; read %s in full.\n' "$file"
  fi
}
replay_decisions || :
exit 0
