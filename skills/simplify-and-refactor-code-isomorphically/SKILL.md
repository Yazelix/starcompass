---
name: simplify-and-refactor-code-isomorphically
description: Ruthlessly de-slopify and refactor agent-written code by reducing code volume, collapsing accidental complexity, deleting provably unnecessary paths, and proving observable behavior is preserved. Use when the user asks to simplify, reduce LOC, de-bloat, de-slopify, clean up agent code, remove duplication, collapse abstractions, delete dead code, or refactor aggressively without changing behavior.
---

# Simplify And Refactor Code Isomorphically

## Overview

Treat simplification as a proof obligation: the smaller program must be observably equivalent to the larger one. Be aggressive about deleting complexity, but conservative about claiming safety.

Default to `ruthless` mode. Use `aggressive` only when the user explicitly asks for a more bounded pass.

## Modes

### Ruthless

Use by default. Hunt for large accretive simplifications across the selected scope:

- delete dead code, orphaned files, stale alternate implementations, unused exports, obsolete compatibility paths, and duplicated helpers when evidence supports it
- collapse wrapper layers, parameterized clones, accidental abstraction stacks, redundant types, unnecessary config, and branches for impossible states
- rewrite module internals when that removes meaningful complexity while preserving public contracts
- use structural codemods or automated rewrites when they are scoped, reviewable, and followed by verification
- keep going through multiple simplification rounds until marginal candidates no longer clear the proof bar

### Aggressive

Use when the user requests lower blast radius. Still simplify boldly, but keep each change closer to one module or one behavioral surface, and prefer smaller verified reductions over broad rewrites.

## Workflow

1. Rebuild context:
   - read applicable project instructions
   - inspect `git status` and current diffs
   - understand the requested scope, entrypoints, tests, public APIs, generated files, and security-sensitive paths
2. Establish the baseline:
   - run existing tests, typechecks, linters, snapshot/golden checks, or app-specific verification commands
   - capture relevant command outputs, CLI/API/UI outputs, file counts, LOC, warnings, complexity signals, and failing-test state
   - use `scripts/snapshot_baseline.sh` when a lightweight baseline artifact helps
3. Map simplification candidates:
   - read `references/pathology-catalog.md`
   - classify duplication and bloat instead of relying on taste
   - inspect call sites before declaring code unused
4. Score candidates:
   - read `references/candidate-scoring.md`
   - rank by LOC removed, complexity removed, confidence, risk, and verification strength
   - reject weak candidates and record why
5. Fill an isomorphism card before editing:
   - read `references/isomorphism-card.md`
   - prove expected equivalence for outputs, errors, ordering, logs, metrics, side effects, lifecycle, serialization, async behavior, hooks, permissions, and security posture
6. Edit with power:
   - prefer direct, coherent edits over timid micro-churn
   - make broad changes when the proof card and baseline support them
   - keep unrelated behavior changes out of the refactor
   - when deleting, remove the entire obsolete path: code, tests for that path, docs references, exports, imports, and config references
7. Verify:
   - rerun the baseline commands and any targeted checks that cover the changed behavior
   - use `scripts/compare_outputs.py` for deterministic file or directory output comparisons
   - use `scripts/changed_file_metrics.sh` to report net simplification
8. Iterate:
   - perform another ruthless pass if verification holds and more high-value candidates remain
   - stop when remaining candidates are mostly speculative, low-payoff, or under-verified
9. Report an audit ledger:
   - what changed and what got smaller
   - why behavior is equivalent
   - verification commands and results
   - rejected simplifications and why
   - residual risk

## Power Gates

Dangerous simplifications are allowed when they pass a proof gate:

- **Delete code/files** when call-site search, exports, config, runtime entrypoints, tests, and docs show the path is unreachable or obsolete.
- **Broad rewrites** when public contracts are identified and baseline coverage or golden outputs can catch drift.
- **Structural codemods** when the matcher is semantic, the diff is reviewable, and verification is rerun afterward.
- **Test deletion or rewrite** when the test only preserves deleted implementation detail, not user-visible behavior.
- **Security cleanup** when removing complexity cannot widen permissions, weaken validation, suppress errors, skip escaping, or reduce auditability.

If a gate cannot be proven, do not silently downgrade the standard. Reject the candidate or isolate it as a behavior-changing proposal.

## Hard Constraints

- Preserve observable behavior unless the user explicitly asks for behavior change.
- Never call a behavior-changing cleanup "isomorphic".
- Do not hide verification failures behind unrelated fixes.
- Do not use regex churn when structure matters; use AST-aware tools or manual edits.
- Do not simplify away validation, authorization, escaping, error semantics, cancellation, cleanup, migrations, telemetry, or resource lifecycles unless equivalence is proven.
- Respect generated-file rules. Update generators rather than hand-editing generated outputs.
- Respect active system, user, repo, and tool instructions. If an active instruction requires confirmation for a destructive operation, gather the proof and ask once with the exact operation.

## References

- `references/pathology-catalog.md`: agent-written code bloat patterns to hunt.
- `references/isomorphism-card.md`: proof checklist before edits.
- `references/candidate-scoring.md`: ruthless/aggressive candidate thresholds.
- `references/verification-playbook.md`: ecosystem-specific verification ideas.

## Scripts

- `scripts/snapshot_baseline.sh [out-dir] [-- command ...]`: capture repo context, metrics, and optional command output.
- `scripts/changed_file_metrics.sh [file ...]`: summarize line counts and diff stats for changed or named files.
- `scripts/compare_outputs.py <before> <after>`: byte-compare files or directory trees and report drift.
