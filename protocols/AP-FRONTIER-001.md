---
id: AP-FRONTIER-001
version: 1
title: One active integration frontier
---

When several greenfield components are intended to work together, keep one
integration boundary under active architectural change whenever practical.

Required practice:

- Stabilize the contracts on the other sides with released artifacts, fixtures,
  adapters, or existing interchangeable projects.
- Dogfood the active frontier through the smallest real vertical slice.
- Move the frontier only after its contract and failure behavior are supported
  by evidence.
- Allow parallel internal work when it does not make multiple integration
  contracts speculative at once.

This is a risk-control default, not a ban on parallel implementation.
