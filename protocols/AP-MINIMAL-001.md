---
id: AP-MINIMAL-001
version: 7
title: Minimum sufficient implementation
---

After identifying the contract, evidence, owner, flow, and boundaries, take the
first sufficient option: no change; existing owner, helper, or pattern;
standard library or native platform; accepted dependency that owns the
behavior; minimum correct local code. Build only for current callers, not
forecasts; abstract only for a second caller: speculation costs every reader.

Judge the whole lifecycle, including coordination, coupling, migration and
removal, portability, operations, proof, and agent context. Scope instructions
narrowly; retain non-obvious constraints and reusable behavior-changing
workflows, not generic or duplicated policy. Load details only when needed.

A patch is not minimal if it treats a symptom below its shared cause, bypasses
a boundary, or raises downstream cost. Leftovers are a change's commonest
defect: update every caller, test, fixture, config, and export it breaks and
delete what it orphans. Prefer fewer files; report unrelated dead code as out of
scope; use patch size only between equally correct system shapes. Handle empty,
malformed, and boundary input; real callers send it. Never remove required
behavior, trust-boundary validation, data-loss protection, security,
accessibility, or the cheapest runnable check for non-trivial logic.
