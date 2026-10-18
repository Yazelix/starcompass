#!/usr/bin/env bash
# Link every skill beside this script into each installed agent host (Claude
# Code, Codex). An existing skill folder that is not already this repo's link is
# moved to a timestamped backup first. Safe to rerun.
set -euo pipefail

skills=$(cd "$(dirname "$0")" && pwd)
backup="${XDG_STATE_HOME:-$HOME/.local/state}/skills-backup/$(date +%Y%m%dT%H%M%S)"

for host in "$HOME/.codex" "$HOME/.claude"; do
  [ -d "$host" ] || continue
  mkdir -p "$host/skills"
  for skill in "$skills"/*/; do
    source=${skill%/}
    target="$host/skills/$(basename "$source")"
    [ "$(readlink "$target" 2>/dev/null)" = "$source" ] && continue
    if [ -e "$target" ] || [ -L "$target" ]; then
      mkdir -p "$backup/$(basename "$host")"
      mv "$target" "$backup/$(basename "$host")/"
      echo "backed up $target"
    fi
    ln -s "$source" "$target"
    echo "linked $target"
  done
done
