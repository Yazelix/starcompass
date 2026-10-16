# Pathology Catalog

Use this catalog to find simplification opportunities common in agent-written code. Do not treat a pattern as removable until call sites and behavior are understood.

## High-Value Agent Slop

- **Wrapper ladders**: functions/classes that only rename, forward, or lightly reshape arguments through several layers.
- **Duplicate implementations**: `new`, `better`, `v2`, `optimized`, or temporary files that coexist with the intended path.
- **Parametric clones**: copy-pasted functions differing only in field names, constants, labels, or small predicates.
- **Semantic clones**: different code shapes implementing the same behavior, especially validation, normalization, serialization, retry, and UI state transforms.
- **Defensive branches for impossible states**: branches that cannot occur after upstream parsing, type narrowing, schema validation, or framework guarantees.
- **Option bags with one caller**: broad config objects, optional flags, and callbacks added for imagined reuse.
- **Stale compatibility shims**: aliases, fallback paths, migrations, or deprecated adapters no supported caller uses.
- **Overgrown error handling**: nested try/catch or Result mapping that preserves no meaningful error semantics.
- **Comment fossils**: comments describing task plans, previous implementation attempts, or obvious mechanics.
- **Test scaffolding bloat**: helpers and fixtures more complex than the behavior under test.
- **Async/lifecycle clutter**: duplicated cancellation, cleanup, hook identity, effect dependency, subscription, or resource lifecycle code.
- **Serialization drift**: repeated manual encode/decode/mapping logic that should share one contract.

## Deletion Candidates

Deletion is a first-class tactic in ruthless mode. Require evidence from:

- direct identifier search (`rg`, language server, compiler, import graph, route registry, CLI command registry)
- exported API and package entrypoints
- runtime registration surfaces, reflection, decorators, config, migrations, templates, and dynamic imports
- tests, docs, examples, CI scripts, and generated indexes
- security and operational surfaces such as auth, audit logs, metrics, rate limits, and migrations

## False Positives

Be careful with code that looks unused but is reached through:

- reflection, plugin loading, dependency injection, decorators, conventions, or filenames
- framework magic such as routes, migrations, hooks, fixtures, serializers, commands, or build config
- external package exports, public APIs, CLI commands, schema fields, feature flags, or environment-driven paths
- security checks and audit trails that intentionally look redundant
- tests that preserve historical bugs or wire compatibility

## Preferred End State

Prefer fewer concepts, not just fewer lines. The best simplification deletes a whole accidental category: duplicate paths, stale options, wrapper layers, bespoke conversions, or impossible states.
