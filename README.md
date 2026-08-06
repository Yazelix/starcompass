# Starcompass

Starcompass is a canonical source for development rules shared across coding
agent projects without turning `AGENTS.md` into a remote dependency.

It provides:

- 21 individually identified and versioned protocols;
- five ordered, composable profiles;
- deterministic, self-contained `AGENTS.md` rendering;
- exact source-commit, protocol-version, overlay, and exception records;
- typed Rust source-backed and source-independent drift checks plus rendering;
- one owning implementation Bead per run or an explicit bounded planning-only
  batch;
- two Codex skills dedicated only to applying and auditing the protocols; and
- self-adoption of the `greenfield` profile through the ordinary consumer
  format.

The source repository is public under either the MIT or Apache-2.0 license. It
has no hosted CI; local verification is the current proof surface.

Starcompass is the project and repository identity. `AP` means Agent Protocol,
the publisher-neutral namespace for IDs such as `AP-SCOPE-001`. Consumer files
use `.agent-protocols.*`, and the protocol skills retain their existing IDs.

## Model

Canonical protocol prose lives in `protocols/AP-*.md`. Profiles contain only
ordered protocol IDs. A consumer selects one or more profiles, owns a local
overlay and exception input, and renders both the canonical and local text into
one complete `AGENTS.md`.

The consumer therefore does not need this repository or network access while an
agent works or while its four local import files are checked for agreement. A
source checkout is required only when importing, updating, or authenticating the
pin and canonical protocol text.

Profiles compose by ordered union: the first appearance of a protocol sets its
position and later duplicates are ignored.

With empty exceptions and overlay, each shipped profile's canonical/import
section is capped at 12,288 bytes. The tested `greenfield + terminal-engine`
combination is capped at 13,030 bytes; with Orbit's measured 17,690-byte local
suffix, the complete instructions remain within 30,720 bytes. These budgets
leave room under Codex's default 32 KiB project-document loader limit without
remote includes or consumer-specific profile forks.

Protocol IDs remain reserved after retirement. Active protocols must appear in
at least one profile. A retired protocol stays in `protocols/` and the generated
index with its retained source and history, but no profile may reference it.
Protocol frontmatter omits `status` while active and uses `status: retired` for
the irreversible transition; retirement changes the canonical file and
therefore requires a version bump.
`AP-SCOPE-001` and `AP-EXCEPTION-001` remain mandatory in every profile while
active; retirement releases that membership requirement.

`AP-SCOPE-001` treats inspection, audit, diagnosis, explanation, and
recommendation as non-mutating requests unless the user also chooses a change.
It preserves user-owned inputs and artifacts unless replacing or deleting the
exact target is required by the requested outcome.

`AP-CONTRACT-001` selects the contract with the fewest unsupported guarantees or
restrictions that fits the user request and available evidence. It leaves
reasonable future behavior unspecified unless the request or evidence requires
a commitment. User-facing documentation, help, examples, and configuration
descriptions remain aligned with their current behavior and identify planned,
partial, or gated surfaces honestly.

`AP-PLAN-001` keeps an implementation run on one owning issue. An explicit user
request may authorize a bounded planning-only batch over a named or accepted
issue set; that run may create, update, or close those records but cannot claim
or implement work.

`AP-REFERENCE-001` requires direct evidence and exact license interpretation.
It distinguishes public-source inspection for ideas from copying, dependency
selection, adaptation, and redistribution, and it forbids treating an
independent user as acting for an agent provider merely because the user chose
that provider's tool.

