---
name: correct
description: Mine the owner's past Claude Code sessions in this repository for corrections they keep repeating, group them into mistake classes, and file one bead per class proposing the highest-level fix that would enforce it, preferring structure and checks over more instruction text. Use when the user says correct, asks why agents keep making the same mistakes, wants repeated corrections turned into rules or checks, or wants to learn from session history.
---

# Correct

Find the mistakes the owner keeps correcting and propose, for each, the fix that makes the correction unnecessary. This skill proposes; it edits nothing, not even a one-line fix (AP-SCOPE-001). Each proposal becomes a bead the owner can choose.

## Workflow

1. **Collect.** Run this skill's `scripts/owner_messages.py` from the repository root. It prints the start of every message the owner typed in this project's Claude Code sessions, oldest first, each with the tail of the agent reply it answered. Pass `--since YYYY-MM-DD` to narrow it. Measure it first with `| wc -c`; over about 60 KB, have a fresh subagent run it and do steps 2 and 3, and keep only its grouped classes (AP-SESSION-001).
2. **Pick out corrections.** A correction rejects, reverses, or repeats something the agent did or said: "no", "don't", "I said", "again", "stop", a restated instruction, or an undo. New requests, answers to questions, and approvals are not corrections. Read the full transcript around a message (`~/.claude/projects/<slug>/<session prefix>*.jsonl`) only when the excerpt cannot tell you.
3. **Group.** Merge corrections with the same underlying mistake into one class. Keep a class only when it occurs in at least two sessions, or when the owner says it is repeated. Drop one-offs.
4. **Find the current owner.** For each class, find what already governs it: a canonical protocol (cite its ID), the repository's local overlay, a skill, a checker, a test, or nothing. Say why that owner did not prevent the mistake: missing, ambiguous, unloaded, or ignored.
5. **Choose the highest working enforcement.** Prefer the first that would have prevented every occurrence:
   1. structure that makes the mistake impossible: a script, a generated file, a removed option;
   2. a mechanical check: the checker, a hook, lint, types, or a test;
   3. a behavior trial that proves a wording fix works (AP-TEST-001);
   4. sharper wording in the existing owner, last; new instruction text costs every session (AP-MINIMAL-001).
6. **File.** Search existing beads for duplicates first. File one bead per class with the `create-beads` body template (`../create-beads/references/bead-body-template.md`), typed `decision` when the owner must choose the enforcement. Each bead cites the evidence as session ID, date, and a short quote, names the current owner and why it failed, gives the proposed enforcement and rejected options, and says how to prove the fix.
7. **Report.** Show a table with each class, its occurrence count, its current owner, the proposed enforcement, and the bead ID. Then list the corrections you dropped as one-offs.

## Rules

- Quote only the few words a bead needs. Transcripts are the owner's local data, so never copy them anywhere else.
- A class whose fix would change a canonical protocol belongs in the protocol's source repository. Outside that repository, report the protocol ID and the proposed change instead of filing it; file it there only when the owner asks.
