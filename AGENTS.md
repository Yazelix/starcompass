# Agent Guidelines

This file is self-contained. Canonical protocol text was rendered into it;
the source repository is needed only to update or verify the import.
Do not edit this generated file directly. Edit `.agent-protocols.local.md`
or `.agent-protocols.exceptions.json`, then render from the pinned source.

## Protocol import record

- Source: `https://github.com/Yazelix/starcompass`
- Source commit: `4b1ea8e2977569fe4846b414e9d0ef1726adde1d`
- Profiles: `greenfield`
- Manifest: `.agent-protocols.json` (schema 1)

| Protocol | Version | SHA-256 |
| --- | ---: | --- |
| `AP-SCOPE-001` | 4 | `8d84dc6fca6097b2055eb3ca8467565709e4c474a18c3030ae3064cc612bdfab` |
| `AP-CONTRACT-001` | 4 | `cdf2e5d69cefd34ceeeaa6b7511b21f921e2be50c17d3c01b85120b20234cff7` |
| `AP-REFERENCE-001` | 3 | `186fa74aa530c9c76ae507685461dff2b1506ecb5087230ded51f02cecb9231a` |
| `AP-MINIMAL-001` | 4 | `612bc9c62a20adf7332724d4663af0a0e591d624de2a76b1c257a9ed428db1f6` |
| `AP-DEPENDENCY-001` | 2 | `ded49add538b82de0a9c522bc8a34720f4ebbb47f39fc2f7ddb25e7aad1700d3` |
| `AP-OWNERSHIP-001` | 2 | `d218a4e0625b659ec366284110bdfce02bd66cb229ab6ba07f2317092ea13053` |
| `AP-TEST-001` | 3 | `58b5837cb679e958192b366bb15d6e34649f5a91ff9f4accdc7edb4ef5cdb873` |
| `AP-PROOF-001` | 6 | `01fc3712fd369189b934b0503173e6a8963be64b7f18869abfaef3cca53866f7` |
| `AP-PLAN-001` | 11 | `fcb7c8d2b99c6641a7cd1244072eda6230f1ccfc3a4aa47c29cf2525fcb1376d` |
| `AP-SESSION-001` | 1 | `f8bdac637027895ec8d1ad4f192e13808d095a809d091b76eadadaf936b2a340` |
| `AP-CI-001` | 3 | `c7cb65a81ce8434d02f2d19306bf93358f3624d0b77886bc91d4de93f7a779ba` |
| `AP-EXCEPTION-001` | 2 | `2c4f00299922edac83286821af07ad485e5a630bca6acbbce918d87614abb395` |
| `AP-GIT-001` | 5 | `f2c3254311de57a23a13aa382fe3285e65bb34e535ecd813d1be783ca79d4bf9` |

### Local exceptions

No local exceptions.

## Canonical protocols

### AP-SCOPE-001 — User-owned scope

The user owns scope. Inspection, audit, diagnosis, explanation, and advice
authorize no implementation, external-state, or durable-planning write.
Implement only a chosen outcome.

Never silently add features, compatibility promises, public surfaces,
migrations, repositories, or planning items. Preserve user inputs and
artifacts; replace or delete an exact target only when required, else write a
distinct result. State consequential assumptions; stop when a missing scope
choice would materially change the result. Recommend and justify an answer to
every choice. “Finish” adds persistence, not authority. Report out-of-scope
findings unless the user chose a durable home.

### AP-CONTRACT-001 — Contract-driven changes

Before code shape, state the smallest observable contract: consumer, trigger,
result, and important failures. Of the contracts consistent with the request
and evidence, choose the fewest unsupported guarantees or restrictions; leave
reasonable future behavior unspecified. Give it a stable ID only when later
consumers need one.

Keep docs, help, examples, and configuration aligned with current commands,
paths, flags, defaults, and availability; label planned, partial, or gated
behavior. Choose one source-of-truth owner, the cheapest falsifying check, and
the smallest complete vertical slice. Update the contract first for an
intentional behavior change. Implementation details are not contracts unless a
component must rely on them.

### AP-REFERENCE-001 — Evidence before code shape

Before a consequential code shape, read affected instructions, code, contracts,
tests, and required subsystem references. Record the adopted, rejected, or
unresolved mechanism; separate evidence from inference and revisit it after a
material shape change. Memory, summaries, and reputation are discovery only.

Apply source-license terms to their exact actors, uses, conditions,
beneficiaries, and direction. Choosing a provider does not make a user act for
it; an “including” example remains scoped by its condition. Inspection is
distinct from copying, adaptation, redistribution, dependency selection, and
incorporation, so a restriction on one does not spread to another. If an
interpretation would block required evidence, identify the exact clause and
roles and resolve material ambiguity with the user. Review does not require
reuse.

### AP-MINIMAL-001 — Minimum sufficient implementation

After identifying the contract, evidence, owner, flow, and boundaries, take the
first sufficient option: no change; existing owner, helper, or pattern;
standard library or native platform; accepted dependency that owns the
behavior; minimum correct local code.

