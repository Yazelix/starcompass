---
id: AP-TEST-001
version: 4
title: Strong and few tests
---

Protect meaningful contracts, regressions, boundaries, and failures with a few
strong tests. Use TDD for deterministic helpers, parsers, protocol behavior,
and regressions whose expected behavior is known first; use contract-first
integration checks for layout, runtime, architecture, forks, and dogfood.

For consequential agent-instruction, prompt, or skill changes, run isolated
representative tasks without supplying the expected conclusion when the user
authorizes a separate agent; otherwise validate in the active context and
report agent behavior as unproven. Structural checks prove structure, not
agent behavior. Test observable effects; delete duplicate proof,
implementation trivia, and scaffolding. Guard absence only when absence is a
security, licensing, size, ownership, or known-regression contract.
