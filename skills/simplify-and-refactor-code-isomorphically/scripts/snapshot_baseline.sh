#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--" || "$#" -eq 0 ]]; then
  out_dir=".codex/isomorphic-refactor/baseline-$(date -u +%Y%m%dT%H%M%SZ)"
else
  out_dir="$1"
  shift
fi

mkdir -p "$out_dir"

{
  printf 'timestamp_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf 'pwd=%s\n' "$(pwd)"
  git rev-parse --show-toplevel 2>/dev/null | sed 's/^/git_root=/' || true
  git rev-parse HEAD 2>/dev/null | sed 's/^/git_head=/' || true
} > "$out_dir/context.txt"

git status --short > "$out_dir/git-status.txt" 2>&1 || true
git diff --stat > "$out_dir/git-diff-stat.txt" 2>&1 || true

if command -v rg >/dev/null 2>&1; then
  rg --files > "$out_dir/files.txt"
else
  find . -type f > "$out_dir/files.txt"
fi

awk '
  /\.(c|cc|cpp|cs|css|go|h|hpp|html|java|js|jsx|kt|md|py|rs|sh|sql|svelte|ts|tsx|vue|yaml|yml)$/ {
    files += 1
    while ((getline line < $0) > 0) lines += 1
    close($0)
  }
  END {
    printf "counted_files=%d\ncounted_lines=%d\n", files, lines
  }
' "$out_dir/files.txt" > "$out_dir/line-counts.txt"

if [[ "${1:-}" == "--" ]]; then
  shift
  if [[ "$#" -gt 0 ]]; then
    set +e
    {
      printf '$'
      printf ' %q' "$@"
      printf '\n'
      "$@"
    } > "$out_dir/command-output.txt" 2>&1
    exit_code="$?"
    set -e
    printf '%s\n' "$exit_code" > "$out_dir/command-exit-code.txt"
  fi
fi

printf '%s\n' "$out_dir"
