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

Protocol versions describe compatibility at the rule level. Every canonical file
change requires a bump because automation cannot safely distinguish editorial
wording from a semantic change. The history gate also prevents removal or reuse
of an ID. It first resolves the requested base to a commit, then reads that
commit's historical index; worktree or index pseudo-revisions cannot satisfy the
gate. Git commits identify the exact catalog, profiles, and rendering
implementation used for an import. Consumers record both because neither
replaces the other.

## Profiles compose; projects still own policy

The five profiles are curated ordered sets, not inheritance trees. Repeated
`--profile` arguments form an ordered union and discard duplicate protocol IDs at
their second occurrence. This supports combinations such as a greenfield
terminal engine without inventing a profile for every project type.

Profiles contain shared rules only. Repository-specific commands, branches,
tools, paths, ownership, and release rules belong in the consumer overlay. A
consumer can alter canonical behavior only through an explicit exception.

## Delivery proof precedes promotion

The optional `release` profile separates proving a delivered artifact from
changing who receives it. `AP-DELIVERY-001` owns installation, identity, and
fresh-session evidence. `AP-PROMOTION-001` owns ordered channel movement and
requires the exact already-proved revision. Promotion cannot manufacture proof.

The model adopts the reusable mechanism from Yazelix Nova's
[`AGENTS.md`](https://github.com/luccahuguet/yazelix/blob/303d4bea457cf669035aabce8848b39c96287785/AGENTS.md):
one linear history, promotion-only accepted channels, exact-revision
fast-forwards, fresh-session dogfood for interaction changes, and additive
reverts that traverse the normal promotion path. Starcompass does not import
Nova's branch names, weekly cadence, Nix commands, platform checks, or runtime
ownership. Consumers declare those local details and explicitly choose whether
the release profile applies.

## Self-contained output beats remote inclusion

Agents must be able to read the complete active policy from the local
`AGENTS.md`, including offline and at historical revisions. Remote includes and
runtime fetches would make behavior depend on availability and mutable external
state. Rendering duplicates text deliberately at the consumption boundary while
retaining one editable canonical source.

The generated import record makes that duplication auditable: repository, exact
commit, profiles, protocol IDs, versions, hashes, and approved exceptions are
visible without opening another file.

## Local overlay and exceptions are different

An overlay adds repository detail without weakening canonical requirements. An
exception narrows, replaces, or suspends one imported protocol. Conflating them
would hide policy changes in ordinary local instructions.

The exception authoring file and generated manifest are both checked. This makes
an unrendered approval change visible instead of letting the manifest silently
remain authoritative.

## Structural integrity is not compliance

The checker is deterministic and deliberately narrow. It can prove that the
catalog, composition, metadata, inputs, and generated output agree. It cannot
prove that an agent read or obeyed instructions, that an approval claim is true,
or that source code follows the protocols.

The audit skill preserves this boundary by reporting mechanical drift and
semantic ambiguity separately. Runtime enforcement, telemetry, and agent-action
attestation are outside the initial scope.

## Adopt upstream implementations without cloning them

The catalog owns stable, self-contained invariants. When an upstream project
already owns a strong portable implementation, that project retains ownership
of its runtime integration, modes, hooks, skills, and host adapters. A canonical
protocol records the reviewed upstream revision and enough local wording to
remain usable without network access.

`AP-MINIMAL-001` follows this boundary with Ponytail. Consumers use Ponytail
directly where supported; Starcompass does not vendor or reproduce its
adapter surface. A fork is justified only after a recorded incompatibility
cannot reasonably be fixed upstream. The structural checker does not install or
attest third-party runtime integrations.

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
their cost. The measured lock graph contains four direct crates and 31 registry
packages across normal, build, and platform-specific metadata. Serialization,
hashing, and atomic replacement each have one narrow call boundary, so a future
replacement does not change the Markdown or JSON compatibility surface.

## Skill boundary

Skills live here only when their sole responsibility is applying or auditing
these protocols. General writing, UI, review, opinion, and repository-management
skills have independent lifecycles and belong elsewhere. Co-location is not a
claim that skills are canonical protocol text; skills must reference the catalog
and checker rather than copy protocol wording.

## Bootstrap and publication

Starcompass dogfoods the ordinary consumer format with the `greenfield` profile.
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

The repository starts private and without hosted CI. This keeps early format
iteration and GitHub Actions cost explicit. Publication, licensing, releases,
and hosted checks require later user decisions.
