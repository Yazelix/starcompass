---
id: AP-DELIVERY-001
version: 3
title: Installed artifact and fresh-session proof
---

Source checks do not prove delivery. For packaging/install/activation/
interaction changes, build exact candidate through delivery path;
record artifact/environment/commands/result; activate without checkout/stale
fallback; verify identity; dogfood from a fresh session. Narrow unexercised
claims. Compilation is not install proof; scripts are not dogfood.

Composition proof covers an unchanged embedded artifact only when recording
exact identity and exercising relied-on behavior. Repeat inner proof for
distinct install/activation or failure isolation.
