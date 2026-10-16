---
name: list-beads
description: List the current project's top 10 ready beads issues (br, beads_rust) — open, unblocked and not deferred — by priority, plus the total ready count. Use when the user says list beads, show beads, what beads are open or ready, or show issues.
allowed-tools: Bash(br ready:*)
---

# List Beads

List this project's 10 highest-priority ready beads issues with `br` (beads_rust): open, with no open blockers, and not deferred. Not `bd`: these repos use `br`, and `bd` reports "no beads database found" in them. Read-only: never create, update, claim, or close issues here.

## Steps

1. Run `br ready --no-auto-flush --no-color --sort priority --limit 10` from the project root. `--sort priority` orders by priority, then age; the default hybrid sort orders only P0/P1 by priority. Use the plain output, not `--json`: JSON carries every full description and runs about 40 times larger.
2. If br finds no `.beads/` here, say this project has no beads and stop. Do not run `br init`.
3. Present the issues as a compact table, highest priority first: #, ID, priority (P0–P4), type, title. The `#` column numbers the rows in display order; it exists only in this printed table and is not a Beads field, so never pass it to `br`.
4. End with one line giving the ready total. Take it from br's trailing note (`Showing 10 of N ready issues`) or, when no note appears, from its header (`N issues with no blockers`); for example `top 10 of 21 ready shown`.

Show only the top 10; the user can ask for the full list or for deferred and blocked issues. Do not start working on any issue unless the user asks.
