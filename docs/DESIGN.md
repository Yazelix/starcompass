# Starcompass design decisions

## Name and compatibility boundary

Starcompass is the project identity. The name describes a common source of
direction that helps agents navigate project-specific decisions without taking
scope away from the user. “Star” distinguishes the project from the many tools
named Compass and keeps it adjacent to the wider Yazelix constellation without
making Starcompass specific to that stack.

Branding does not silently rename compatibility surfaces. Permanent `AP-*`
protocol IDs, `.agent-protocols.*` consumer files, and existing skill IDs keep
their technical names. New import manifests record the Starcompass repository
URL. Existing manifests remain exact historical import records and receive the
current source URL when deliberately upgraded.

## The durable unit is a protocol

Each rule has a permanent ID, a positive unsigned integer version, a title, and
canonical Markdown. The generated index records the file hash. Profiles refer
only to IDs, so wording has one owner and profile composition cannot create
hidden copies.

Catalog directories must be real directories, while protocol and profile
sources and the generated index must be regular files in the selected checkout.
Following symlinks would let a clean commit claim mutable bytes that Git records
only as a target path.

Protocol versions describe compatibility at the rule level. Every canonical file
change requires a bump because automation cannot safely distinguish editorial
wording from a semantic change. The history gate also prevents removal or reuse
of an ID. It first resolves the requested base to a commit, then reads that
commit's historical index; worktree or index pseudo-revisions cannot satisfy the
gate. Git commits identify the exact catalog, profiles, and rendering
implementation used for an import. Consumers record both because neither
replaces the other.

Permanent identity does not require permanent profile membership. Protocol
frontmatter owns lifecycle: no `status` field means active, while
`status: retired` records an irreversible retirement. Active protocols must
appear in at least one profile; retired protocols remain source-backed and
indexed but must appear in none. New IDs must begin active. Retirement changes
the canonical file, so the version-bump rule applies. Profile-wide
membership requirements apply only while their owning protocol remains active.
Keeping the retired body and digest preserves historical authentication without
a separate archive, retirement registry, or synthetic profile.

## Profiles compose; projects still own policy

The three profiles are curated ordered sets, not inheritance trees. Repeated
`--profile` arguments form an ordered union and discard duplicate protocol IDs at
their second occurrence. This supports combinations such as `surgical` plus
`release` without inventing a profile for every project type.

`regular` and `surgical` are postures: how much change a project welcomes.
`surgical` contains every `regular` protocol, and a test keeps it that way, so
a consumer picks one of the two. `AP-SURGICAL-001` states in its own text that
it wins where `AP-MINIMAL-001` differs, so an agent does not have to resolve
that conflict itself. `release` is an add-on for one kind of product, so it
lists only its own protocols and the two mandatory ones and renders after a
posture. Fork, orchestrator, and terminal-engine role profiles were retired:
a soft fork takes the `surgical` posture, and component rules belong in the
overlays of the consumers that have those components.

Profiles contain shared rules only. Repository-specific commands, branches,
tools, paths, ownership, and release rules belong in the consumer overlay. A
consumer can alter canonical behavior only through an explicit exception.

## Delivery proof precedes promotion

The optional `release` add-on separates proving a delivered artifact from
changing who receives it. `AP-DELIVERY-001` owns installation, identity, and
fresh-session evidence. `AP-PROMOTION-001` owns ordered channel movement and
requires the exact already-proved revision. Promotion cannot manufacture proof.

The model adopts the reusable mechanism from Yazelix Nova's
[`AGENTS.md`](https://github.com/Yazelix/nova/blob/303d4bea457cf669035aabce8848b39c96287785/AGENTS.md):
one linear history, promotion-only accepted channels, exact-revision
fast-forwards, fresh-session dogfood for interaction changes, and additive
reverts that traverse the normal promotion path. Starcompass does not import
Nova's branch names, weekly cadence, Nix commands, platform checks, or runtime
ownership. Consumers declare those local details and explicitly choose whether
the release add-on applies.

## Self-contained output beats remote inclusion

Agents must be able to read the complete active policy within the consumer,
including offline and at historical revisions: from `AGENTS.md`, or from it and
the generated protocol file it names. Remote includes and
runtime fetches would make behavior depend on availability and mutable external
state. Rendering duplicates text deliberately at the consumption boundary while
retaining one editable canonical source.

The generated import record makes that duplication auditable: repository, exact
commit, profiles, protocol IDs, versions, hashes, and approved exceptions are
visible without opening another file.

Self-contained does not mean unbounded. Each shipped profile has its own cap
on its empty-overlay canonical/import section, about 200 bytes above its
measured size, so space trimmed from a small profile cannot quietly refill
under a larger profile's cap: `regular` 12,400 bytes (measured 12,192; 11,964
as `greenfield` before the trim), `surgical` 14,100 (13,908), and `release`
4,400 (4,360; 12,451 as a full profile). The largest combination,
`surgical + release`, is limited to 16,600 bytes (16,415), leaving about
15.8 KiB of local rules under Codex's default 32 KiB project-document limit. The
budget belongs to canonical composition; consumers
still own their overlays, and the renderer does not reject host-specific total
sizes.

