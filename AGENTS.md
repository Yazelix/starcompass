# Agent Guidelines

This file is self-contained. Canonical protocol text was rendered into it;
the source repository is needed only to update or verify the import.
Do not edit this generated file directly. Edit `.agent-protocols.local.md`
or `.agent-protocols.exceptions.json`, then render from the pinned source.

## Protocol import record

- Source: `https://github.com/Yazelix/starcompass`
- Source commit: `a65f9149d79c1eec47705ccdedef75a9e57d2bc0`
- Profiles: `greenfield`
- Manifest: `.agent-protocols.json` (schema 1)

| Protocol | Version | SHA-256 |
| --- | ---: | --- |
| `AP-SCOPE-001` | 4 | `8d84dc6fca6097b2055eb3ca8467565709e4c474a18c3030ae3064cc612bdfab` |
| `AP-CONTRACT-001` | 4 | `cdf2e5d69cefd34ceeeaa6b7511b21f921e2be50c17d3c01b85120b20234cff7` |
| `AP-REFERENCE-001` | 4 | `906bcda869549979647d20e957ff48909fa5f36024f584be6657bdc058d58e72` |
| `AP-MINIMAL-001` | 5 | `a707f9f27a4ca42c4e78738830fdafd51d1c66acf80541590b9e32e016ddbe37` |
| `AP-DEPENDENCY-001` | 2 | `ded49add538b82de0a9c522bc8a34720f4ebbb47f39fc2f7ddb25e7aad1700d3` |
| `AP-OWNERSHIP-001` | 2 | `d218a4e0625b659ec366284110bdfce02bd66cb229ab6ba07f2317092ea13053` |
| `AP-TEST-001` | 4 | `1c1c5eafa1f611fd7575ecf00b32a2068a477614c8be78ac4efa41630ec8b5ad` |
| `AP-PROOF-001` | 11 | `310e1f798c3334e08402f9e42f447ae9352d171fc92888afb141c6b228f84e56` |
| `AP-PLAN-001` | 12 | `5ee8deccb4b3c61ddd4a52b8ed6f69ebd30e24b65ace867871a89c75ef02a984` |
| `AP-SESSION-001` | 4 | `51c15e878070caaf372721c2eb083354f5f63492f5c4e694cf31aaefcefa53a7` |
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
representative tasks without supplying the expected conclusion when the user
authorizes a separate agent; otherwise validate in the active context and
report agent behavior as unproven. Structural checks prove structure, not
agent behavior. Test observable effects; delete duplicate proof,
implementation trivia, and scaffolding. Guard absence only when absence is a
security, licensing, size, ownership, or known-regression contract.

### AP-PROOF-001 — Explicit proof lifecycle

Proof records the revision, command or observation, environment, and result
with its surface. Never widen or reuse proof after a material change.

Use the highest boundary that exercises the changed contract. Add broader or
lower suites only for a changed owner, failure, trust boundary, distinct
delivery, or promotion, not because a suite exists, an issue lists it, or it
ran before. Exact downstream proof covers lower checks only for the identity
and behavior it exercised.

Run a complete suite once per unchanged candidate and environment; metadata,
docs, and planning follow-ups reuse it. Commands are evidence, not a checklist.
Keep logs only when reproduction is costly, disputed, or performance-sensitive,
or a summary loses detail; a temporary path is not proof. Unattended runs and
waits close unused stdin, time out, check progress early, and never match
themselves as the target; stalls fail.

Finish every change to code, tests, or scripts with review by a fresh agent
that did not write it, since authors miss their own leftovers; in Claude Code,
use the converge skill. Docs-only and planning-only work skips it. A pass that
changes anything requires another; convergence needs a full clean pass. Nits
never block it, and text-only fixes restart a review at most twice per slice.

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
the issue tool (beads_rust `br` when available), never its storage.

### AP-SESSION-001 — Session continuity across compaction

Compaction is lossy; avoid it. Write each user decision to its durable owner
when it is made. Delegate work whose raw output matters only as a conclusion,
such as broad searches, long logs and checks, or research, to a fresh
subagent; subagents are authorized unless the user or local rules say
otherwise.

Budget absolute context tokens, not window share; if the host reports none,
estimate high:

- From 400k, add no scope or new steps: close the issue if it can close by
  600k, else bring work in progress to a verified state and hand off.
- Past 600k, continue only to reach an imminent verified state or keep live
  state that cannot be written down; record why on the issue.
- Hand off before the host would compact. After any compaction, reread the
  issue and hand off.

A handoff is an issue comment: state, next step, and open decisions with
reasons. The next session primes, e.g. "prime, then continue <issue> from its
handoff".

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

Do not add a protocol, profile, enforcement skill, compatibility surface, or
planning bead until the user has chosen that direction.

## Ownership Model

- `protocols/*.md` owns protocol wording and metadata.
- `protocols/index.json` is generated by `starcompass index`; never edit it
  directly.
- `profiles/*.json` owns ordered protocol composition.
- `src/starcompass.rs` and its `src/starcompass/` modules are the sole owner of
  validation, rendering, and drift detection; each module's `//!` header names
  the one concern it owns. `src/main.rs` owns only process I/O and exit status.
- `skills/` holds the owner's agent skills and `skills/install.sh`. Skills
  reference protocols by ID instead of copying their text.
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

Prefer dependency-free code and transparent formats. Do not add a parser,
framework, template engine, or package merely to save a small amount of local
code. Do not duplicate canonical protocol text in profiles, skills, or docs.

## Testing

Tests should prove stable IDs and versions, invalid registry/profile rejection,
deterministic rendering, exception recording, or drift detection.

Use temporary directories for generated consumer fixtures. Do not commit
generated test output unless it is a deliberate compatibility fixture.

## Beads

Serialize `br` writes. Run `br sync --flush-only` before committing Beads
changes.

## Git Workflow

Work directly on `edge`. Keep commits focused.

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

For the full repository, run:

```sh
cargo fmt --check
cargo check --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
claude plugin validate --strict skills
git diff --check
```

`claude plugin validate --strict` checks every skill's frontmatter the way
Claude Code loads it, but it accepts malformed YAML. Where Codex is installed,
its stricter checker also parses the YAML; it prints any skill that fails:

```sh
for skill in skills/*/; do
  python3 ~/.codex/skills/.system/skill-creator/scripts/quick_validate.py "$skill" >/dev/null || echo "invalid: $skill"
done
```

Do not add hosted CI until the user explicitly chooses its cost and trust
model.
