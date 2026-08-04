---
id: AP-DELIVERY-001
version: 1
title: Installed artifact and fresh-session proof
---

Source-tree checks do not prove the artifact or runtime experience delivered to
a user. When the changed surface has a package, installation, activation, or
interactive-runtime contract, verify that surface before calling the revision
releasable.

Required practice:

- Build through the intended delivery path from the exact candidate revision.
- Record the artifact identity, relevant environment, commands, and result.
- Install or activate the artifact in an isolated destination without silently
  falling back to the development checkout or stale generated state.
- Verify that the installed program or package identifies the intended
  candidate when the product exposes an identity surface.
- Dogfood user-visible interaction changes from a newly started session; an
  already-running session does not prove startup, packaging, or activation.
- Keep claims narrower when a platform, installer, or interaction cannot be
  exercised. Missing proof does not become implied support.

Automate deterministic delivery checks where useful, but do not describe
compilation alone as installation proof or a scripted probe as manual dogfood.
