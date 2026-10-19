# Changelog

## Unreleased

### Added

- Twenty-five stable-ID, individually versioned canonical protocols.
- Starcompass project and repository identity with stable protocol and
  consumer-format names.
- Minimum-sufficient-implementation discipline.
- Installed-artifact and fresh-session delivery proof plus exact-revision
  channel promotion.
- Composable `regular` and `surgical` posture profiles and a `release`
  add-on.
- Typed Rust catalog indexing, self-contained `AGENTS.md` rendering, pinned
  structural drift detection, and atomic cross-platform output replacement.
- History checks for permanent protocol IDs and mandatory version increments.
- Explicit irreversible protocol retirement that preserves indexed history
  while removing obsolete rules from all profiles.
- Explicit local overlay and approved-exception inputs and import records.
- Focused contract tests for composition, validation, exception handling, and
  drift.
- Host-independent protocol behavior trials with isolated agent inputs and
  separate assessment rubrics.
- Protocol-only apply and audit Codex skills.
- The owner's general agent skills, imported from `luccahuguet/skills` with
  their history, and `skills/install.sh`, which links every skill into Claude
  Code and Codex.
- Generated `AGENTS.md` files identify their editable consumer-owned inputs.
- Exact-pinned self-adoption through the ordinary consumer format and a
  two-revision bootstrap sequence for canonical source changes.
- Source-independent `check-consumer` validation for manifest, overlay,
  exception, generated import-record, and repository-local-rule agreement.
- Public source distribution under either the MIT or Apache-2.0 license, with
  repository and package-discovery metadata.
- Directory-local instruction files: `render --nested DIR` generates
  `DIR/AGENTS.md` from that directory's overlay, records it in the manifest,
  and lists it in the root import record. Both checks reject a drifted,
  edited, or escaping directory file, and any root-to-directory chain above
  Codex's 32 KiB default.
- Optional separate protocol file: `render --protocol-file NAME` writes the
  canonical protocol text to a consumer file that the root `AGENTS.md` names;
  `AGENTS.md` keeps the import record and local rules and tells agents to read
  the file. The manifest records it; `check` compares it with the pinned
  source, and `check-consumer` checks its header and rejects inlined protocol
  text, an escaping path, or an auto-loaded `AGENTS.md` or `AGENTS.override.md`
  name.
- `AP-SESSION-001` in the `greenfield` and `fork` profiles: treat context
  compaction as lossy, record user decisions when they are made, reread the
  active issue after compaction, and restart sessions at clean breaks rather
  than compacting repeatedly. Since version 2, agents budget absolute context
  tokens instead of window share and hand off through an issue comment the
  next session primes from. Version 3 moves the wind-down to 400k-600k: agents
  add no scope or new steps from 400k, continue past 600k only to reach an
  imminent verified state, and hand off before the host would compact or after
  any compaction. It drops the 100k delegation and fresh-issue marks and the
  180k and 250k marks; 2026 evidence shows gradual degradation that becomes
  clear past about 256k rather than a cliff near 100k, and each handoff costs a
  re-prime. Version 4 delegates work whose raw output matters only as a
  conclusion, such as broad searches, long logs and checks, or research, to a
  fresh subagent, and authorizes subagents unless the user or local rules say
  otherwise. The `greenfield + terminal-engine` canonical/import budget rose
  from 13,100 to 13,500 bytes to fit version 2.

### Fixed

- Preserve leading Markdown indentation in canonical protocol bodies and local
  overlays, and normalize trailing review-condition whitespace before adding
  punctuation.
- Let mandatory profile protocols follow the same retirement lifecycle, and
  reject protocol IDs with empty `AREA` segments.
- Reject explicit null review conditions or protocol statuses, year-zero
  exception dates, empty or option-like Git base revisions, and duplicate IDs
  in historical indexes.
- Restore fresh `0644` generated files on Unix and report catalog entry errors
  instead of silently omitting them.
- Accept the full unsigned protocol-version range and recognize global help
  regardless of whether `--root` precedes it.
- Reject symlinked or special catalog sources before external bytes can be
  mistaken for pinned content, and resolve render-path identities so aliases
  cannot overwrite inputs or each other.
- Reject catalog profile names and import source or profile values that can
  break inline-code framing, and fold exception line endings without admitting
  carriage-return Markdown blocks.
- Reject repeated singular CLI options before command I/O and avoid duplicating
  terminal punctuation in exception review conditions.
- Require `render` to receive the exception authoring file used by both check
  commands, including an empty document when the consumer has no exceptions.
- Disable Cargo library auto-discovery so the package exposes only the selected
  binary interface.
- Reject `--nested` directories and protocol-file paths that pass through a
  symlink below the consumer, in `render` before any consumer file is read or
  written and in both checks before those files are read.

### Changed

- The canonical source repository is `Yazelix/starcompass`; existing imports
  keep their exact commit identity while recording the organization-owned URL.
