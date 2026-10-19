---
id: AP-FORK-001
version: 3
status: retired
title: Minimal isolated fork delta
---

A fork owns only behavior that cannot reasonably live upstream or externally.
Start from exact upstream source and behavior; prefer configuration, extension
points, then isolated patches over rewrites. Keep fork modules, commits, and
tests separable; avoid unrelated upstream cleanup. Keep the delta cohesive and
measurable and justify growth with user-visible ownership.