Judge the whole lifecycle, including duplicate truth, coordination, coupling,
migration and removal, portability, operations, proof, and agent context. Scope
instructions narrowly; retain non-obvious constraints and reusable
behavior-changing workflows, not generic or duplicated policy. Load details
only when needed.

A patch is not minimal if it preserves a wrong or duplicate owner, treats a
symptom below its shared cause, bypasses a boundary, or raises downstream cost.
Prefer deletion, direct ownership, and fewer files; use patch size only between
equally correct system shapes. Never remove required behavior, trust-boundary
validation, data-loss protection, security, accessibility, or the cheapest
runnable check for non-trivial logic.

When available, use upstream
[Ponytail](https://github.com/DietrichGebert/ponytail/tree/16f29800fd2681bdf24f3eb4ccffe38be3baec6b)
as a fallible code-shape bias after these constraints. Its absence does not
suspend them, and its hooks do not prove compliance.

### AP-DEPENDENCY-001 — Dependency gate

Before adding a dependency, name its capability and contract. Compare owned
code, the standard library, and credible candidates for correctness,
maintenance, platform and license fit, transitive weight, API stability, and
net complexity. Record the choice, meaningful rejections, and replacement cost;
pin it and prove the relied-on behavior. Remove it when its ownership no longer
justifies its cost.

### AP-OWNERSHIP-001 — One owner per invariant

Give every invariant, state transition, and user-visible policy one owner;
consumers must not reconstruct or reinterpret it. Name the owner before adapters
or synchronization and delete duplicates. Put policy at the highest layer with
enough context and enforcement at the lowest correct layer. Make cross-boundary
data explicit and versioned for independently released consumers.

### AP-TEST-001 — Strong and few tests

Protect meaningful contracts, regressions, boundaries, and failures with a few
strong tests. Use TDD for deterministic helpers, parsers, protocol behavior,
and regressions whose expected behavior is known first; use contract-first
integration checks for layout, runtime, architecture, forks, and dogfood.

For consequential agent-instruction, prompt, or skill changes, run isolated
representative tasks without supplying the expected conclusion. Structural
checks prove structure, not agent behavior. Test observable effects; delete
duplicate proof, implementation trivia, and scaffolding. Guard absence only
when absence is a security, licensing, size, ownership, or known-regression
contract.

### AP-PROOF-001 — Explicit proof lifecycle

Proof records revision, command/observation, environment, result/surface,
and phase: proposed, implemented, mechanically verified, dogfooded, accepted,
or promoted. Never widen/reuse after material change.

Use the highest boundary exercising the changed contract. Add broader/lower
suites only for changed owner, failure, trust, distinct delivery, or
promotion—not availability, issue text, or prior phase. Exact downstream proof
subsumes lower checks only for the identity/behavior exercised.

Run a complete suite once per unchanged candidate/environment. Metadata/docs/
planning follow-ups reuse it; commands are evidence, not a checklist. Keep logs
only when reproduction is costly, disputed, performance-sensitive, or lossily
summarized; temporary paths are not proof.

Performance claims need a measured bottleneck, fixed workload/environment,
baseline, oracle, and distribution/bound; retain constraining negatives. Review
changes require another pass; convergence needs a full clean pass.

### AP-PLAN-001 — Durable planning state

Keep later-needed outcomes/constraints in planning or canonical docs. Issues
hold chosen goals/decisions, material defects, or schedulable follow-ups;
methods stay with their owner. Acceptance names outcomes/cheapest falsifiers,
not command catalogs unless the boundary/transition needs them.

A run includes continuations until control returns; reads are unrestricted. A
writing run does exactly one:

1. Owns at most one issue and may create/claim/update/implement/close it.
2. With explicit user authorization, changes a named/accepted planning-only
   batch; claims nothing and changes no implementation.

Bind implementation before writes; return at completion, block, or handoff.
Report unapproved findings; create issues only for material out-of-scope or
schedulable work, deferred outside authorized batches.

Keep fields current; append only needed contracts, decisions, dependencies,
acceptance, material failures/negatives, constraining rejections, and approvals.
Baseline/reference comments record only changed identities, dirty state,
mechanism/stop decisions, and first check. Keep status/prerequisites honest; use
the issue tool, never its storage.

### AP-SESSION-001 — Session continuity across compaction

Treat context compaction as lossy. Write each user decision to its durable
owner when it is made, and reread the active issue after any compaction. Once
a session has compacted, start the next issue in a fresh session; within an
issue, a second compaction or any sign of lost context triggers a handoff
comment (state, next step, open decisions) and a new session. Prefer a
clean-break restart to compaction, and start a large issue fresh when most of
the context is already used.

### AP-CI-001 — Bounded continuous integration

Before hosted automation, name its contract and why local proof is
insufficient. Bound triggers, jobs, timeouts, permissions, artifacts, cache,
concurrency; specify fork and secret behavior. Prefer one cheap deterministic
job. Record when to expand, reduce, or remove. Confidence must justify its
financial, latency, security, and maintenance costs, including private minutes
and storage.

### AP-EXCEPTION-001 — Explicit local exceptions

Only an exception approved by the user or named authority may narrow, replace,
or suspend an import. Record protocol ID, exact scope, reason, approver, date,
and any expiry or review condition. Unrecorded conflicts are drift; additive
detail is an overlay.

### AP-GIT-001 — Safe repository history

Repository history is shared user state. Inspect status and local policy;
preserve unrelated or concurrent work and use the least destructive sufficient
operation.

Amend the unpublished task commit for same-task corrections; refresh downstream
commits and generated artifacts. After external reliance (push, promotion,
release, or pin), commit corrections separately. Never reset, discard,
force-push, rewrite published history, or create a branch without authority.
Verify intended diff before commit and remote state after push. Roll back with a
reviewed revert unless local policy says otherwise.

## Repository-local rules

### Agent Guidelines

Starcompass is the canonical source for small, composable development protocols
used by coding agents. It publishes stable protocol IDs, profiles, rendered
`AGENTS.md` files that are self-contained within the consumer, and structural
drift checks.

## Core Rule

The user decides scope. Do not add a protocol, profile, enforcement skill,
compatibility surface, or planning bead until the user has chosen that
direction.

## Ownership Model

- `protocols/*.md` owns protocol wording and metadata.
- `protocols/index.json` is generated by `starcompass index`; never edit it
  directly.
- `profiles/*.json` owns ordered protocol composition.
- `src/starcompass.rs` and its `src/starcompass/` modules are the sole owner of
  validation, rendering, and drift detection; each module's `//!` header names
  the one concern it owns. `src/main.rs` owns only process I/O and exit status.
- `skills/` contains only skills whose sole purpose is applying or auditing
  these protocols.
- A consumer repository owns its local overlay and explicit exceptions.

Protocol IDs are permanent. Never recycle, rename, or remove an ID. Every change
to a canonical protocol file must increment that protocol's positive integer
version; the history gate cannot safely infer whether wording is editorial.

Generated instructions must be self-contained within the consumer. An agent
must not need network access or this repository to understand the active rules;
canonical protocol text lives in `AGENTS.md` or in a generated consumer file
that `AGENTS.md` names. Each import
records the source repository, exact source commit, protocol IDs and versions,
and every approved local exception.

The checker proves structural integrity and byte-for-byte drift. It does not
claim that an agent complied with prose rules.

Profiles compose by ordered union. Preserve the order supplied by the consumer
and de-duplicate protocol IDs at their first occurrence. Do not hide local rules
inside a custom profile; keep them in the consumer overlay.

## Method

Before selecting a code shape for work with durable planning state:

1. State the user-visible or operator-visible contract in the owning Bead.
2. Inspect the affected protocol, profile, format, checker, and tests.
3. Review relevant prior art or local precedent and record what is adopted or
   rejected when the decision matters later.
4. Consider the standard library and multiple credible dependencies before
   adding one; record the winner, rejected candidates, and exit cost.
5. Choose one owner and the cheapest check that proves the contract.
6. Implement the smallest complete slice.

Prefer dependency-free code and transparent formats. Do not add a parser,
framework, template engine, or package merely to save a small amount of local
code. Do not duplicate canonical protocol text in profiles, skills, or docs.

## Testing

Keep tests strong and few. Tests should prove stable IDs and versions, invalid
registry/profile rejection, deterministic rendering, exception recording, or
drift detection. Avoid mirror tests for literals and implementation trivia.

Use temporary directories for generated consumer fixtures. Do not commit
generated test output unless it is a deliberate compatibility fixture.

## Beads

Use `br` for all issue work. Do not edit `.beads/` files directly. Serialize
`br` writes. Treat Bead fields as mutable current state and reserve comments or
`br audit` for chronology needed as evidence. Run `br sync --flush-only` before
committing Beads changes.

## Git Workflow

Work directly on `edge`. Do not create another branch unless the user asks.
Keep commits focused, preserve unrelated user changes, and never rewrite
published history or force-push.

## Bootstrap Self-Adoption

Starcompass consumes its own `greenfield` profile from the exact prior
protocol-bearing revision recorded in `.agent-protocols.json`. The root
`AGENTS.md` and `.agent-protocols.json` are generated outputs; edit the local
overlay or exception input instead.

A same-commit self-pin is impossible because committing generated adoption files
changes the commit. Do not add a special self-hosting profile, checker mode, or
exception to hide that fact. Changes to canonical protocols, profiles, or the
checker use two revisions: make and verify the source change under the existing
pin, commit it, then render the self-adoption from a clean detached checkout at
that exact source commit in a follow-up revision. Run the structural check from
the checkout named by the manifest, never from a merely newer checkout.

## Documentation and LOC

Keep `README.md` aligned with the current command and file formats. Update
`CHANGELOG.md` when protocol behavior, profile composition, import metadata, or
checker behavior changes.

Update the README LOC scorecard whenever project files change. Generated Beads
state and lockfiles are excluded; protocol index lines are included because the
index is part of the published surface.

## Verification

Run the cheapest checks for the changed surface. For the full repository:

```sh
cargo fmt --check
cargo check --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
python3 /home/lucca/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/apply-agent-protocols
python3 /home/lucca/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/audit-agent-protocols
git diff --check
```

Do not add hosted CI until the user explicitly chooses its cost and trust
model.
