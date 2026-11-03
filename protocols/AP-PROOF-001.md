---
id: AP-PROOF-001
version: 12
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

Finish every change to code, tests, or scripts with one full review by fresh
agents that did not write it, a second model too where available and behavior
changes, since authors miss their own leftovers; in Claude Code, use the
converge skill. Docs-only and planning-only work skips it. The author fixes or
rebuts each finding; its finder re-checks only those fixes and rebuttals. Rerun
the full review only when a fix changes behavior beyond its finding or crosses
a trust boundary. Nits never block, and text-only fixes need no re-check.
