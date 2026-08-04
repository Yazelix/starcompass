---
id: AP-OWNERSHIP-001
version: 1
title: One owner per invariant
---

Every invariant, state transition, and user-visible policy must have one clear
owner. Other components may consume its output; they must not independently
reconstruct or reinterpret the same truth.

Required practice:

- Name the owner before adding adapters or synchronization.
- Prefer deleting duplicate owners over reconciling them.
- Keep policy at the highest layer that has the necessary context and mechanism
  at the lowest layer that can enforce it correctly.
- Make cross-boundary data explicit and versioned when independently released
  components depend on it.
