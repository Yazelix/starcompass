# Agent Guidelines

This file is self-contained. Canonical protocol text was rendered into it;
the source repository is needed only to update or verify the import.
Do not edit this generated file directly. Edit `.agent-protocols.local.md`
or `.agent-protocols.exceptions.json`, then render from the pinned source.

## Protocol import record

- Source: `https://github.com/luccahuguet/starcompass`
- Source commit: `9822be230cdf6508da6599238af9de68ea148712`
- Profiles: `greenfield`
- Manifest: `.agent-protocols.json` (schema 1)

| Protocol | Version | SHA-256 |
| --- | ---: | --- |
| `AP-SCOPE-001` | 1 | `b3f7e012df0708d4baf8957e3c315878a9eb8cd7fddf637dfde1506609d08444` |
| `AP-CONTRACT-001` | 1 | `0aa692f4c52542111149b691b7d0c015a0c16523cd620a67b0cf335f3284da82` |
| `AP-REFERENCE-001` | 1 | `3d72ac864b050af7783493245be59450c39db940d736ea6fb60269cd21f7927b` |
| `AP-MINIMAL-001` | 1 | `256c158cc8b226e4baf96d5590531ea180edc9717338dac0fe51a34dd037791f` |
| `AP-DEPENDENCY-001` | 1 | `a389ff9054708574c52ec5e5dd7fc3e2d13b125d2218c70062f50a86761981ca` |
| `AP-OWNERSHIP-001` | 1 | `bdc09117f79b0d8dbe78e2dd8673a2463398aa31880fe27209fd6cc47f58bbf7` |
| `AP-TEST-001` | 1 | `363da7c542521be22233a4cc3373c0d3c3c5a9a0cf37f633cd5029545f4a3bee` |
| `AP-PROOF-001` | 1 | `1af235a56e9711d55362d869fa4057f1658d0aa7fe766030be0897ee5fd7c02b` |
| `AP-PLAN-001` | 4 | `5bc0426788693717e8af011277d9358c65899e839d7c0c71299003ec0d8acc4b` |
| `AP-CI-001` | 1 | `78f1662259cd83f33d22ff4ddd0859ab0d4f704ba4f38756eef40a8b9b787bec` |
| `AP-EXCEPTION-001` | 1 | `f66749229dbbc005e1c3103bfed86cf95169d7b32466c72fd8442e841cadb268` |
| `AP-GIT-001` | 3 | `16d27b0df7ccc94880bb31020e822e32b37503f43c2cf7a69de333300cbfdacf` |

### Local exceptions

No local exceptions.

## Canonical protocols

### AP-SCOPE-001 — User-owned scope

The user decides product and project scope. An agent may inspect, explain, test,
or make the smallest implementation needed for the chosen goal, but it must not
silently create a feature, compatibility promise, public surface, migration,
repository, or planning item outside that direction.

Required practice:

- Separate safe implementation details from choices that change product scope.
- State consequential assumptions; stop when a missing choice would materially
  change the result.
- Treat a terminal instruction such as “finish” as persistence, not broader
  authority.
- Keep useful out-of-scope observations as findings unless the user has chosen
  a durable planning destination for them.

### AP-CONTRACT-001 — Contract-driven changes

State the irreducible externally observable behavior before choosing the code
shape. Give durable contracts stable identifiers when later code, tests, or
repositories need to cite them.

Required practice:

- Name the consumer, trigger, observable result, and important failure behavior.
- Identify the current sources of truth and decide which one owner survives.
- Choose the cheapest check that can falsify the contract.
- Implement the smallest vertical slice that satisfies it.
- Update the contract first when an intentional behavior change is chosen.

Do not turn implementation details into contracts unless another component must
rely on them.

### AP-REFERENCE-001 — Evidence before code shape

Review the relevant sources before deciding architecture or implementation
shape. Memory, summaries, and reputation are discovery aids, not sufficient
evidence for a consequential decision.

Required practice:

