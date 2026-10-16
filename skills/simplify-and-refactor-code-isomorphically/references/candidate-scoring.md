# Candidate Scoring

Score candidates before editing. The goal is to spend autonomy on simplifications with real leverage.

## Score Fields

- **LOC saved**: expected net reduction, including tests/docs/config when relevant.
- **Concepts removed**: count of abstractions, code paths, options, files, or states eliminated.
- **Confidence**: probability the behavior is understood.
- **Verification strength**: how well tests/goldens/manual checks cover the behavior.
- **Risk**: chance of hidden behavior drift, security regression, or migration damage.
- **Blast radius**: number of modules, call sites, public contracts, and runtime entrypoints touched.

## Ruthless Defaults

Ruthless mode accepts candidates that are large, medium-risk, or cross-cutting when the proof is strong enough.

Prefer candidates that:

- remove an entire path, abstraction, file, option family, or duplicate implementation
- reduce future agent confusion
- make tests simpler while preserving user-facing assertions
- improve security by deleting bypasses, stale fallbacks, or inconsistent validation
- unlock more simplification rounds

Reject candidates when:

- confidence is below 70% and verification cannot be strengthened quickly
- behavior equivalence depends on "probably"
- the main payoff is style preference
- the diff is large but does not remove concepts
- the candidate mixes behavior change with refactor

## Aggressive Mode

Aggressive mode is still bold, but it raises the threshold for broad rewrites:

- prefer one module, one path, or one abstraction family at a time
- require confidence above 80% for deletion
- avoid cross-package public API reshapes unless tests or consumers prove equivalence
- stop after the first clean verified simplification round unless the user asks to continue

## Rejection Ledger

Record rejected candidates so future passes do not rediscover them:

- candidate
- tempting simplification
- rejection reason
- what evidence would change the decision

Short rejection notes are enough.
