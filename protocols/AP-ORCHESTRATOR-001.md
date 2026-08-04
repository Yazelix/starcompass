---
id: AP-ORCHESTRATOR-001
version: 1
title: Thin orchestrator ownership
---

An orchestrator owns composition, lifecycle policy, compatibility selection,
and user-facing defaults. It must not absorb mechanisms or state already owned
by its child components.

Required practice:

- Consume released or pinned child artifacts through explicit contracts.
- Keep child-specific translation at a narrow boundary.
- Reject copies, hidden forks, and duplicate configuration schemas.
- Make startup, shutdown, update, and partial-failure policy observable.
- Validate the orchestrator independently with substitutes or fixtures when a
  child is not yet ready.
