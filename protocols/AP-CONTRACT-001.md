---
id: AP-CONTRACT-001
version: 3
title: Contract-driven changes
---

State the irreducible externally observable behavior before choosing the code
shape. Give durable contracts stable identifiers when later code, tests, or
repositories need to cite them.

Required practice:

- Among contracts that fit the user request and available evidence, choose the one
  with the fewest unsupported guarantees or restrictions. Leave reasonable
  future behavior unspecified unless the request or evidence requires a
  commitment.
- Treat user-facing documentation, help, examples, and configuration
  descriptions as contract surfaces. Keep their commands, paths, flags,
  defaults, and availability aligned with their current owner; label planned,
  partial, or gated behavior instead of presenting it as current.
- Name the consumer, trigger, observable result, and important failure behavior.
- Identify the current sources of truth and decide which one owner survives.
- Choose the cheapest check that can falsify the contract.
- Implement the smallest vertical slice that satisfies it.
- Update the contract first when an intentional behavior change is chosen.

Do not turn implementation details into contracts unless another component must
rely on them.
