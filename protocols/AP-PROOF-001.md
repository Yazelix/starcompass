---
id: AP-PROOF-001
version: 4
title: Explicit proof lifecycle
---

Proof supports only its recorded command or observation, revision, environment,
result, and exercised surface. Distinguish proposed, implemented, mechanically
verified, dogfooded, accepted, and promoted states; rerun stale proof and never
widen its claim.

For performance claims, measure the bottleneck, compare the same representative
workload and environment with a recorded baseline, retain a correctness oracle,
and report a distribution or bound. Preserve constraining negative results.

A review or simplification pass that materially changes its subject invalidates
completion. Repeat it on the revised state and declare convergence only after a
full pass finds no actionable in-scope bugs or simplifications.
