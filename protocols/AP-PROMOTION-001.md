---
id: AP-PROMOTION-001
version: 1
title: Exact-revision channel promotion
---

Release channels are ordered references over one linear history, not independent
development lines. The consumer declares the channel order, required evidence,
and promotion authority; this protocol does not prescribe branch names or
release cadence.

Required practice:

- Declare the channel ancestry invariant and the development channel where all
  tracked changes originate, including fixes, reverts, documentation, and
  planning state.
- Treat accepted and user-facing channels as promotion-only. Advance each by
  fast-forwarding it to the exact verified revision already present in its
  immediate predecessor.
- Do not create direct channel commits, merge commits, cross-channel
  cherry-picks, published-history rebases, force-pushes, or skipped promotion
  stages.
- Require the checks and delivery evidence defined for the target channel, and
  block a candidate with a known critical or high-severity regression.
- Require explicit user authority before advancing a user-facing channel.
- Roll back through a new revert on the development channel, then verify and
  promote that revision through the normal order.

Promotion changes exposure, not evidence. Moving a reference must never turn an
unproved revision into a proved one.
