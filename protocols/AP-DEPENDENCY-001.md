---
id: AP-DEPENDENCY-001
version: 1
title: Dependency gate
---

Choose dependencies for architectural fit and net system simplicity, not name
recognition or short-term convenience.

Before adding a crate, package, framework, service, or embedded project:

- State the capability and contract it would own.
- Consider the standard library, owned code, and multiple credible candidates.
- Compare maintenance, platform fit, correctness, transitive weight, licensing,
  API stability, and the lines and complexity removed.
- Record the chosen candidate, meaningful rejections, and replacement cost.
- Pin deliberately and add the smallest check that proves the relied-on behavior.

Remove a dependency when it no longer owns enough behavior to justify its cost.
