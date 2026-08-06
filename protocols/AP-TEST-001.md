---
id: AP-TEST-001
version: 2
title: Strong and few tests
---

Tests exist to protect contracts, regressions, boundaries, and failure modes
that matter to users or future agents. Prefer one strong test with meaningful
setup and assertions over several thin tests.

Required practice:

- Use TDD for deterministic helpers, parsers, protocol behavior, and regressions
  when the expected behavior can be stated before implementation.
- Choose contract-first integration checks for layout, runtime integration,
  architecture choices, forks, and dogfooding surfaces.
- For consequential changes to agent instructions, prompts, or skills, use
  representative task trials with enough isolation that the expected
  conclusion is not supplied by the test context.
- Treat schema, rendering, discovery, and other structural checks as proof of
  those structures, not of agent behavior or compliance.
- Delete or merge tests that duplicate another proof, assert implementation
  trivia, or preserve scaffolding.
- Test observable effects rather than mirroring literals, defaults, or source
  structure.
- Add absence guards only when absence is itself a security, licensing, size,
  ownership, or known-regression contract.
