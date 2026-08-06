---
id: AP-PROMOTION-001
version: 2
title: Exact-revision channel promotion
---

Release channels are ordered references over one linear history; the consumer
declares order, evidence, authority, and the development channel where every
change originates. Promotion-only channels fast-forward to the exact verified
revision in their predecessor. No direct channel commits, merges, cross-channel
cherry-picks, published rebases, force-pushes, or skipped stages.

Require target-channel delivery evidence, no known critical or high-severity
regression, and explicit authority for user-facing promotion. Roll back with a
development-channel revert, then verify and promote normally. Promotion changes
exposure, not proof.
