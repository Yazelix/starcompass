---
id: AP-REFERENCE-001
version: 2
title: Evidence before code shape
---

Review the relevant sources before deciding architecture or implementation
shape. Memory, summaries, and reputation are discovery aids, not sufficient
evidence for a consequential decision.

Required practice:

- Read the affected local code, contracts, tests, and repository instructions.
- Inspect designated external references at the subsystem named by local rules.
- Record the concrete mechanism adopted, rejected, or left unresolved.
- Distinguish direct source evidence from inference.
- Revisit the evidence when the proposed shape changes materially.
- Apply source-license wording to the actors, uses, and conditions it actually
  names. Do not infer that an independent user or project acts on behalf of,
  for the benefit of, or under the direction of an agent or tool provider
  merely because the user selected that provider's service. Examples
  introduced by words such as “including” remain scoped by the condition they
  illustrate.
- Distinguish inspecting public source for ideas from copying, adapting,
  redistributing, selecting a dependency, or incorporating the source. A
  restriction on one of those actions does not silently erase required source
  inspection when the requested research itself remains permitted.
- If license interpretation would exclude required evidence, identify the
  exact clause, actor, beneficiary, direction, and requested use. Resolve a
  material ambiguity with the user instead of broadening the restriction by
  association or substituting reputation and secondary summaries for source.

Reference review is a decision gate, not a requirement to copy the reference.
