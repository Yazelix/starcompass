---
name: audit-agent-protocols
description: Audit a repository's pinned Starcompass import for structural drift, missing metadata, ambiguous local exceptions, and direct rule conflicts. Use when the user asks to verify, check, review, or diagnose AGENTS.md protocol integrity or whether canonical agent rules remain correctly imported.
---

# Audit Starcompass Protocols

Audit without changing the consumer. Separate machine-provable import integrity
from semantic review and from claims about historical agent compliance.

## Establish the Pin

1. Read the consumer's full `AGENTS.md`, `.agent-protocols.json`,
   `.agent-protocols.local.md`, `.agent-protocols.exceptions.json`, and any
   protocol file the manifest records.
2. Inspect repository status and instructions. Do not clean or rewrite drift.
3. Locate a Starcompass checkout at the exact source commit recorded in the
   manifest. If it is unavailable, report structural verification as blocked;
   do not substitute the latest checkout.
4. Read the source repository's complete `AGENTS.md`, selected profiles, imported
   protocol files, and checker instructions.

## Mechanical Audit

Run the validation and check entrypoints documented by the pinned source
revision; historical pins retain their historical implementation. For current
Rust revisions:

```sh
cargo run --locked --quiet --manifest-path "$STARCOMPASS/Cargo.toml" -- \
  --root "$STARCOMPASS" validate
cargo run --locked --quiet --manifest-path "$STARCOMPASS/Cargo.toml" -- \
  --root "$STARCOMPASS" check \
  --overlay <consumer>/.agent-protocols.local.md \
  --exceptions <consumer>/.agent-protocols.exceptions.json \
  --manifest <consumer>/.agent-protocols.json \
  --agents <consumer>/AGENTS.md
```

This proves the canonical catalog and profiles are intact, the source commit and
protocol versions are pinned, the overlay and exception input match the
manifest, and `AGENTS.md` and any protocol file are the exact deterministic
rendering.

It does not prove that an agent followed the instructions. Never describe a
clean drift check as compliance evidence.

## Semantic Audit

After the mechanical check, inspect each exception:

- It targets an imported protocol and identifies a precise local scope.
- Its reason describes a genuine conflict, not convenience or undocumented
  policy.
- Its approver is the user or named project authority.
- Temporary exceptions have a concrete review condition.
- The local overlay does not contradict an imported protocol without an
  exception.
- Duplicate or obsolete exceptions are removed only after approval.

Also check that the selected profiles and their order match the repository roles
and that local rules remain self-contained. Treat changing the profile set or
exception policy as a user-owned scope decision.

## Report

Lead with one of: clean, structurally drifted, semantically ambiguous, or
verification blocked. List exact files and protocol IDs for findings. Separate
confirmed defects from judgment calls and state the narrowest repair. Do not
apply, commit, push, create issues, or update the source pin unless the user asks.
