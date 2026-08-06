---
id: AP-FAILURE-001
version: 3
title: Designed lifecycle and failure semantics
---

Long-lived sessions and services define start, attach/detach, crash/restart,
shutdown, version mismatch, abandonment; bound queues, memory, retries,
timeouts, slow consumers. Own cancellation and idempotent cleanup; persist only
required recovery state. Test deterministic failures when practical; dogfood
other process/platform cases.
