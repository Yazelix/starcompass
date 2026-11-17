---
name: handoff
description: End a session cleanly with a self-contained Beads handoff, standing decisions, and background-loop restart inventory, then give the exact restart line. Also records checkpoints as in-flight state changes. Use when the user says "handoff", "wrap up", "checkpoint", or "hand off to the next agent", or AP-SESSION-001 calls for a handoff.
---

# Handoff

## Overview

A handoff trades this session's context for a clean one. The next agent starts from the repository, the owning bead, and the handoff comment only, so everything it needs must be written down. Chat history does not survive.

A checkpoint records in-flight state when it changes, as `AP-SESSION-001` requires; it does not wait for a token threshold or require compaction. Use the template with "Checkpoint" in place of "Handoff", verify changed claims, update the owning bead, and continue without a restart line.

## Workflow

1. **Stop expanding.** Start no new step, subtask, refactor, or investigation. Finish only the step already in progress, and only if it can reach a verified state quickly; otherwise leave it and describe exactly where it stopped.
2. **Verify what you claim.** Run the cheapest check that proves each finished step. Record unverified work as unverified.
3. **Leave a known tree.** Follow the repository's commit policy. If you commit, use a focused message. If you leave changes uncommitted, list the dirty files in the handoff. Never leave work that only exists in this session's memory.
4. **Find the owning bead.** Use the repository's Beads CLI (`br` when available). If the work has no bead, propose creating one; if the user declines or the repository does not use Beads, give the handoff in your reply instead.
5. **Split if the rest is too big.** If the remaining work will not fit comfortably in one fresh session, turn the bead into an epic of child beads under the Bead Size rule in the `create-beads` skill (`../create-beads/SKILL.md`), and point the handoff at the first ready child.
6. **Write the handoff comment** on the owning bead (`br comments add <id> -f <file>` for multi-line text), using the template below. Copy the current standing-decisions block in full from the session's `standing-decisions.md` when present; a summary or link loses instructions. List background loops and monitors, whether each survives or stopped, and its restart command and target. Verify surviving processes before restarting so the successor does not duplicate them; replace stale terminal IDs with current targets. Write `none` when absent; builders do not need to create a decisions file.
7. **Flush and report.** Run `br sync --flush-only` if the repository commits Beads state, then end your reply with the restart line:

   > Start a fresh session with: prime, then continue `<id>` from its handoff.

## Handoff template

```text
Handoff (<reason: symptom, compaction cap, or user request>).
State: <revision and dirty files>; <steps done and the check that verified each>; <work in progress and exactly where it stopped>.
Next step: <the first concrete action, with file or command>.
Open decisions: <each question, the options, your recommendation, and why>.
Do not retry: <approaches tried or rejected, and why>. (Omit if none.)
Standing decisions: <full current block, including dates and owner quotes; none if absent>.
Background loops and monitors: <status, restart command, and target for each; none if absent>.
```

## Rules

- Write for a reader with no access to this conversation: name files, commands, IDs, and revisions instead of "the issue we discussed".
- Keep the reasons. Summaries that keep what was done but drop why cause the next agent to retry rejected approaches.
- Record user decisions as decisions, not as open questions.
- Keep it short: the next agent should be able to act after reading the bead and this comment.
- Do not close the bead unless its acceptance criteria are met.