- Compact wording keeps automatic continuations inside one run, binds
  implementation to its issue, specifies fork and secret behavior, retains
  deterministic failure tests, bounds slow-consumer impact, and separates
  corrections after external reliance from unpublished amendments.
- Canonical protocol wording retains non-obvious model weaknesses, high-cost
  boundaries, and repository workflows while removing generic or repeated
  guidance. Every individual profile fits a 12.5 KiB canonical/import budget, and
  the measured Orbit profile combination preserves about 1.5 KiB of
  host-instruction headroom.
- `AP-SCOPE-001` keeps inspection, audit, diagnosis, explanation, and advice
  non-mutating unless the user also chooses a change, and preserves user inputs
  and artifacts from implicit replacement or deletion. Since version 4, every
  choice offered to the user carries a recommended answer and its reason;
  tighter wording keeps the orchestrator profile within its budget. Version 5
  leaves compatibility promises and public surfaces to `AP-CONTRACT-001`'s
  fewest unsupported guarantees and keeps features, migrations, repositories,
  and planning items.
- `AP-CONTRACT-001` selects the contract with the fewest unsupported guarantees
  or restrictions, leaves reasonable future behavior unspecified, and treats
  current user-facing documentation as a contract surface. Version 5 keeps the
  cheapest falsifying check and leaves the one source-of-truth owner to
  `AP-OWNERSHIP-001` and the smallest complete vertical slice to
  `AP-MINIMAL-001`.
- `AP-MINIMAL-001` evaluates minimality after contract and ownership selection,
  across total system and lifecycle cost, and rejects a patch that merely
  minimizes the current diff. Durable agent context uses
  the narrowest useful scope, avoids duplicate or generic wording, and loads
  details only when needed. Version 5 removes the Ponytail paragraph, reversing
  its adoption in every profile: it put directives in every profile for a tool
  most consumers lack. Repositories that use Ponytail add the README's overlay
  text instead. Version 6 leaves duplicate truth, wrong or duplicate owners, and
  direct ownership to `AP-OWNERSHIP-001`; patch size still decides only between
  equally correct system shapes. Together with the version 5 overlap removals
  in `AP-SCOPE-001` and `AP-CONTRACT-001`, this shrinks `greenfield` by 173
  bytes. Version 7 owns the code-level anti-bloat rules, each with its reason:
  build only for current callers, not forecasts, and abstract only for a second
  caller; update every caller, test, fixture, configuration, and export a change
  breaks and delete what it orphans, in place of a general preference for
  deletion; report unrelated dead code as out of scope; and handle empty,
  malformed, and boundary input. The README no longer offers Ponytail overlay
  text, and Starcompass no longer recommends Ponytail. Every profile grows by
  348 bytes.
- `AP-TEST-001` uses representative isolated task trials for consequential
  agent-instruction changes and keeps structural validation distinct from
  behavioral compliance. Since version 4, trials run when the user authorizes
  a separate agent; otherwise the active agent validates in its own context
  and reports agent behavior as unproven.
- `AP-PROOF-001` lets agents claim review or simplification convergence only after a full pass over the
  revised state finds no actionable in-scope bugs or simplifications. Durable
  evidence excludes disposable paths and conditionally retains raw logs, while
  unchanged source proof survives focused metadata, documentation, and planning
  follow-ups. Proof targets the highest sufficient changed boundary and expands
  only for affected ownership, failure, trust, delivery, or promotion surfaces.
  Since version 7, unattended checks and reviews close unused stdin, have a
  timeout, and get an early progress check, and a stalled run counts as a
  failure. Since version 8, the same applies to waits on a run, and a wait never
  matches itself as its target, so the waiter cannot keep its own wait open.
  The `greenfield + terminal-engine` canonical/import budget rises from 13,030
  to 13,100 bytes to fit it.
  Since version 9, nits never block convergence, and text-only fixes restart a
  review at most twice per slice. With `AP-TEST-001` version 4, the
  `greenfield + terminal-engine` canonical/import budget rises from 13,500 to
  13,700 bytes to fit both.
  Version 10 states its rules in plain sentences, drops the proof-phase list,
  which no protocol relied on, and moves performance claims to `AP-PERF-001`,
  shrinking `greenfield` by 150 bytes.
  Since version 11, every change to code, tests, or scripts finishes through
  review by a fresh agent that did not write it, through the `converge` skill
  in Claude Code; docs-only and planning-only work is exempt.
- `AP-PERF-001`, in the `terminal-engine` and `release` profiles, requires a
  measured bottleneck, a fixed workload and environment, a baseline, a
  correctness oracle, and a distribution or bound before a performance claim,
  and keeps the negative results that constrain it. To fit it, the
  single-profile canonical/import budget rises from 12,800 to 12,850 bytes and
  the `greenfield + terminal-engine` budget from 13,700 to 13,900 bytes.
  Version 2 also covers redesigning for speed: fix a representative workload,
  record a baseline, and measure where the time goes before choosing a design.