Every profile includes `AP-MINIMAL-001`. Ponytail is its adopted agent-side
implementation where the host supports it. Consumers use the
[upstream project](https://github.com/DietrichGebert/ponytail/tree/16f29800fd2681bdf24f3eb4ccffe38be3baec6b)
directly; Starcompass does not copy its modes, hooks, skills, or host
adapters. Starcompass treats Ponytail as a fallible implementation bias, not an
architectural authority: accepted contracts, correct ownership, evidence, and
net system complexity determine the valid shape before patch size is minimized.
The protocol remains self-contained so its invariant still applies offline or
when Ponytail is unavailable. The checker does not install Ponytail or claim
that its instructions were followed. Durable agent context is included in
lifecycle cost: instructions use the narrowest useful scope, avoid duplicate or
generic wording, and load detailed references only when needed.

`AP-TEST-001` uses representative isolated task trials for consequential
changes to agent instructions, prompts, and skills. Structural validation of
their schema, rendering, or discovery does not prove agent behavior or
compliance. [`docs/BEHAVIOR_TRIALS.md`](docs/BEHAVIOR_TRIALS.md) provides six
host-independent cases. Operators give an agent only a case's input and fixture,
use a separate assessment rubric, and record the exercised revision and
environment without assigning a general compliance score.

Under `AP-PROOF-001`, agents treat review and simplification completion as
convergence over the accepted scope. A pass that makes a material change
invalidates the prior assessment; completion requires a full pass over the
revised state with no actionable in-scope bugs or simplifications.

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

`render` requires an exception input; use this document when none exist:

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
The renderer requires non-empty, single-line repository and profile values
without backticks because the import record uses inline code spans. It preserves
leading Markdown indentation in canonical protocol bodies and local overlays,
folds exception line endings into spaces, and adds review-condition punctuation
only when the author omitted it.

## Commands

Build a standalone local binary with `cargo build --locked --release`; the
result is `target/release/starcompass`. Starcompass distributes its source
through GitHub and does not publish release artifacts or an installer. Cargo
exposes no library target; `src/starcompass.rs` is a private module of the
executable.

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

Only `--profile` may be repeated. Other command options are singular; the CLI
rejects duplicates before reading or writing files.

Check the consumer using a Starcompass checkout at the exact commit pinned
in its manifest when the complete source-backed proof is required:

```sh
cargo run --locked --quiet -- check \
  --overlay /path/to/consumer/.agent-protocols.local.md \
  --exceptions /path/to/consumer/.agent-protocols.exceptions.json \
  --manifest /path/to/consumer/.agent-protocols.json \
  --agents /path/to/consumer/AGENTS.md
```

Check every relationship derivable from the four consumer files without a
Starcompass checkout, Git repository, catalog, credentials, or network:

```sh
starcompass check-consumer \
  --overlay /path/to/consumer/.agent-protocols.local.md \
  --exceptions /path/to/consumer/.agent-protocols.exceptions.json \
  --manifest /path/to/consumer/.agent-protocols.json \
  --agents /path/to/consumer/AGENTS.md
```

`check-consumer` validates manifest structure, overlay hashing, exception
parity, the generated import record, and the complete repository-local suffix.
It treats the canonical protocol section as opaque and reports that it did not
verify canonical prose or manifest protocol digests against source. It does not
accept `--root`.

`validate` and `index` can run during source development. `render` and `check`
refuse a dirty protocol, profile, or checker source because an uncommitted change
cannot be represented by the recorded Git commit.

Canonical protocol and profile directories must be real directories, while
their source files and `protocols/index.json` must be regular files. Symlinks
and special files are rejected because their bytes are not owned by the
recorded Git revision. Before writing, `render` resolves the overlay, exception
input, manifest, and `AGENTS.md` paths and rejects aliases that identify
the same location. Parent-directory traversal or a symlink cannot let an output
replace another input or output.

Writes use atomic replacement. On Unix, generated files are fresh owner-written
artifacts with mode `0644`, matching the established consumer format.

## What the checks prove

The source-independent `check-consumer` command rejects:

- malformed manifest, source, profile, protocol, digest, or exception records;
- overlay or exception-input drift from the manifest;
- generated source, commit, profile, schema, import-table, or local-rule drift;
- an empty canonical protocol section.

The source-backed `check` command composes that proof with the pinned checkout
and additionally rejects:

- malformed, unindexed, or stale protocol catalog data;
- active protocols absent from every profile or retired profile members;
- symlinked or special catalog sources;
- malformed or unrenderable profiles and unknown or duplicate profile members;
- a source checkout different from the manifest's full Git SHA;
- protocol, profile, canonical prose, manifest, or complete generated-file
  drift.

`validate` shares the catalog and profile checks. The separate `version-check`
history proof rejects protocol ID removal, version regression, source changes
without a version bump, introduction as retired, and reactivation.

Neither check command proves that an agent followed the prose, that a
repository's code satisfies the rules, or that an exception was truthfully
approved. The `audit-agent-protocols` skill performs the human-semantic part of
an import audit and reports it separately from the mechanical result.

## Protocol lifecycle

Protocol IDs use `AP-<AREA>-<NUMBER>` and are permanent. `AREA` starts with an
uppercase letter and contains non-empty uppercase-alphanumeric segments
separated by single hyphens; `NUMBER` contains three decimal digits. Every
change to a canonical protocol file bumps its positive integer version because
an automated history gate cannot safely distinguish editorial from semantic
wording. Run `version-check` before `index`. Its base must be a non-empty Git
revision that resolves to a commit; the checker reads the historical index from
that exact commit. Important rationale and rejected alternatives live in Beads.

To retire a protocol, bump its version, add `status: retired` to its
frontmatter, and remove it from every profile in the same source change. Then
run `version-check`, `index`, and `validate` normally. The index retains the
record, profiles cannot select it, and the history gate prevents both new IDs
from starting retired and retired IDs from reactivating.

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
src/starcompass.rs         index, render, and drift-check owner
src/main.rs                `starcompass` process boundary
tests/                     focused contract tests
skills/                    protocol-only Codex skills
docs/DESIGN.md             architecture decisions and boundaries
docs/BEHAVIOR_TRIALS.md    manual isolated agent-behavior cases
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
target/debug/starcompass check-consumer \
  --overlay .agent-protocols.local.md \
  --exceptions .agent-protocols.exceptions.json \
  --manifest .agent-protocols.json \
  --agents AGENTS.md
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

The scorecard counts tracked project text and code, excluding `.beads/` state
and lockfiles.

| Surface | LOC |
| --- | ---: |
| Protocols and generated index | 466 |
| Profiles | 104 |
| Rust checker and package manifest | 1,416 |
| Tests | 902 |
| Enforcement skills | 175 |
| Repository policy, docs, and licenses | 1,816 |
| **Total** | **4,879** |
