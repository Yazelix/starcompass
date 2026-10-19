---
id: AP-SURGICAL-001
version: 1
title: Surgical changes
---

Treat preserved code, an upstream or a codebase in maintenance, as owned
elsewhere: edits there cost merges and regressions. Prefer configuration and
extension points, then isolated patches, over rewrites. Keep changes in their
own modules, commits, and tests, so each can be dropped alone. Leave unrelated
preserved code alone; keep the delta small and measurable.

Where AP-MINIMAL-001 differs, this wins: fix a shared cause in preserved code
locally and report it, since a shared fix reaches callers you do not own, and
prefer a new file to edits across preserved files. Still delete orphans in code
you own and handle implied bad input.

With an upstream, record its baseline revision and each patch's reason and
removal condition, so patches drop once upstream fixes them, and audit upstream
changes instead of replaying patches mechanically.
