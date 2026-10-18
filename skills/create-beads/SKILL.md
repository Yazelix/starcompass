---
name: create-beads
description: Turn work the user has chosen into right-sized Beads issues, one bead per fresh agent session, with bigger work split into an epic of child beads, dependencies for order, and self-contained bodies. Use when the user asks to create beads, plan or break down a feature as beads, turn a plan into issues, or file a follow-up. Other bead-writing skills follow its Bead Size rule and body template.
---

# Create Beads

## Overview

Work in plan space. Turn a goal the user has already chosen into beads that a fresh agent can each prime, finish, and verify in one session. Do not implement anything while using this skill.

## Tracker

- Use `br` (beads_rust) when it is available. Use `bd` only when the repository clearly uses it.
- Mutate planning only through the CLI. Never edit `.beads/` files directly, and serialize writes.
- Run `br sync --flush-only` afterwards when the repository commits Beads state.
- If no Beads CLI exists, report that and stop. Do not write Markdown plans or TODO files instead.

## Bead Size

Size each implementation bead to fit one fresh agent session with room to spare: one independently verifiable outcome, a bounded set of files, and one verification path. As a rule of thumb, an agent should be able to prime, finish, and verify it within about 100k tokens of context. Small beads close before context grows, which avoids mid-bead handoffs and compaction.

- When work is bigger than that, make an epic: the overall outcome and acceptance live on the parent, and each child bead (`<beads> create --parent <epic-id> ...`) delivers one verifiable slice. Add `blocks` dependencies between children when order matters.
- Each child stands on its own: its own description, acceptance, and verification, so an agent can finish it after reading only that bead and its parent.
- Do not split below a meaningful verified change. A child that cannot be checked on its own belongs with its neighbour.
- The epic closes when all its children close (`br epic close-eligible`).

## Workflow

1. Read the applicable `AGENTS.md` files and restate the outcome in one sentence. If a missing scope choice would change the plan, ask before creating anything.
2. Recon only as far as the beads need: the owning files and symbols, the repository's verification commands, and existing beads (`br search <terms>`, `br list --status open`) to avoid duplicates and find parents or blockers.
3. Choose the shape. One bead if the work fits the Bead Size rule; otherwise an epic plus children ordered by dependency, each child one verifiable slice.
4. Write each body with `references/bead-body-template.md`. Pass multi-line bodies with `--description-file`.
5. Create the epic first (`-t epic`), then the children with `--parent`, then the dependencies (`br dep add <blocked> <blocker>`).
6. Re-read the result (`br show`, `br epic status`, `br ready`) and check that the first child is ready and the graph matches the intended order.

## Rules

- Create only the beads for the work the user chose. Report other ideas instead of filing them.
- Record user decisions and rejected alternatives with their reasons in the bead they constrain.
- Name exact files, commands, and IDs. A bead must make sense without this conversation.

## Final Response

Report the created bead IDs with titles, the epic and its children, the recommended order and the first ready bead, and anything intentionally left unfiled.
