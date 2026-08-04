---
id: AP-MINIMAL-001
version: 1
title: Minimum sufficient implementation
---

Understand the affected flow before choosing the smallest complete solution.
Use the first option that fully satisfies the chosen contract:

1. Make no change when the required behavior already exists.
2. Reuse an existing owner, helper, or pattern in the repository.
3. Use the standard library or a native platform capability.
4. Use an already accepted dependency that owns the behavior.
5. Implement the minimum local code that is correct and maintainable.

Prefer deletion over addition, direct ownership over adapters, and fewer files
over scaffolding. Minimalism must not remove required behavior, trust-boundary
validation, data-loss protection, security, accessibility, or the cheapest
runnable check for non-trivial logic.

Ponytail is the adopted agent-side implementation of this discipline when the
host supports it. Use the upstream project directly rather than copying its
rules or adapters. The reviewed source is
[DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail/tree/16f29800fd2681bdf24f3eb4ccffe38be3baec6b).
If Ponytail is unavailable or disabled, the self-contained requirements above
still apply. Its instruction hooks improve consistency; they do not prove
compliance.
