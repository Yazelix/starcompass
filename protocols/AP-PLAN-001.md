---
id: AP-PLAN-001
version: 10
title: Durable planning state
---

Keep later-needed outcomes/constraints in planning or canonical docs. Issues
hold chosen goals/decisions, material defects, or schedulable follow-ups;
methods stay in owning issue.

A run includes automatic continuations until control returns; reads are
unrestricted. A writing run either:

1. Owns one issue at most; may create, claim, update, implement, or close it.
2. With explicit user authorization, creates/updates/closes a named/accepted
   planning-only batch; claims nothing and changes no implementation.

Never combine them.

Bind implementation to its issue before planning/code writes; return at
completion, blocking, or handoff. Report unapproved findings; create separate
issues only for material out-of-scope or independently schedulable work; defer
them outside such batches.

Keep fields current; append only evidence. Preserve contracts,
decisions, dependencies, acceptance, material negatives/failures,
constraining rejections, and approvals. Execution-baseline and reference-gate
comments must not repeat issue text; record only changing identities, dirty
state, adopted/rejected mechanisms, stop-condition decisions, and first
check. Before handoff, keep status honest, model real prerequisites, reconcile,
and use the issue tool; never edit storage.