- Read the affected local code, contracts, tests, and repository instructions.
- Inspect designated external references at the subsystem named by local rules.
- Record the concrete mechanism adopted, rejected, or left unresolved.
- Distinguish direct source evidence from inference.
- Revisit the evidence when the proposed shape changes materially.

Reference review is a decision gate, not a requirement to copy the reference.

### AP-MINIMAL-001 — Minimum sufficient implementation

Understand the affected flow before choosing the smallest complete solution.
Use the first option that fully satisfies the chosen contract:

1. Make no change when the required behavior already exists.
2. Reuse an existing owner, helper, or pattern in the repository.
3. Use the standard library or a native platform capability.
4. Use an already accepted dependency that owns the behavior.
5. Implement the minimum local code that is correct and maintainable.

Prefer deletion over addition, direct ownership over adapters, and fewer files
over scaffolding. Minimalism must not remove required behavior, trust-boundary
validation, data-loss protection, security, accessibility, or the cheapest
runnable check for non-trivial logic.

Ponytail is the adopted agent-side implementation of this discipline when the
host supports it. Use the upstream project directly rather than copying its
rules or adapters. The reviewed source is
[DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail/tree/16f29800fd2681bdf24f3eb4ccffe38be3baec6b).
If Ponytail is unavailable or disabled, the self-contained requirements above
still apply. Its instruction hooks improve consistency; they do not prove
compliance.

### AP-DEPENDENCY-001 — Dependency gate

Choose dependencies for architectural fit and net system simplicity, not name
recognition or short-term convenience.

Before adding a crate, package, framework, service, or embedded project:

- State the capability and contract it would own.
- Consider the standard library, owned code, and multiple credible candidates.
- Compare maintenance, platform fit, correctness, transitive weight, licensing,
  API stability, and the lines and complexity removed.
- Record the chosen candidate, meaningful rejections, and replacement cost.
- Pin deliberately and add the smallest check that proves the relied-on behavior.

Remove a dependency when it no longer owns enough behavior to justify its cost.

### AP-OWNERSHIP-001 — One owner per invariant

Every invariant, state transition, and user-visible policy must have one clear
owner. Other components may consume its output; they must not independently
reconstruct or reinterpret the same truth.

Required practice:

- Name the owner before adding adapters or synchronization.
- Prefer deleting duplicate owners over reconciling them.
- Keep policy at the highest layer that has the necessary context and mechanism
  at the lowest layer that can enforce it correctly.
- Make cross-boundary data explicit and versioned when independently released
  components depend on it.

### AP-TEST-001 — Strong and few tests

Tests exist to protect contracts, regressions, boundaries, and failure modes
that matter to users or future agents. Prefer one strong test with meaningful
setup and assertions over several thin tests.

Required practice:

- Use TDD for deterministic helpers, parsers, protocol behavior, and regressions
  when the expected behavior can be stated before implementation.
- Choose contract-first integration checks for layout, runtime integration,
  architecture choices, forks, and dogfooding surfaces.
- Delete or merge tests that duplicate another proof, assert implementation
  trivia, or preserve scaffolding.
- Test observable effects rather than mirroring literals, defaults, or source
  structure.
- Add absence guards only when absence is itself a security, licensing, size,
  ownership, or known-regression contract.

### AP-PROOF-001 — Explicit proof lifecycle

Claims and proofs have a lifecycle. A passing check supports only the exact
revision, environment, and surface it exercised.

Required practice:

- Record the command or observation, relevant environment, revision, and result.
- Distinguish proposed, implemented, mechanically verified, manually dogfooded,
  accepted, and promoted states.
- Re-run stale proof after relevant code, dependency, platform, or contract
  changes.
- Never promote a narrower check into a broader claim.
- Preserve important negative results; they constrain the next valid design.

### AP-PLAN-001 — Durable planning state

Keep the outcomes and constraints that later work needs in the project's
durable planning system or canonical documentation. An issue represents a
chosen goal, decision, material defect, or schedulable follow-up. Review and
implementation methods belong to that issue.

