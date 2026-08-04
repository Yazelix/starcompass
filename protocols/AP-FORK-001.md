---
id: AP-FORK-001
version: 1
title: Minimal isolated fork delta
---

A fork should own only the behavior that cannot reasonably live upstream or in
an external layer. Keep the patch surface small, cohesive, and easy to identify.

Required practice:

- Start from the exact upstream source and behavior under change.
- Prefer upstream configuration, extension points, and narrowly isolated patches
  before broad rewrites.
- Keep fork-specific modules, commits, and tests separable where the architecture
  permits it.
- Avoid opportunistic cleanup in upstream-owned code.
- Measure the maintained delta and justify growth with user-visible ownership.
