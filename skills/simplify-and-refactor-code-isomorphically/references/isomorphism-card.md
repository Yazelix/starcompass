# Isomorphism Card

Fill this before every meaningful refactor. A short answer is fine, but each row must be considered.

## Candidate

- ID/title:
- Files:
- Current behavior:
- Proposed smaller behavior:
- Public contract:
- Expected LOC/complexity reduction:

## Equivalence Rows

| Surface | Same? | Proof |
| --- | --- | --- |
| Return values / rendered output / emitted files |  |  |
| Error types, messages, status codes, and exit codes |  |  |
| Ordering, sorting, pagination, and stability |  |  |
| Logs, metrics, telemetry, audit records |  |  |
| Side effects, writes, network calls, process exits |  |  |
| Auth, permissions, escaping, validation, security checks |  |  |
| Serialization, schemas, wire formats, persisted data |  |  |
| Async behavior, cancellation, retries, timers, cleanup |  |  |
| Resource lifecycle: files, locks, sockets, subscriptions |  |  |
| React/UI identity: hooks, keys, refs, focus, hydration |  |  |
| Performance envelope and memory behavior |  |  |
| Test and fixture meaning |  |  |

## Decision

- Accept / reject:
- Why:
- Verification before:
- Verification after:
- Residual risk:

If any row is unknown and important, add targeted verification or reject the candidate.
