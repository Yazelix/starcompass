---
id: AP-CI-001
version: 1
title: Bounded continuous integration
---

Hosted automation must buy enough confidence to justify its financial,
latency, security, and maintenance cost.

Before enabling CI:

- Name the protected contract and why local verification is insufficient.
- Bound triggers, job count, timeouts, permissions, artifacts, cache growth, and
  concurrency.
- Prefer one cheap deterministic job before matrices or scheduled runs.
- Make fork and secret behavior explicit.
- Record the evidence required to expand, reduce, or remove the workflow.

Private-repository minutes and cache storage are product constraints, not an
invisible externality.