- `AP-PLAN-001` permits one owning implementation issue or a user-authorized
  bounded planning-only batch, prevents a run from combining both shapes, and
  keeps execution and reference-gate comments delta-only. Acceptance records
  observable outcomes and cheapest falsifiers instead of full command catalogs.
- `AP-DELIVERY-001` lets one outer composed proof cover an unchanged embedded
  artifact only for its recorded exact identity and exercised behavior.
- `AP-REFERENCE-001` requires direct evidence before a consequential code
  shape. Version 4 moves license interpretation to `AP-LICENSE-001`, shrinking
  `greenfield` by 535 bytes.
- `AP-LICENSE-001`, in the `fork` profile, applies source-license conditions to
  their actual actors and uses, distinguishes inspection from reuse, and
  prevents provider association from replacing required source evidence.
- The `greenfield` profile drops `AP-CI-001`, shrinking by 529 bytes: a small
  new project rarely has hosted CI. The `fork`, `orchestrator`, `release`, and
  `terminal-engine` profiles keep it, so `greenfield + terminal-engine` is
  unchanged.
- The `greenfield` profile is renamed `regular`: it serves every project, not
  only new ones, and its protocols are unchanged. There is no `greenfield`
  alias. Existing imports still verify from their pinned source commit, and
  consumers pass `--profile regular` on their next upgrade.
- `AP-SURGICAL-001` and the `surgical` profile serve soft forks and projects in
  maintenance. `surgical` is `regular` plus `AP-LICENSE-001` and
  `AP-SURGICAL-001`, which treats preserved code as owned elsewhere: prefer
  configuration and extension points, then isolated patches, over rewrites;
  keep changes separable; leave unrelated preserved code alone. Where it
  differs from `AP-MINIMAL-001`, it wins: fix a shared cause in preserved code
  locally and report it. With an upstream, record its baseline revision and
  each patch's reason and removal condition. The single-profile
  canonical/import budget rises from 12,850 to 12,900 bytes to fit it.
- The `fork`, `orchestrator`, and `terminal-engine` profiles are removed, and
  `AP-ORCHESTRATOR-001`, `AP-FRONTIER-001`, `AP-TERMINAL-001`,
  `AP-FAILURE-001`, `AP-PORTABILITY-001`, `AP-FORK-001`, and `AP-FORK-002` are
  retired; their IDs stay reserved. `AP-SURGICAL-001` carries the fork posture
  and upstream bookkeeping, and terminal, failure, and portability rules belong
  in the overlays of consumers with those components. `release` becomes an
  add-on of `AP-SCOPE-001`, `AP-PERF-001`, `AP-CI-001`, `AP-DELIVERY-001`,
  `AP-PROMOTION-001`, and `AP-EXCEPTION-001` that renders after a posture.
  The combined budget case becomes `surgical + release`, provisionally capped
  at 15,300 bytes. Existing imports still verify from their pinned source
  commit. On their next upgrade, consumers pick a posture and add `release`
  only if they promote through channels: a project, or a hard fork that owns
  its code, uses `regular`; a soft fork or a project in maintenance uses
  `surgical`; former `orchestrator` and `terminal-engine` consumers use
  `regular` and keep component rules in their overlay. For the owner's
  consumers: eon, starcompass, and machines_vs_aliens use `regular`; nova uses
  `regular`, plus `release` if it keeps channel promotion; nova-rio and
  nova-helix use `surgical`.
- Each shipped profile has its own canonical/import budget, about 200 bytes
  above its measured size, in place of the shared 12,900-byte single-profile
  budget and the provisional 15,300-byte combined one: `regular` 11,400 bytes
  (11,964 as `greenfield` before the trim, 11,172 after), `surgical` 13,100
  (12,888), `release` 4,400 (12,451 as a full profile, 4,194 as an add-on),
  and `surgical + release` 15,500 (15,270; the previous largest combination,
  `greenfield + terminal-engine`, measured 13,488). A shared cap would let
  space trimmed from `regular` refill under `surgical`'s cap.
- `AP-PLAN-001` makes issue fields mutable current state and reserves append-only
  history for evidence. Since version 12, the issue tool is beads_rust (`br`)
  when it is available. `AP-GIT-001` folds same-unit corrections into the
  current task's unpublished commit; pushed or externally pinned revisions get
  follow-up commits.
- Source-backed `check` composes the consumer-local proof with exact source HEAD,
  catalog, protocol digest, canonical prose, and complete generated-file proof.
- Exact public Git revisions are the initial distribution surface; crates.io,
  prebuilt binaries, and hosted CI remain separate release gates.
- The checker is split into private, concern-owned modules under
  `src/starcompass/` (catalog, manifest, render, check, and command dispatch)
  with unit tests beside the code they cover. Commands, generated output, and
  error messages are unchanged.
