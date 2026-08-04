---
id: AP-TERMINAL-001
version: 1
title: Terminal state authority
---

One component must be authoritative for terminal semantic state. Renderers,
transports, and clients consume versioned projections; they must not invent a
second emulator state machine.

Required practice:

- Define ownership of parsing, grid state, modes, scrollback, selection,
  graphics, title, and process attachment.
- Specify whether clients receive raw bytes, checkpoints plus tails, structured
  frames, or another explicit replication contract.
- Treat reconnect, resize, alternate-screen state, partial parser input, and
  slow consumers as contract cases.
- Test semantic equivalence at attachment boundaries, not only visual similarity.
