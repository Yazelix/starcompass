# Verification Playbook

Use the strongest verification available for the touched behavior. Passing tests are not the only proof; golden outputs, CLI snapshots, API responses, and UI checks can be better.

## Generic Baseline

- `git status --short`
- `git diff --stat`
- project test command
- project typecheck/lint command
- targeted command or fixture output for the behavior being simplified
- LOC and changed-file metrics

## Ecosystem Checks

- **JavaScript/TypeScript**: `bun test`, `bun run typecheck`, `bun run lint`, `tsc --noEmit`, route/API snapshots, Playwright for UI behavior.
- **Python**: `pytest`, `ruff check`, `mypy` or `pyright`, CLI output snapshots, fixture comparisons.
- **Rust**: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, CLI golden output.
- **Go**: `go test ./...`, `go vet ./...`, API/CLI golden output.
- **Frontend**: screenshot comparison, DOM text checks, keyboard/focus behavior, hydration warnings, accessibility snapshots.
- **Data/schema**: migration dry runs, fixture round-trips, serialized JSON/schema comparison, backward-compatible decode checks.

## When Baseline Is Already Red

If the baseline fails before edits:

- capture the failure exactly
- do not "fix" unrelated failures as part of the refactor
- use targeted checks for the changed surface
- require that post-refactor failures are identical or narrower

## Output Comparison

Use `scripts/compare_outputs.py` for deterministic file and directory comparisons. For nondeterministic outputs, normalize only the volatile fields explicitly and document the normalization.

## Security Verification

For auth, validation, escaping, secrets, command execution, file paths, network calls, and parsing:

- identify the security invariant before editing
- verify the invariant after editing
- do not remove "redundant" checks unless an upstream check is mandatory and tested
- preserve audit logs and error visibility
