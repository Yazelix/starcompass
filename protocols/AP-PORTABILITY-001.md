---
id: AP-PORTABILITY-001
version: 1
title: Portable core with explicit platform seams
---

Do not let one operating system's APIs, process model, paths, packaging, or
event facilities become an accidental foundation when supported targets are
broader.

Required practice:

- Keep platform-neutral contracts and state in the core.
- Isolate OS-specific code behind the smallest meaningful seam.
- Evaluate Linux and macOS implications before adopting foundational runtime,
  process, graphics, filesystem, or transport dependencies.
- Prove platform behavior on the platform; compilation alone is narrower
  evidence.
- Record intentionally unsupported platforms rather than implying portability.
