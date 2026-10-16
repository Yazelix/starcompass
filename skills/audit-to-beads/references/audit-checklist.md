# Audit Checklist

Use this checklist for broad `audit-to-beads` audits. A finding is only valid with concrete evidence.

## Correctness

- swallowed or downgraded errors
- missing failure states
- unawaited async work or stale lifecycle cleanup
- null/empty/boundary cases
- state combinations that should be unrepresentable
- retry/idempotency hazards

## Security

- committed credentials or secrets; never quote the value
- unvalidated input crossing into shell, SQL, HTML, filesystem, network, or dynamic execution boundaries
- missing server-side authorization
- request/body contracts that trust arbitrary input
- sensitive data in logs or client-visible errors
- high/critical dependency advisories affecting reachable paths

## Performance

- N+1 loops or repeated scans where indexed lookup belongs
- repeated expensive work with a clear cache boundary
- unbounded output or missing pagination
- heavy dependency for small behavior
- redundant CI/build/test work

## Tests

- risky/churned code without meaningful tests
- weak tests that assert command discovery, strings, or implementation trivia
- missing characterization tests before refactors
- no one-command verification baseline
- slow tests that could be narrower

## Tech Debt And Architecture

- duplicate implementations or drifted clones
- stale compatibility shims or migration paths
- private wrappers with zero live callers
- god modules and junk-drawer utilities
- abstraction layers with one implementation
- ownership boundaries that force lockstep edits

## Dependencies And Migrations

- abandoned or duplicate dependencies
- EOL runtimes/frameworks
- deprecated APIs with real removal timelines
- manifest/lock drift
- migrations whose blast radius needs sequencing

## DX And Tooling

- missing or broken formatter/lint/typecheck/test commands
- unclear local setup or required env vars
- slow feedback loops without documented faster gates
- poor error messages that force source inspection

## Docs

- docs that are actively wrong
- missing public API or behavior contract docs
- missing architecture decision for contested active areas

## Direction

- unfinished intent visible in docs, TODOs, flags, or half-built modules
- product promises without implementation
- cheap adjacent capabilities made possible by existing architecture
- maintainer friction visible in repeated manual workflows

For every finding, record: evidence, impact, effort, risk, confidence, and fix sketch.
