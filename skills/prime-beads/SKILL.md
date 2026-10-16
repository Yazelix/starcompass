---
name: prime-beads
description: Audit and refine Beads planning before implementation by rereading AGENTS.md, reviewing every relevant bead carefully, checking whether each bead and dependency makes sense, and revising Beads only through the project's Beads CLI (`bd` or `br`). Use when the user asks to review, optimize, restructure, improve, or sanity-check beads, issues, dependencies, acceptance criteria, scope, or plan-space before coding. Prefer `br` when the project does not clearly specify `bd`.
---

# Prime Beads

## Overview

Operate in plan space before implementation. Reread the repo rules, inspect the bead set with care, and use the selected Beads CLI as the only mutation surface for issue content and dependencies.

## CLI Selection

Use the Beads CLI that belongs to the project:

- Prefer explicit repo instructions in `AGENTS.md`, README, or Beads-generated guidance.
- If the repo clearly uses `bd`, use `bd`.
- If the repo clearly uses `br`, use `br`.
- If both are available or the project is ambiguous, use `br`.
- Keep the selected command in a shell variable mentally, such as `<beads>`, and do not mix CLIs in one pass unless a repo instruction requires it.

## Non-Negotiables

- Reread every applicable `AGENTS.md` before judging or changing beads so the workflow rules are fresh.
- Use only the selected Beads CLI commands to create, update, close, or link beads. Do not edit `.beads/` files directly.
- Do not use ad-hoc markdown TODOs, separate memory files, or issue comments as substitutes for Beads planning.
- Serialize Beads CLI commands unless the repository explicitly supports concurrent Beads access.
- Do not oversimplify by deleting necessary scope, acceptance criteria, edge cases, user-visible behavior, or features.
- Do not lose functionality. If simplifying a bead, preserve the real user outcome and make removed scope explicit.

## Workflow

1. Read applicable `AGENTS.md` files completely.
2. Select the Beads CLI for this repository using the CLI Selection rules.
3. Run the repository's preferred Beads context commands:
   - For `br`: `br list --status open --limit 0 --json`, `br ready --limit 50 --json`, `br status` or `br info`, and targeted `br show <id>` calls.
   - For `bd`: `bd prime`, `bd ready`, and targeted `bd show <id>` calls.
4. Identify the bead set in scope:
   - all beads the user named
   - open beads relevant to the requested work
   - parent, child, blocker, and dependency beads needed to understand the plan
5. Review each bead systematically:
   - Does the title describe a real outcome?
   - Does the description explain the current problem clearly?
   - Are acceptance criteria concrete and verifiable?
   - Is the scope neither too broad nor artificially narrow?
   - Are dependencies directionally correct and complete?
   - Are priority, type, labels, and status appropriate?
   - Are hidden migration, compatibility, docs, tests, or validation needs captured?
   - Is there any lost feature, user workflow, or edge case?
6. Improve the plan while it is cheap:
   - split beads that bundle unrelated work
   - merge or close duplicates
   - clarify acceptance criteria and verification paths
   - add missing blockers or parent-child relationships
   - record important design decisions or rejected alternatives
   - adjust priority only when the evidence supports it
7. Apply changes with the selected Beads CLI only:
   - `<beads> update <id> ...` for text, status, priority, type, or labels
   - `<beads> create ...` for missing work
   - `<beads> close <id> ...` for duplicates or obsolete beads, using the CLI's supported reason/comment flags when available
   - `<beads> dep add ...` or the repo's documented dependency command for relationships
8. Re-read the changed beads with `<beads> show` or equivalent and verify the graph still makes sense.
9. Summarize what changed and what remains intentionally unchanged.

## Quality Bar

A good bead plan makes implementation easier without deciding implementation prematurely. It should clarify user value, sequencing, dependencies, verification, and risk.

Favor plan quality over task count. However, do not collapse distinct features, failure modes, migrations, or validation requirements just to make the graph look smaller.

Prefer source-of-truth planning in Beads over long chat memory. If a decision will matter later, put it in the bead or linked contract using the selected Beads CLI.

## Final Response

Report:

- which beads were inspected
- which beads were revised, created, closed, or linked
- why the revisions improve the plan
- what risks, open questions, or implementation constraints remain
- any changes intentionally not made to avoid losing functionality
