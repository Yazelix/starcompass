---
id: AP-MINIMAL-001
version: 6
title: Minimum sufficient implementation
---

After identifying the contract, evidence, owner, flow, and boundaries, take the
first sufficient option: no change; existing owner, helper, or pattern;
standard library or native platform; accepted dependency that owns the
behavior; minimum correct local code.

Judge the whole lifecycle, including coordination, coupling, migration and
removal, portability, operations, proof, and agent context. Scope instructions
narrowly; retain non-obvious constraints and reusable behavior-changing
workflows, not generic or duplicated policy. Load details only when needed.

A patch is not minimal if it treats a symptom below its shared cause, bypasses
a boundary, or raises downstream cost. Prefer deletion and fewer files; use
patch size only between equally correct system shapes. Never remove required
behavior, trust-boundary validation, data-loss protection, security,
accessibility, or the cheapest runnable check for non-trivial logic.
