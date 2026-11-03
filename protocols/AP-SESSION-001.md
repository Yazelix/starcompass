---
id: AP-SESSION-001
version: 6
title: Session continuity across compaction
---

Compaction is lossy; keep it rare and checkpointed. Write each user decision
to its durable owner when it is made. Delegate work whose raw output matters
only as a conclusion, such as broad searches, long logs and checks, or
research, to a fresh subagent; subagents are authorized unless the user or
local rules say otherwise.

Budget absolute context tokens, not window share; if the host reports none,
estimate high:

- From 300k, start no new step: close the issue if it can close soon, else
  bring the current step to a verified state, checkpoint, and compact, or ask
  the operator to. Never pass 400k without a checkpoint.
- After a compaction, reread the instructions and the latest checkpoint, treat
  the summary as discovery only, and continue.
- After a session's fifth compaction, hand off from 300k instead of
  compacting.

A checkpoint or handoff is an issue comment: state, next step, open decisions
with reasons, and approaches ruled out. After a handoff, the next session
primes, e.g. "prime, then continue <issue> from its handoff".
