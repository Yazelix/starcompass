---
id: AP-PROOF-001
version: 5
title: Explicit proof lifecycle
---

Proof records revision, command/observation, environment, result, exercised
surface, and phase: proposed, implemented, mechanically verified, dogfooded,
accepted, or promoted. Never widen it; rerun after material change. Run a
complete suite once per materially unchanged candidate/environment. Docs,
contract metadata, and planning-only follow-ups reuse it: recorded source
commands are evidence, not a checklist; never rerun them.

Keep raw logs for costly reproduction, disputes, performance proof, or lossy
summaries; temporary paths are not evidence.

Performance claims require bottleneck measurement, fixed representative
workload/environment, baseline, correctness oracle, and distribution/bound.
Preserve constraining negatives.

Repeat review/simplification after changing its subject. Claim convergence
only when a full pass finds no actionable in-scope bugs or simplifications.
