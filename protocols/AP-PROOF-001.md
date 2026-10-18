---
id: AP-PROOF-001
version: 9
title: Explicit proof lifecycle
---

Proof records revision, command/observation, environment, result/surface,
and phase: proposed, implemented, mechanically verified, dogfooded, accepted,
or promoted. Never widen/reuse after material change.

Use the highest boundary exercising the changed contract. Add broader/lower
suites only for changed owner, failure, trust, distinct delivery, or
promotion—not availability, issue text, or prior phase. Exact downstream proof
subsumes lower checks only for the identity/behavior exercised.

Run a complete suite once per unchanged candidate/environment. Metadata/docs/
planning follow-ups reuse it; commands are evidence, not a checklist. Keep logs
only when reproduction is costly, disputed, performance-sensitive, or lossily
summarized; temporary paths are not proof. Unattended runs and waits close
unused stdin, time out, check progress early, and never match themselves as the
target; stalls fail.

Performance claims need a measured bottleneck, fixed workload/environment,
baseline, oracle, and distribution/bound; retain constraining negatives. Review
changes require another pass; convergence needs a full clean pass. Nits never
block it, and text-only fixes restart a review at most twice per slice.
