#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -gt 0 ]]; then
  files=("$@")
else
  mapfile -t files < <(git diff --name-only --diff-filter=ACMRT 2>/dev/null || true)
fi

printf 'file\tlines\n'
for file in "${files[@]}"; do
  [[ -f "$file" ]] || continue
  lines=$(wc -l < "$file" | tr -d ' ')
  printf '%s\t%s\n' "$file" "$lines"
done

printf '\n'
git diff --stat -- "${files[@]}" 2>/dev/null || true