## Local overlay and exceptions are different

An overlay adds repository detail without weakening canonical requirements. An
exception narrows, replaces, or suspends one imported protocol. Conflating them
would hide policy changes in ordinary local instructions.

`render` requires the exception authoring file. Both check commands compare it
with the generated manifest, exposing approval changes that have not been
rendered.

## Structural integrity is not compliance

The checker has two deterministic proof tiers. `check-consumer` needs only the
consumer files: overlay, exception input, manifest, generated `AGENTS.md`, and
any nested or protocol files the manifest records. It validates
their schemas and locally derivable relationships, reproduces the import record
and final local-rule framing, and treats the canonical protocol section as
opaque. It neither reads nor discovers a source root.
The renderer rejects backticks and line endings in inline import metadata and
folds exception line endings. These constraints keep self-consistent inputs from
injecting Markdown blocks through generated framing.
Catalog validation applies the same profile-name grammar, so every accepted
profile can appear in the import record.

Before rendering, the checker canonicalizes each path's longest existing
ancestor and normalizes the unresolved suffix. This catches parent traversal and
symlink aliases without creating missing output directories during validation.

Source-backed `check` invokes the consumer proof first, then uses the exact
pinned checkout to reconstruct the catalog, composition, protocol digests,
canonical prose, and complete generated file. Keeping the narrow proof inside
the full proof gives the consumer format one interpretation owner without
claiming that self-reported hashes authenticate unavailable source.

Neither tier can prove that an agent read or obeyed instructions, that an
approval claim is true, or that source code follows the protocols.

The audit skill preserves this boundary by reporting mechanical drift and
semantic ambiguity separately. Runtime enforcement, telemetry, and agent-action
attestation are outside the initial scope.

## Planning records outcomes

Beads exports one mutable snapshot per issue. Agents keep its contract,
decisions, constraints, acceptance evidence, and material failures aligned with
the accepted result. Append-only audit records retain chronology when later
work needs it as evidence.

An implementation run binds to one owning issue and returns control before
starting another. The user may authorize a bounded planning-only batch over a
named or accepted issue set. A batch may create, update, or close those records.
It cannot claim work, edit implementation, or combine planning with an
implementation run. Unapproved findings remain reports for later user direction.

Review corrections update the owning issue and the current task's unpublished
commit. Agents refresh local SHA-pinned artifacts after amendment and add
follow-up commits after publication or an external pin. A separate issue records
only a material finding outside the owning scope or one worth scheduling.

## Upstream agent tools belong in the consumer overlay

The catalog owns stable, self-contained invariants that every consumer of a
profile needs. A third-party agent tool keeps ownership of its runtime
integration, modes, hooks, skills, and host adapters. Starcompass does not
vendor or reproduce that surface, and the structural checker does not install
or attest it. A fork is justified only after a recorded incompatibility cannot
reasonably be fixed upstream.

Canonical text does not name such a tool. Each directive in a profile costs
attention in every consumer, and most consumers lack any given tool. A
repository that uses one records it, with the reviewed upstream revision, in its
own overlay.

`AP-MINIMAL-001` evaluates minimality across the accepted system and lifecycle,
including coordination, coupling, portability, migration, operations, and proof.
Patch size is only a tie-breaker among shapes satisfying the same contract and
the ownership duties of `AP-OWNERSHIP-001`, so a locally larger root-cause or
owner correction can be the minimum solution. The protocol owns its code-level
rules instead of deferring to an external code-shape tool: build only for
current callers, finish the change and delete its orphans, report unrelated dead
code, and handle implied bad input. Each is a requirement with its reason,
because agents keep requirements longer than prohibitions in long sessions.

## Dependency decision

The checker is a Rust 2024 binary with Rust 1.85 as its minimum toolchain. Rust
was selected while the implementation and compatibility surface were still
small: a distributable binary, typed schemas, and one language across the wider
stack outweigh the build cost. The 620-line Python implementation served as the
byte-parity oracle and was removed after the Rust contract tests matched its
manifest, exception, index, and `AGENTS.md` output. Go would also provide a
single binary but would add a second compiled language to the stack. POSIX shell
is too brittle for nested JSON, Unicode, atomic writes, and precise validation.

The direct dependency gate selects exact releases:

- `serde` 1.0.229 with derive and `serde_json` 1.0.151 own typed JSON schemas and
  deterministic serialization. A handwritten parser and the untyped `json`
  crate duplicate or weaken schema ownership; `simd-json` solves irrelevant
  throughput pressure.
- `sha2` 0.11.0 with default features disabled owns portable SHA-256. Owned
  cryptography, an external command, `ring`, and system crypto broaden the
  correctness or platform surface.
- `atomic-write-file` 0.3.0 owns all-old-or-all-new replacement across Unix,
  Windows, and WASI. A standard-library temp-and-rename path diverges on Windows;
  `tempfile` owns a broader lifecycle, and direct bindings create local platform
  code. On Unix, Starcompass disables existing-file metadata preservation and
  sets mode `0644` to retain the established fresh-output contract.

