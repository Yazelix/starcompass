---
id: AP-PROOF-001
version: 2
title: Explicit proof lifecycle
---

Claims and proofs have a lifecycle. A passing check supports only the exact
revision, environment, and surface it exercised.

Required practice:

- Record the command or observation, relevant environment, revision, and result.
- Distinguish proposed, implemented, mechanically verified, manually dogfooded,
  accepted, and promoted states.
- Re-run stale proof after relevant code, dependency, platform, or contract
  changes.
- Never promote a narrower check into a broader claim.
- For a performance claim, compare the same representative workload and
  environment against a recorded baseline, measure the bottleneck before
  changing the design, retain a correctness oracle, and report a representative
  distribution or bound rather than only a favorable sample.
- Preserve important negative results; they constrain the next valid design.
