---
id: AP-FORK-002
version: 1
title: Upstream and removal discipline
---

Every fork feature needs an upstream relationship and an exit story.

Required practice:

- Record the upstream baseline and patch provenance.
- Audit upstream changes before rebasing or replacing behavior; do not
  mechanically replay patches.
- Prefer contributing generally useful mechanisms upstream when feasible.
- Define when an upstream feature, external component, or contract change makes
  the local patch removable.
- Test both the intended fork behavior and the upstream boundary most likely to
  invalidate it.
