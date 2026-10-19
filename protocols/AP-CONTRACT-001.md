---
id: AP-CONTRACT-001
version: 5
title: Contract-driven changes
---

Before code shape, state the smallest observable contract: consumer, trigger,
result, and important failures. Of the contracts consistent with the request
and evidence, choose the fewest unsupported guarantees or restrictions; leave
reasonable future behavior unspecified. Give it a stable ID only when later
consumers need one. Choose the cheapest check that would falsify it.

Keep docs, help, examples, and configuration aligned with current commands,
paths, flags, defaults, and availability; label planned, partial, or gated
behavior. Update the contract first for an intentional behavior change.
Implementation details are not contracts unless a component must rely on them.
