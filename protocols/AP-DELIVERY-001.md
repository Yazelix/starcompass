---
id: AP-DELIVERY-001
version: 2
title: Installed artifact and fresh-session proof
---

Source checks do not prove delivery. For packaging, installation, activation,
or interaction changes, build the exact candidate through its delivery path;
record artifact, environment, commands, and result; install or activate in
isolation without checkout or stale-state fallback; verify exposed identity;
and dogfood interaction from a fresh session. Narrow claims for unexercised
platforms or surfaces. Compilation is not installation proof, and a script is
not manual dogfood.
