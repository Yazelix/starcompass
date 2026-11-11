---
id: AP-COORDINATOR-001
version: 2
title: One coordinator across agent sessions
---

When several agent sessions share a repository or machine, the user may name
one coordinator session. It orders landings on the shared branch, freezes the
branch during a final proof, schedules heavy runs and installs against machine
load, starts sessions where account quota lets work finish, and briefs and
retires them. Other sessions follow its schedule and report their state,
blocks, and pending user choices to it. No session types reports or briefs
into a terminal a person may use.

The coordinator may relay user decisions, approvals included. A relayed
decision binds only in the project it was given for and only when the message
quotes the user's words and names exactly what was approved. The receiver
records it as relayed, with the date. When a relay is ambiguous, the receiver
asks the user directly.

The user may grant the coordinator standing approval for a named class of
decisions with named exclusions; within it the coordinator approves, quoting
the grant on each issue, and reports afterwards. A grant never reverses a user
decision. Outside a grant, the coordinator never decides for the user. It never
answers a host permission prompt or asks a session to do what its own
permissions block.
