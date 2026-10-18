---
id: AP-SESSION-001
version: 2
title: Session continuity across compaction
---

Compaction is lossy; avoid it. Write each user decision to its durable owner
when it is made. Budget absolute context tokens, not window share; if the host
reports none, estimate high:

- From 100k, delegate exploration, logs, and long checks to fresh subagents;
  avoid broad reads.
- From 180k, add no scope or new steps: close the issue if it can close by
  250k, else bring work in progress to a verified state and hand off.
- Past 250k, continue only to reach an imminent verified state or keep live
  state that cannot be written down; record why on the issue.
- At 400k or after any compaction, reread the issue and hand off.

A handoff is an issue comment: state, next step, and open decisions with
reasons. The next session primes, e.g. "prime, then continue <issue> from its
handoff". Start each issue fresh unless the session is uncompacted and under
100k.
