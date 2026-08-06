# Agent Guidelines

This file is self-contained. Canonical protocol text was rendered into it;
the source repository is needed only to update or verify the import.
Do not edit this generated file directly. Edit `.agent-protocols.local.md`
or `.agent-protocols.exceptions.json`, then render from the pinned source.

## Protocol import record

- Source: `https://github.com/luccahuguet/starcompass`
- Source commit: `59b208c8a15a35ba2c32070557a20583b8aa4736`
- Profiles: `greenfield`
- Manifest: `.agent-protocols.json` (schema 1)

| Protocol | Version | SHA-256 |
| --- | ---: | --- |
| `AP-SCOPE-001` | 2 | `f66415c50cfedcb850725391f54873dae5bd067189fcf63fc2cf7c2efd3bd489` |
| `AP-CONTRACT-001` | 3 | `4b092f0fb04d2d2064d0004a711dbfebb1a18ad8de7c5ac42237e136d4a934ca` |
| `AP-REFERENCE-001` | 2 | `ecb28af3796a9964dd98c037463a33b7444d9c39a0365783fb0e9ae9fc007b9c` |
| `AP-MINIMAL-001` | 3 | `719ca1e060ffbc969ee96e3d8c3c6242be5bc52b34ba2ec5988f80ab841ef31c` |
| `AP-DEPENDENCY-001` | 1 | `a389ff9054708574c52ec5e5dd7fc3e2d13b125d2218c70062f50a86761981ca` |
| `AP-OWNERSHIP-001` | 1 | `bdc09117f79b0d8dbe78e2dd8673a2463398aa31880fe27209fd6cc47f58bbf7` |
| `AP-TEST-001` | 2 | `80ed9a1d3038c704a26ecf03525a85974ab8de3ddafa4aab2c2ce56e947bf00b` |
| `AP-PROOF-001` | 3 | `6d986bb67f310d9ff95fede9c354f6de37716c2b049db070e4cddca0f1357e86` |
| `AP-PLAN-001` | 6 | `04b96c47978d49d2e6584f89626bca42b6cb39df537fa3e5abd6da0bdc2edfda` |
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

- Treat a request to inspect, audit, diagnose, explain, or recommend as authority
  only for that non-mutating result. Do not edit implementation, publish
  external state, or write durable planning state unless the user also chooses
  that action.
- Treat existing inputs and artifacts as user-owned. Replace or delete an exact
  target only when the requested outcome requires it; otherwise preserve it and
  use a distinct result when output must be written.
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

- Among contracts that fit the user request and available evidence, choose the one
  with the fewest unsupported guarantees or restrictions. Leave reasonable
  future behavior unspecified unless the request or evidence requires a
  commitment.
- Treat user-facing documentation, help, examples, and configuration
  descriptions as contract surfaces. Keep their commands, paths, flags,
  defaults, and availability aligned with their current owner; label planned,
  partial, or gated behavior instead of presenting it as current.
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
- Apply source-license wording to the actors, uses, and conditions it actually
  names. Do not infer that an independent user or project acts on behalf of,
  for the benefit of, or under the direction of an agent or tool provider
  merely because the user selected that provider's service. Examples
  introduced by words such as “including” remain scoped by the condition they
  illustrate.
- Distinguish inspecting public source for ideas from copying, adapting,
  redistributing, selecting a dependency, or incorporating the source. A
  restriction on one of those actions does not silently erase required source
  inspection when the requested research itself remains permitted.
- If license interpretation would exclude required evidence, identify the
  exact clause, actor, beneficiary, direction, and requested use. Resolve a
  material ambiguity with the user instead of broadening the restriction by
  association or substituting reputation and secondary summaries for source.

Reference review is a decision gate, not a requirement to copy the reference.

### AP-MINIMAL-001 — Minimum sufficient implementation

Use the accepted contract and source evidence to identify the correct owner,
affected flow, and necessary boundaries before minimizing code. Then use the
first option that fully satisfies those constraints:

1. Make no change when the required behavior already exists.
2. Reuse an existing correct owner, helper, or pattern in the repository.
3. Use the standard library or a native platform capability.
4. Use an already accepted dependency that owns the behavior.
5. Implement the minimum local code that is correct and maintainable.

