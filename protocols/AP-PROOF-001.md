---
id: AP-PROOF-001
version: 10
title: Explicit proof lifecycle
---

Proof records the revision, command or observation, environment, and result
with its surface. Never widen or reuse proof after a material change.

Use the highest boundary that exercises the changed contract. Add broader or
lower suites only for a changed owner, failure, trust boundary, distinct
delivery, or promotion, not because a suite exists, an issue lists it, or it
ran before. Exact downstream proof covers lower checks only for the identity
and behavior it exercised.

Run a complete suite once per unchanged candidate and environment; metadata,
docs, and planning follow-ups reuse it. Commands are evidence, not a checklist.
Keep logs only when reproduction is costly, disputed, or performance-sensitive,
or a summary loses detail; a temporary path is not proof. Unattended runs and
waits close unused stdin, time out, check progress early, and never match
themselves as the target; stalls fail.

Review changes require another pass; convergence needs a full clean pass. Nits
never block it, and text-only fixes restart a review at most twice per slice.
