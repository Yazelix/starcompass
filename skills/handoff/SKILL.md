---
name: handoff
description: End a session cleanly so a fresh agent can continue the work. Stop starting new steps, bring work in progress to a verified state, and write a self-contained handoff on the owning Beads issue (state, next step, open decisions with reasons), then give the exact line that starts the next session. Use when the user says "handoff", "wrap up", or "hand off to the next agent", when context use crosses the repository's handoff mark, or right after a compaction mid-issue.
---

# Handoff

## Overview

A handoff trades this session's context for a clean one. The next agent starts from the repository, the owning bead, and the handoff comment only, so everything it needs must be written down. Chat history does not survive.

## Workflow

1. **Stop expanding.** Start no new step, subtask, refactor, or investigation. Finish only the step already in progress, and only if it can reach a verified state quickly; otherwise leave it and describe exactly where it stopped.
2. **Verify what you claim.** Run the cheapest check that proves each finished step. Record unverified work as unverified.
3. **Leave a known tree.** Follow the repository's commit policy. If you commit, use a focused message. If you leave changes uncommitted, list the dirty files in the handoff. Never leave work that only exists in this session's memory.
4. **Find the owning bead.** Use the repository's Beads CLI (`br` when available). If the work has no bead, propose creating one; if the user declines or the repository does not use Beads, give the handoff in your reply instead.
5. **Split if the rest is too big.** If the remaining work will not fit comfortably in one fresh session, turn the bead into an epic of child beads under the Bead Size rule in the `create-beads` skill (`../create-beads/SKILL.md`), and point the handoff at the first ready child.
6. **Write the handoff comment** on the owning bead (`br comments add <id> -f <file>` for multi-line text), using the template below.
7. **Flush and report.** Run `br sync --flush-only` if the repository commits Beads state, then end your reply with the restart line:

   > Start a fresh session with: prime, then continue `<id>` from its handoff.

## Handoff template

```text
Handoff (<reason: context mark, compaction, or user request>).
State: <revision and dirty files>; <steps done and the check that verified each>; <work in progress and exactly where it stopped>.
Next step: <the first concrete action, with file or command>.
Open decisions: <each question, the options, your recommendation, and why>.
Do not retry: <approaches tried or rejected, and why>. (Omit if none.)
```

## Rules

- Write for a reader with no access to this conversation: name files, commands, IDs, and revisions instead of "the issue we discussed".
- Keep the reasons. Summaries that keep what was done but drop why cause the next agent to retry rejected approaches.
- Record user decisions as decisions, not as open questions.
- Keep it short: the next agent should be able to act after reading the bead and this comment.
- Do not close the bead unless its acceptance criteria are met.
