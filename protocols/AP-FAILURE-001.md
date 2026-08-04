---
id: AP-FAILURE-001
version: 1
title: Designed lifecycle and failure semantics
---

Long-lived sessions and services must define lifecycle, recovery, and pressure
behavior before happy-path feature breadth.

Required practice:

- Name state transitions for start, attach, detach, crash, restart, shutdown,
  version mismatch, and abandoned resources.
- Bound queues, memory, retries, timeouts, and slow-consumer behavior.
- Make cancellation ownership and cleanup idempotent.
- Preserve enough durable state to meet the recovery contract and no more.
- Test failures deterministically where possible and dogfood the remaining
  process and platform interactions.
