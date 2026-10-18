---
name: list-beads
description: List the current project's in-progress beads issues (br, beads_rust) and its top 10 ready ones — open, unblocked and not deferred — by priority, plus the total ready count. Use when the user says list beads, show beads, what beads are open or ready, or show issues.
allowed-tools: Bash(br ready:*), Bash(br list:*)
---

# List Beads

List this project's in-progress beads issues and its 10 highest-priority ready ones with `br` (beads_rust). Ready means open, with no open blockers, and not deferred; claimed work is never ready, so it needs its own query. Not `bd`: these repos use `br`, and `bd` reports "no beads database found" in them. Read-only: never create, update, claim, or close issues here.

## Steps

1. Run `br list --status in_progress --no-auto-flush --no-color --sort priority` from the project root.
2. Run `br ready --no-auto-flush --no-color --sort priority --limit 10`. `--sort priority` orders by priority, then age; the default hybrid sort orders only P0/P1 by priority. Use the plain output for both commands, not `--json`: JSON carries every full description and runs about 40 times larger.
3. If br finds no `.beads/` here, say this project has no beads and stop. Do not run `br init`.
4. Present the in-progress issues first as a compact table (ID, priority, type, title), or one line saying none are in progress. Then present the ready issues as a compact table, highest priority first: #, ID, priority (P0–P4), type, title. The `#` column numbers the rows in display order; it exists only in this printed table and is not a Beads field, so never pass it to `br`.
5. End with one line giving the ready total. Take it from br's trailing note (`Showing 10 of N ready issues`) or, when no note appears, from its header (`N issues with no blockers`); for example `top 10 of 21 ready shown`.

Show only the top 10 ready issues; the user can ask for the full list or for deferred and blocked issues. The order is priority, then age. It is not a recommendation: a bead on the critical path can rank below unrelated beads of the same priority unless its priority or dependencies say otherwise. If the user asks what to do next, say this list cannot answer that, and offer to read the in-progress beads' comments and the project's planning entry point instead of picking from the list. Do not start working on any issue unless the user asks.
