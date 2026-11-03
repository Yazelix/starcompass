#!/usr/bin/env bash
# Link every skill beside this script into each installed agent host (Claude
# Code, Codex), and the repo's dcg packs into dcg's pack folder when dcg's
# config folder exists. An existing entry that is not already this repo's link
# is moved to a timestamped backup first. Safe to rerun.
set -euo pipefail

skills=$(cd "$(dirname "$0")" && pwd)
backup="${XDG_STATE_HOME:-$HOME/.local/state}/skills-backup/$(date +%Y%m%dT%H%M%S)"

link() {
  local source=$1 target=$2 group=$3
  [ "$(readlink "$target" 2>/dev/null)" = "$source" ] && return
  if [ -e "$target" ] || [ -L "$target" ]; then
    mkdir -p "$backup/$group"
    mv "$target" "$backup/$group/"
    echo "backed up $target"
  fi
  ln -s "$source" "$target"
  echo "linked $target"
}

for host in "$HOME/.codex" "$HOME/.claude"; do
  [ -d "$host" ] || continue
  mkdir -p "$host/skills"
  for skill in "$skills"/*/; do
    source=${skill%/}
    link "$source" "$host/skills/$(basename "$source")" "$(basename "$host")"
  done
done

dcg_config="$HOME/.config/dcg"
if [ -d "$dcg_config" ]; then
  mkdir -p "$dcg_config/packs"
  for pack in "$(dirname "$skills")"/guards/dcg/*.yaml; do
    link "$pack" "$dcg_config/packs/$(basename "$pack")" dcg
  done
fi
