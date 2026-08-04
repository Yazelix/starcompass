# Starcompass

Starcompass is a canonical source for development rules shared across coding
agent projects without turning `AGENTS.md` into a remote dependency.

It provides:

- 21 individually identified and versioned protocols;
- five ordered, composable profiles;
- deterministic, self-contained `AGENTS.md` rendering;
- exact source-commit, protocol-version, overlay, and exception records;
- a typed Rust structural drift checker and renderer;
- two Codex skills dedicated only to applying and auditing the protocols; and
- self-adoption of the `greenfield` profile through the ordinary consumer
  format.

The repository is private while the format and broader consumer adoption are
incubating. It has no hosted CI. Local verification is the current proof surface.

Starcompass is the project and repository identity. Stable technical format
names remain unchanged: protocol IDs use `AP-*`, consumer files use
`.agent-protocols.*`, and the protocol skills retain their existing IDs.

## Model

Canonical protocol prose lives in `protocols/AP-*.md`. Profiles contain only
ordered protocol IDs. A consumer selects one or more profiles, owns a local
overlay and exception input, and renders both the canonical and local text into
one complete `AGENTS.md`.

The consumer therefore does not need this repository or network access while an
agent works. The source checkout is required only when importing, updating, or
checking the pin.

Profiles compose by ordered union: the first appearance of a protocol sets its
position and later duplicates are ignored.

