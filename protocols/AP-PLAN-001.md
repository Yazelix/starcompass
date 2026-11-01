---
id: AP-PLAN-001
version: 14
title: Durable planning state
---

Keep later-needed outcomes/constraints in planning or canonical docs. Issues
hold chosen goals/decisions, material defects, or schedulable follow-ups;
methods stay with their owner. Acceptance names outcomes/cheapest falsifiers,
not command catalogs unless the boundary/transition needs them.

A run includes continuations until control returns; reads are unrestricted. A
writing run does exactly one:

1. Works issues in sequence, and may create/claim/update/implement/close
   each; finishes or hands off one before claiming the next. Independent
   issues (no dependency or shared files) instead run at once, each in its
   own isolated subagent and worktree with its own record, proof, review,
   and commit; land them one at a time. Their temporary branches are
   authorized; remove each worktree and branch after landing.
2. With explicit user authorization, changes a named/accepted planning-only
   batch; claims nothing and changes no implementation.

Bind each issue before its first write; return at completion, block, or
handoff. Report unapproved findings; create issues only for material
out-of-scope or schedulable work, deferred outside authorized batches.

Keep fields current; append only needed contracts, decisions, dependencies,
acceptance, material failures/negatives, constraining rejections, and approvals.
Baseline/reference comments record only changed identities, dirty state,
mechanism/stop decisions, and first check. Keep status/prerequisites honest; use
the issue tool (beads_rust `br` when available), never its storage.
