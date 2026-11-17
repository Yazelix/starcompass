---
id: AP-SESSION-001
version: 7
title: Session continuity across compaction
---

Write each user decision to its durable owner when it is made. Delegate work
whose raw output matters only as a conclusion, such as broad searches, long
logs and checks, or research, to a fresh subagent; subagents are authorized
unless the user or local rules say otherwise.

Long-lived lead and desk sessions keep one "Standing decisions" block in their
state folder's `standing-decisions.md`: every standing owner instruction,
including decision modes, approvals, "don't ask about X", and conventions such
as "continue means go". Record each immediately with its date and the owner's
words; remove revoked entries. Update this block and in-flight issue state
when they change, not at a context threshold. Builders are outside this
long-lived-session regime.

Only the session's own launch sets `STARCOMPASS_STATE_DIR` to its absolute state
folder before starting the host; independent leads set their own value, and
builder/reviewer launchers clear it. The compact hook replays a bounded excerpt;
read the full file after compaction even when replay is unavailable or truncated.
Reread the instructions and latest checkpoint too; summaries are discovery only.

A symptom is the owner correcting a session for forgetting or contradicting a
recorded decision, or the session repeating a state-changing action it already
did. At the first symptom, reread the block and latest checkpoint. At the second
within the same compaction segment, hand off regardless of the compaction count.
After the fifth compaction, hand off instead of compacting again.

A checkpoint or handoff is an issue comment: state, next step, open decisions
with reasons, and approaches ruled out. Every handoff copies the standing
decisions block in full when present and lists background loops and monitors to
restart. After a handoff, the next session primes, e.g. "prime, then continue
<issue> from its handoff".