Every profile includes `AP-MINIMAL-001`. Ponytail is its adopted agent-side
implementation where the host supports it. Consumers use the
[upstream project](https://github.com/DietrichGebert/ponytail/tree/16f29800fd2681bdf24f3eb4ccffe38be3baec6b)
directly; Starcompass does not copy its modes, hooks, skills, or host
adapters. The protocol remains self-contained so its invariant still applies
offline or when Ponytail is unavailable. The checker does not install Ponytail
or claim that its instructions were followed.

| Profile | Intended role | Specialized protocols |
| --- | --- | --- |
| `greenfield` | Small new projects | dependency, proof, planning, and bounded-CI gates |
| `fork` | Maintained downstream forks | minimal delta and upstream/removal discipline |
| `orchestrator` | Thin composition layers | orchestrator ownership, one active frontier, portability |
| `release` | Products with ordered release channels | installed-artifact proof, fresh-session dogfood, exact-revision promotion |
| `terminal-engine` | Terminal/session engines | terminal authority, lifecycle, failure, and portability |

A project may combine roles, for example `greenfield` plus `terminal-engine`
plus `release`. The release profile is opt-in: it does not create branches or
prescribe channel names, cadence, packaging commands, or platform support.
`AP-DELIVERY-001` proves the delivered artifact and fresh-session experience;
`AP-PROMOTION-001` advances that exact proved revision through the
consumer-declared channel order. Profile selection remains a product-scope
decision; the renderer does not infer it from repository contents.

## Consumer layout

| File | Owner | Purpose |
| --- | --- | --- |
| `.agent-protocols.local.md` | Consumer | Repository-specific instructions that add to the canonical rules |
| `.agent-protocols.exceptions.json` | Consumer | Explicit approved changes to imported rules, including an empty list when none exist |
| `.agent-protocols.json` | Renderer | Exact source, profile, protocol, exception, and overlay record |
| `AGENTS.md` | Renderer | Self-contained instructions read by coding agents |

Do not hand-edit the generated manifest or `AGENTS.md`. Change the local overlay
or exception input and render again. During first adoption, preserve every rule
from the existing `AGENTS.md` in the overlay before replacing the original with
generated output.

An empty exception input is:

```json
{
  "schema": 1,
  "exceptions": []
}
```

An exception has five required fields and one optional review condition:

```json
{
  "schema": 1,
  "exceptions": [
    {
      "protocol": "AP-CI-001",
      "scope": "private alpha",
      "reason": "Hosted jobs are disabled while cost is measured.",
      "approved_by": "user",
      "approved_at": "2026-08-04",
      "review_condition": "Review before public beta."
    }
  ]
}
```

An overlay adds local detail. An exception narrows, replaces, or suspends an
imported requirement and therefore requires explicit approval.

## Commands

Build a standalone local binary with `cargo build --locked --release`; the
result is `target/release/starcompass`. During incubation, Starcompass does not
publish release artifacts or an installer.

Validate the source catalog:

```sh
cargo run --locked --quiet -- validate
```

After changing protocol source or metadata, validate the version transition
before regenerating its indexed hashes:

```sh
cargo run --locked --quiet -- version-check --base HEAD
cargo run --locked --quiet -- index
cargo run --locked --quiet -- validate
```

From a clean, committed Starcompass checkout, render a consumer:

```sh
cargo run --locked --quiet -- render \
  --profile greenfield \
  --profile terminal-engine \
  --overlay /path/to/consumer/.agent-protocols.local.md \
  --exceptions /path/to/consumer/.agent-protocols.exceptions.json \
  --manifest-out /path/to/consumer/.agent-protocols.json \
  --agents-out /path/to/consumer/AGENTS.md
```

Check the consumer using a Starcompass checkout at the exact commit pinned
in its manifest:

```sh
cargo run --locked --quiet -- check \
  --overlay /path/to/consumer/.agent-protocols.local.md \
  --exceptions /path/to/consumer/.agent-protocols.exceptions.json \
  --manifest /path/to/consumer/.agent-protocols.json \
  --agents /path/to/consumer/AGENTS.md
```

`validate` and `index` can run during source development. `render` and `check`
refuse a dirty protocol, profile, or checker source because an uncommitted change
cannot be represented by the recorded Git commit.

## What the checker proves

The checker rejects:

- malformed, duplicate, unindexed, stale, orphaned, removed, or unversioned
  protocol changes;
- malformed profiles and unknown or duplicate profile members;
- malformed or out-of-profile exceptions;
- a source checkout different from the manifest's full Git SHA;
- protocol, profile, overlay, exception-input, manifest, or generated-file drift.

It does not prove that an agent followed the prose, that a repository's code
satisfies the rules, or that an exception was truthfully approved. The
`audit-agent-protocols` skill performs the human-semantic part of an import audit
and reports it separately from the mechanical result.

## Protocol lifecycle

Protocol IDs use `AP-<AREA>-<NUMBER>` and are permanent. Every change to a
canonical protocol file bumps its positive integer version because an automated
history gate cannot safely distinguish editorial from semantic wording. Run
`version-check` before `index`. Important rationale and rejected alternatives
live in Beads.

Consumers upgrade intentionally:

1. verify the old import at its pinned source commit;
2. inspect protocol, profile, and checker differences;
3. reconsider affected exceptions;
4. render from the chosen clean commit;
5. run the drift check and the consumer's own verification.

## Repository layout

```text
protocols/                 canonical Markdown and generated index
profiles/                  ordered JSON protocol sets
src/lib.rs                 index, render, and drift-check owner
src/main.rs                `starcompass` process boundary
tests/                     focused contract tests
skills/                    protocol-only Codex skills
docs/DESIGN.md             architecture decisions and boundaries
.agent-protocols.local.md  Starcompass-specific policy input
.agent-protocols.*.json    exact self-import and exception records
AGENTS.md                  generated self-contained active policy
.beads/                    durable planning state managed through br
```

Starcompass consumes its own `greenfield` profile through the same files and
renderer used by other repositories. Its manifest pins the latest accepted
protocol-bearing revision rather than the consumer revision that contains the
generated files. A same-commit pin is impossible because committing the import
would change the commit being named.

Canonical protocol, profile, or checker changes therefore use two revisions.
Commit and verify the source change under the existing pin, then render the
self-adoption from a clean detached checkout at that source revision and commit
the refreshed import separately. This compiler-style bootstrap needs no special
profile, checker mode, or exception.

## Verification

```sh
cargo fmt --check
cargo check --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
python3 /home/lucca/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/apply-agent-protocols
python3 /home/lucca/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/audit-agent-protocols
git diff --check
```

The self-import check must use a clean checkout at the source commit recorded in
`.agent-protocols.json`, not the newer consumer checkout:

```sh
PINNED_STARCOMPASS=/path/to/clean/checkout-at-the-manifest-source-commit
cargo run --locked --quiet --manifest-path "$PINNED_STARCOMPASS/Cargo.toml" -- \
  --root "$PINNED_STARCOMPASS" check \
  --overlay .agent-protocols.local.md \
  --exceptions .agent-protocols.exceptions.json \
  --manifest .agent-protocols.json \
  --agents AGENTS.md
```

## LOC scorecard

The scorecard counts tracked project text and code, excluding `.beads/` state.

| Surface | LOC |
| --- | ---: |
| Protocols and generated index | 575 |
| Profiles | 104 |
| Rust checker and package manifest | 1,141 |
| Tests | 377 |
| Enforcement skills | 175 |
| Repository policy and docs | 1,016 |
| **Total** | **3,388** |
