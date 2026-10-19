---
name: apply-agent-protocols
description: Apply or update a pinned Starcompass profile in a repository while preserving its local rules. Use when the user asks to adopt, import, render, upgrade, or synchronize canonical agent protocols, AGENTS.md rules, protocol profiles, or recorded local exceptions.
---

# Apply Starcompass Protocols

Render canonical protocols into a consumer's `AGENTS.md`, self-contained
within the consumer. Keep the repository's own rules in a separate overlay and
pin every imported rule to an exact source commit.

## Preconditions

1. Read the consumer repository's complete `AGENTS.md` and inspect its worktree.
2. Locate the Starcompass checkout containing this skill. Read its complete
   `AGENTS.md`, `README.md`, selected profiles, and affected protocol files.
3. Run the pinned checkout's `starcompass validate` command. For current Rust
   revisions:

   ```sh
   cargo run --locked --quiet --manifest-path "$STARCOMPASS/Cargo.toml" -- \
     --root "$STARCOMPASS" validate
   ```

4. Require a clean source checkout and a full Git commit. Never render from
   uncommitted protocol, profile, or checker changes.

Do not select or combine profiles when the choice would change project scope.
Use the ordered profile set chosen by the user or the set already pinned. If
neither exists, explain the candidates and obtain the missing choice.

## Consumer Files

Maintain exactly these inputs and outputs:

- `.agent-protocols.local.md`: repository-owned rules not supplied by the profile.
- `.agent-protocols.exceptions.json`: explicit approved exceptions, including an
  empty schema-1 list when none exist.
- `.agent-protocols.json`: generated import manifest; do not edit manually.
- `AGENTS.md`: generated instructions; do not edit manually.
- The protocol file, when the manifest records `protocol_file`: generated
  canonical protocol text that `AGENTS.md` names; do not edit manually.

For first adoption, move every existing repository rule into the local overlay
without weakening, summarizing, or silently deduplicating it. Normalize only the
outer document heading needed for clean embedding. Review the resulting diff to
prove no local instruction disappeared.

An added local detail is an overlay. A rule that narrows, replaces, or suspends
an imported requirement is an exception and needs explicit approval. Record the
protocol ID, scope, reason, approver, approval date, and a review condition when
temporary. Never invent approval from repository history or chat implication.

## Render

From the clean Starcompass checkout, run:

```sh
cargo run --locked --quiet --manifest-path "$STARCOMPASS/Cargo.toml" -- \
  --root "$STARCOMPASS" render \
  --profile <profile> \
  --overlay <consumer>/.agent-protocols.local.md \
  --exceptions <consumer>/.agent-protocols.exceptions.json \
  --manifest-out <consumer>/.agent-protocols.json \
  --agents-out <consumer>/AGENTS.md
```

Repeat `--profile` in the chosen order when profiles compose, such as
`--profile surgical --profile release`. The renderer de-duplicates
protocol IDs while preserving the first profile's order.

When a subdirectory needs its own rules, add `--nested <dir>` for each one, and
keep that directory's rules in `<dir>/.agent-protocols.local.md`. Starcompass
renders `<dir>/AGENTS.md` from it. Never hand-write a directory `AGENTS.md`.

When the root instructions near Codex's 32 KiB chain limit, add
`--protocol-file <name>` to write the canonical protocol text to that consumer
file instead of into `AGENTS.md`.

Read the full rendered `AGENTS.md` and any protocol file. Verify that the
import record contains the expected source commit, ordered profiles, protocol
IDs and versions, and every exception.
Verify that its repository-local section preserves the overlay.

Then run:

```sh
cargo run --locked --quiet --manifest-path "$STARCOMPASS/Cargo.toml" -- \
  --root "$STARCOMPASS" check \
  --overlay <consumer>/.agent-protocols.local.md \
  --exceptions <consumer>/.agent-protocols.exceptions.json \
  --manifest <consumer>/.agent-protocols.json \
  --agents <consumer>/AGENTS.md
```

Run the consumer repository's own documentation and verification checks. Do not
commit or push unless the user requested the repository workflow that includes
those actions.

## Updating an Existing Import

First verify the current files with a Starcompass checkout at the manifest's
pinned commit. Inspect the protocol, version, profile, and checker diff between
that commit and the proposed source commit. Re-evaluate exceptions whose target
changed. Render only after the user has chosen any semantic profile or exception
change, then repeat the full check. Render reads `--nested` and
`--protocol-file` from flags, not from the manifest: pass every directory and
the protocol file the manifest records, or the render silently drops them.

Report the old and new source commits, protocol version changes, profile changes,
exception changes, preserved local overlay, and verification result.
