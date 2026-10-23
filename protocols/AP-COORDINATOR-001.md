---
id: AP-COORDINATOR-001
version: 1
title: One coordinator across agent sessions
---

When several agent sessions share a repository or machine, the user may name
one coordinator session. It orders landings on the shared branch, freezes the
branch during a final proof, schedules heavy runs and installs against machine
load, and starts, briefs, and retires sessions. Other sessions follow its
schedule and report their state, blocks, and pending user choices to it.

The coordinator may relay user decisions, approvals included. A relayed
decision binds only when the message quotes the user's words and names exactly
what was approved. The receiver records it as relayed, with the date. The
coordinator never decides for the user, never answers a host permission
prompt, and never asks a session to do what its own permissions block. When a
relay is ambiguous, the receiver asks the user directly.