The standard library owns the fixed CLI, diagnostics, Git subprocesses, and
test fixtures. `clap`, `lexopt`, `pico-args`, `anyhow`, `thiserror`,
`assert_cmd`, and `tempfile` do not remove enough local behavior to justify
their cost. The parser permits repeated `--profile` arguments and rejects
repeated scalar options before command I/O. The measured lock graph contains
four direct crates and 35 registry package instances across normal, build, and
platform-specific metadata.
Serialization, hashing, and atomic replacement each have one narrow call
boundary, so a future replacement does not change the Markdown or JSON
compatibility surface.

## Claude Code first, Codex supported

Starcompass is written for Claude Code first. Protocols and skills may name its
mechanisms, such as subagents, skills, and hooks, instead of describing a
neutral host. Codex stays a supported client: it reads the same rendered
`AGENTS.md`, its skill metadata stays in each skill's `agents/openai.yaml`, and
it serves as a second-model reviewer. A Claude-only stance was rejected because
it would demote that reviewer to a guest tool.

Supporting Codex keeps its limits binding. Codex is the second most important
client, so its default 32 KiB project-document limit still sets the byte budgets
above and the nested-chain check; Claude-primary wording never loosens them.

Skill validation runs on the primary host: `claude plugin validate --strict`
checks each skill as Claude Code loads it. It tolerates malformed YAML
frontmatter that Codex's `quick_validate.py` rejects, so that script remains an
extra check where Codex is installed.

## Skill boundary

Starcompass holds the owner's agent skills of any kind, not only the two that
apply and audit these protocols, so one repository owns both the rules and the
skills that carry them out. A rule may name a skill that lives here. The
skills came from `luccahuguet/skills` with their history; that repository is
archived. Co-location is not a claim that skills are canonical protocol text;
skills reference protocols by ID and use the catalog and checker rather than
copy protocol wording.

Durable agent context is part of system lifecycle cost. Instructions belong at
the narrowest scope that needs them, retain non-obvious constraints and reusable
workflows that change behavior, and load detailed references only for relevant
tasks. Generic model capability and policy already owned elsewhere do not need a
second durable copy. Structural validation proves that an instruction artifact
is well formed and discoverable; representative isolated task trials supply the
separate behavioral evidence for consequential changes when the user authorizes a
separate agent, and otherwise that evidence is reported as missing.

## Bootstrap and publication

Starcompass dogfoods the ordinary consumer format.
Its generated root `AGENTS.md` cannot pin the commit that contains itself:
committing the result would immediately change the claimed source revision.
Instead, the manifest pins the latest accepted protocol-bearing revision and
the generated adoption lands in a follow-up revision, like a compiler built by
its preceding version.

Canonical protocol, profile, and checker changes are committed and verified
under the existing pin before a clean detached checkout at that new revision
renders the next self-adoption. The exact-pinned checkout also runs the
structural check. This uses the public profile, manifest, renderer, and checker
without a special self-hosting mode or exception.

The repository is public under `MIT OR Apache-2.0`, sets `publish = false`, and
has no hosted CI. Public source at exact Git revisions is the selected initial
distribution surface. Crate or binary releases, supported targets, credentials,
and hosted checks remain separate approval gates.

The source-independent checker makes distribution useful but does not select a
channel. The decision gate compares:

| Candidate | Consumer fit | Irreversible or recurring cost |
| --- | --- | --- |
| Public Git source (selected) | Exact commits expose the complete catalog, renderer, checker, and history without credentials | Consumers need Git and a Rust build for cold use; a branch name is mutable, so imports must keep pinning full commits |
| crates.io binary package | `cargo install --locked starcompass --version X.Y.Z` builds the source-independent checker for the consumer host without Starcompass Git access | Publishes the packaged source permanently; requires a crates.io token, SemVer/MSRV ownership, and consumer compile time; source-backed commands still need the canonical catalog |
| Public checksum-pinned binary | Fast cold CI with an explicit version, target triple, and independently pinned SHA-256 | Requires a public artifact location, one build per supported target, checksum publication, replacement/revocation policy, and release ownership; a downloadable asset is not immutable by URL alone |
| Authenticated private binary | Preserves repository and artifact privacy | Every consumer needs read credentials; cross-repository Actions and forks need explicit token/secret behavior; revocation can break cold CI |

The package disables Cargo library auto-discovery and exposes only the
executable target. `src/starcompass.rs` and its submodules remain private
modules for separation and unit tests. crates.io is the leading artifact
candidate because Cargo supplies host compilation, exact-version installation,
and lockfile use without a target matrix. A crate-installed binary would make
`check-consumer` source-independent, while `render`, source-backed `check`,
`validate`, and `index` still require the canonical catalog from a checkout.
Its permanent source publication and release-contract consequences keep it
unselected. A public prebuilt binary is the fallback when consumer compile cost
becomes material enough to justify release targets and artifact maintenance.
Authenticated private artifacts and copied or vendored checkers do not fit
unattended downstream adoption.