Evaluate minimality across the accepted system and its lifecycle, not only the
current patch. Lines, files, dependencies, and patch size are evidence, not
objectives. Include duplicated truth, cross-owner coordination, coupling,
migration and removal, portability, operations, and proof cost.

Durable agent context is part of that lifecycle cost. Put instructions at the
narrowest scope that needs them. Keep durable wording for non-obvious
constraints and reusable workflows that change behavior; do not duplicate
policy already owned elsewhere or restate generic agent capability. Load
detailed references only when the task needs them. Context reduction must not
hide requirements needed to act safely or correctly.

A smaller patch is not minimal when it preserves a known misplaced or duplicate
owner, patches a symptom below its shared cause, bypasses an accepted boundary,
or increases downstream coordination. A necessary root-cause or ownership
correction may be locally larger. Use patch size only as a tie-breaker among
shapes that satisfy the same contract, ownership, lifecycle, and proof duties.

Prefer deletion over addition, direct ownership over adapters, and fewer files
over scaffolding. Minimalism must not remove required behavior, trust-boundary
validation, data-loss protection, security, accessibility, or the cheapest
runnable check for non-trivial logic.

Ponytail is the adopted agent-side aid for this discipline when the host
supports it; it is a fallible implementation bias, not an architectural
authority. Apply its YAGNI, shortest-diff, standard-library, native, and
one-line heuristics only after the constraints above. Repository contracts,
ownership, evidence, dependency decisions, safety, accessibility, portability,
proof obligations, and explicit user choices take precedence. Do not invoke
Ponytail to avoid a necessary architectural or root-cause change.

Use the upstream project directly rather than copying its rules or adapters.
The reviewed source is
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
- For consequential changes to agent instructions, prompts, or skills, use
  representative task trials with enough isolation that the expected
  conclusion is not supplied by the test context.
- Treat schema, rendering, discovery, and other structural checks as proof of
  those structures, not of agent behavior or compliance.
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
- For a performance claim, compare the same representative workload and
  environment against a recorded baseline, measure the bottleneck before
  changing the design, retain a correctness oracle, and report a representative
  distribution or bound rather than only a favorable sample.
- When completion requires review or simplification, a pass that produces a
  material change invalidates the prior completion assessment. Repeat the
  relevant pass against the revised state and declare completion only after a
  full pass finds no actionable in-scope bugs or simplifications.
- Preserve important negative results; they constrain the next valid design.

### AP-PLAN-001 — Durable planning state

Keep the outcomes and constraints that later work needs in the project's
durable planning system or canonical documentation. An issue represents a
chosen goal, decision, material defect, or schedulable follow-up. Review and
implementation methods belong to that issue.

An agent run is one uninterrupted execution ending when control returns to the
user, including automatic continuations. A run may inspect any planning state
read-only. A run that writes planning state or implementation may use one of two
issue-work shapes:

1. Own at most one issue. The run may create, claim, update, implement, or close
   that issue.
2. Use a bounded planning-only batch with explicit user authorization over a
   named or accepted issue set. The run may create, update, or close only that
   set; it may not claim an issue or make implementation edits.

Do not combine these shapes in one run.

Required practice:

- Bind an implementation run to its one owning issue before its first issue
  write or implementation edit.
- After the owning implementation issue is complete, blocked, or handed off,
  stop and return control to the user. Do not begin another issue, including
  work newly unblocked by the completion.
- Leave unapproved follow-up findings as reported findings. A planning-only
  batch may include only its named or accepted issue set.
- After review, fresh-eyes, simplification, or verification of an owning issue,
  update its editable fields to describe the accepted state instead of pass
  chronology.
- Create a separate issue only for a material finding outside the prior owning
  scope or one worth scheduling on its own. Outside an authorized planning-only
  batch, defer creation to a later run. Name it after the outcome or finding.
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
- `src/starcompass.rs` is the sole owner of validation, rendering, and drift
  detection; `src/main.rs` owns only process I/O and exit status.
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
