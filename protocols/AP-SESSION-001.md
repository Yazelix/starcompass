---
id: AP-SESSION-001
version: 5
title: Session continuity across compaction
---

Compaction is lossy; avoid it. Write each user decision to its durable owner
when it is made. Delegate work whose raw output matters only as a conclusion,
such as broad searches, long logs and checks, or research, to a fresh
subagent; subagents are authorized unless the user or local rules say
otherwise.

Budget absolute context tokens, not window share; if the host reports none,
estimate high:

- From 300k, add no scope or new steps: close the issue if it can
  close soon, else bring work in progress to a verified state and
  hand off.
- Never pass 400k: hand off before the host would compact. After any
  compaction, reread the issue and hand off.

A handoff is an issue comment: state, next step, and open decisions with
reasons. The next session primes, e.g. "prime, then continue <issue> from its
handoff".
