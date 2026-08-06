---
id: AP-MINIMAL-001
version: 3
title: Minimum sufficient implementation
---

Use the accepted contract and source evidence to identify the correct owner,
affected flow, and necessary boundaries before minimizing code. Then use the
first option that fully satisfies those constraints:

1. Make no change when the required behavior already exists.
2. Reuse an existing correct owner, helper, or pattern in the repository.
3. Use the standard library or a native platform capability.
4. Use an already accepted dependency that owns the behavior.
5. Implement the minimum local code that is correct and maintainable.

Evaluate minimality across the accepted system and its lifecycle, not only the
current patch. Lines, files, dependencies, and patch size are evidence, not
objectives. Include duplicated truth, cross-owner coordination, coupling,
migration and removal, portability, operations, and proof cost.

Durable agent context is part of that lifecycle cost. Put instructions at the
narrowest scope that needs them. Keep durable wording for non-obvious
constraints and reusable workflows that change behavior; do not duplicate
policy already owned elsewhere or restate generic agent capability. Load
detailed references only when the task needs them. Context reduction must not
hide requirements needed to act safely or correctly.

A smaller patch is not minimal when it preserves a known misplaced or duplicate
owner, patches a symptom below its shared cause, bypasses an accepted boundary,
or increases downstream coordination. A necessary root-cause or ownership
correction may be locally larger. Use patch size only as a tie-breaker among
shapes that satisfy the same contract, ownership, lifecycle, and proof duties.

Prefer deletion over addition, direct ownership over adapters, and fewer files
over scaffolding. Minimalism must not remove required behavior, trust-boundary
validation, data-loss protection, security, accessibility, or the cheapest
runnable check for non-trivial logic.

Ponytail is the adopted agent-side aid for this discipline when the host
supports it; it is a fallible implementation bias, not an architectural
authority. Apply its YAGNI, shortest-diff, standard-library, native, and
one-line heuristics only after the constraints above. Repository contracts,
ownership, evidence, dependency decisions, safety, accessibility, portability,
proof obligations, and explicit user choices take precedence. Do not invoke
Ponytail to avoid a necessary architectural or root-cause change.

Use the upstream project directly rather than copying its rules or adapters.
The reviewed source is
[DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail/tree/16f29800fd2681bdf24f3eb4ccffe38be3baec6b).
If Ponytail is unavailable or disabled, the self-contained requirements above
still apply. Its instruction hooks improve consistency; they do not prove
compliance.
