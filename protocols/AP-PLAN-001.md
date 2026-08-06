---
id: AP-PLAN-001
version: 8
title: Durable planning state
---

Keep later-needed outcomes and constraints in the designated planning system or
canonical docs. Issues represent chosen goals, decisions, material defects, or
schedulable follow-ups; methods stay in their owning issue.

A run ends when control returns, including after automatic continuations.
Read-only planning inspection is unrestricted. Planning writes or implementation
use one shape:

1. Own at most one issue; create, claim, update, implement, or close only it.
2. With explicit user authorization, create, update, or close a named or
   accepted planning-only batch; claim nothing and edit no implementation.

Do not combine these shapes in one run.

Bind implementation work to its issue before planning or code writes. When that
issue completes, blocks, or hands off, return without starting another. Report
unapproved findings; create separate issues only for material out-of-scope or
independently schedulable work, and outside an authorized batch defer creation.

Keep editable fields at accepted current state after review, simplification, or
verification; reserve append-only history for needed evidence. Preserve
contracts, decisions, dependencies, acceptance evidence, material negative
results and failures, constraining rejections, and approvals. Keep raw logs with
their proof. Keep status honest, model only real prerequisites, and reconcile
before handoff. Use the designated issue tool; never edit its storage directly.