Required practice:

- After review, fresh-eyes, simplification, or verification, update the owning
  issue's editable fields to describe the accepted state instead of pass
  chronology.
- Create a separate issue only for a material finding outside the owning scope
  or one worth scheduling on its own. Name it after the outcome or finding.
- Record the contract, decision boundary, dependencies, acceptance evidence,
  material negative results, and rejected alternatives that constrain later
  work.
- Reserve append-only comments and audit records for chronology needed as
  evidence. Keep raw command logs and build transcripts with their proof. Omit
  baseline hashes, failed attempts, and candidate scoring unless they constrain
  later work.
- Keep issue status honest: planned, active, blocked, and complete are distinct.
- Model real prerequisites as dependencies; do not create decorative graphs.
- Reconcile planning state with the repository before handoff.
- Use the repository-designated issue tool and never edit its storage directly.

Do not erase approvals, contract changes, material failures, or evidence needed
to understand the accepted result.

### AP-CI-001 — Bounded continuous integration

Hosted automation must buy enough confidence to justify its financial,
latency, security, and maintenance cost.

Before enabling CI:

- Name the protected contract and why local verification is insufficient.
- Bound triggers, job count, timeouts, permissions, artifacts, cache growth, and
  concurrency.
- Prefer one cheap deterministic job before matrices or scheduled runs.
- Make fork and secret behavior explicit.
- Record the evidence required to expand, reduce, or remove the workflow.

Private-repository minutes and cache storage are product constraints, not an
invisible externality.

### AP-EXCEPTION-001 — Explicit local exceptions

A local rule may narrow, replace, or suspend an imported protocol only through
an explicit exception approved by the user or named project authority.

Each exception records:

- the protocol ID;
- its exact scope;
- the reason the canonical rule does not fit;
- who approved it and when;
- an expiry or review condition when the exception is temporary.

Unrecorded conflicts are drift. A local rule that merely adds detail without
changing the canonical requirement is an overlay, not an exception.

### AP-GIT-001 — Safe repository history

Repository history is shared user state. Preserve unrelated work, follow the
local branch and promotion model, and use the least destructive operation that
achieves the requested result.

Required practice:

- Inspect status and repository instructions before editing.
- Treat existing and concurrent changes as user-owned unless proven otherwise.
- Fold a correction into the current task's unpublished commit when it belongs
  to the same unit of work. Refresh and reverify dependent local commits and
  generated artifacts.
- Use a follow-up commit after a push, promotion, release, external pin, or any
  other point where someone outside the current local work can rely on the
  revision.
- Do not reset, discard, force-push, rewrite published history, or create a
  branch without authority from the user or repository policy.
- Verify the intended diff before committing and the remote state after pushing.
- Make rollbacks additive through a reviewed revert unless policy says otherwise.

## Repository-local rules

### Agent Guidelines

Starcompass is the canonical source for small, composable development protocols
used by coding agents. It publishes stable protocol IDs, profiles, rendered
self-contained `AGENTS.md` files, and structural drift checks.

## Core Rule

The user decides scope. Do not add a protocol, profile, enforcement skill,
compatibility surface, or planning bead until the user has chosen that
direction.

## Ownership Model

- `protocols/*.md` owns protocol wording and metadata.
- `protocols/index.json` is generated by `starcompass index`; never edit it
  directly.
- `profiles/*.json` owns ordered protocol composition.
- `src/lib.rs` is the sole owner of validation, rendering, and drift detection;
  `src/main.rs` owns only process I/O and exit status.
- `skills/` contains only skills whose sole purpose is applying or auditing
  these protocols.
- A consumer repository owns its local overlay and explicit exceptions.

Protocol IDs are permanent. Never recycle, rename, or remove an ID. Every change
to a canonical protocol file must increment that protocol's positive integer
version; the history gate cannot safely infer whether wording is editorial.

Generated `AGENTS.md` files must be self-contained. An agent must not need
network access or this repository to understand the active rules. Each import
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
