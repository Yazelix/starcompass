---
id: AP-SESSION-001
version: 1
title: Session continuity across compaction
---

Treat context compaction as lossy. Write each user decision to its durable
owner when it is made, and reread the active issue after any compaction. Once
a session has compacted, start the next issue in a fresh session; within an
issue, a second compaction or any sign of lost context triggers a handoff
comment (state, next step, open decisions) and a new session. Prefer a
clean-break restart to compaction, and start a large issue fresh when most of
the context is already used.
