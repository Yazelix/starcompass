# Changelog

## Unreleased

### Added

- Twenty-two stable-ID, individually versioned canonical protocols.
- Starcompass project and repository identity with stable protocol and
  consumer-format names.
- Minimum-sufficient-implementation discipline with Ponytail as its adopted,
  upstream-owned agent integration.
- Installed-artifact and fresh-session delivery proof plus exact-revision
  channel promotion.
- Composable `greenfield`, `fork`, `orchestrator`, `release`, and
  `terminal-engine` profiles.
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
- Generated `AGENTS.md` files identify their editable consumer-owned inputs.
- Exact-pinned `greenfield` self-adoption through the ordinary consumer format
  and a two-revision bootstrap sequence for canonical source changes.
- Source-independent `check-consumer` validation for manifest, overlay,
  exception, generated import-record, and repository-local-rule agreement.
- Public source distribution under either the MIT or Apache-2.0 license, with
  repository and package-discovery metadata.
- Directory-local instruction files: `render --nested DIR` generates
  `DIR/AGENTS.md` from that directory's overlay, records it in the manifest,
  and lists it in the root import record. Both checks reject a drifted,
  edited, or escaping directory file, and any root-to-directory chain above
  Codex's 32 KiB default.
- `AP-SESSION-001` in the `greenfield` and `fork` profiles: treat context
  compaction as lossy, record user decisions when they are made, reread the
  active issue after compaction, and restart sessions at clean breaks rather
  than compacting repeatedly.

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

### Changed

- The canonical source repository is `Yazelix/starcompass`; existing imports
  keep their exact commit identity while recording the organization-owned URL.
- Compact wording keeps automatic continuations inside one run, binds
  implementation to its issue, specifies fork and secret behavior, retains
  deterministic failure tests, bounds slow-consumer impact, and separates
  corrections after external reliance from unpublished amendments.
- Canonical protocol wording retains non-obvious model weaknesses, high-cost
  boundaries, and repository workflows while removing generic or repeated
  guidance. Every individual profile fits a 12 KiB canonical/import budget, and
  the measured Orbit profile combination preserves 2 KiB of host-instruction
  headroom.
- `AP-SCOPE-001` keeps inspection, audit, diagnosis, explanation, and advice
  non-mutating unless the user also chooses a change, and preserves user inputs
  and artifacts from implicit replacement or deletion. Since version 4, every
  choice offered to the user carries a recommended answer and its reason;
  tighter wording keeps the orchestrator profile within its 12 KiB budget.
- `AP-CONTRACT-001` selects the contract with the fewest unsupported guarantees
  or restrictions, leaves reasonable future behavior unspecified, and treats
  current user-facing documentation as a contract surface.
- `AP-MINIMAL-001` evaluates minimality after contract and ownership selection,
  across total system and lifecycle cost. Ponytail remains a fallible runtime
  bias and cannot justify preserving a wrong owner or merely minimizing the
  current diff. Durable agent context uses the narrowest useful scope, avoids
  duplicate or generic wording, and loads details only when needed.
- `AP-TEST-001` uses representative isolated task trials for consequential
  agent-instruction changes and keeps structural validation distinct from
  behavioral compliance.
- `AP-PROOF-001` requires comparable baselines, bottleneck evidence, correctness
  oracles, and representative results before making performance claims. Agents
  may claim review or simplification convergence only after a full pass over the
  revised state finds no actionable in-scope bugs or simplifications. Durable
  evidence excludes disposable paths and conditionally retains raw logs, while
  unchanged source proof survives focused metadata, documentation, and planning
  follow-ups. Proof targets the highest sufficient changed boundary and expands
  only for affected ownership, failure, trust, delivery, or promotion surfaces.
- `AP-PLAN-001` permits one owning implementation issue or a user-authorized
  bounded planning-only batch, prevents a run from combining both shapes, and
  keeps execution and reference-gate comments delta-only. Acceptance records
  observable outcomes and cheapest falsifiers instead of full command catalogs.
- `AP-DELIVERY-001` lets one outer composed proof cover an unchanged embedded
  artifact only for its recorded exact identity and exercised behavior.
- `AP-REFERENCE-001` applies source-license conditions to their actual actors
  and uses, distinguishes inspection from reuse, and prevents provider
  association from replacing required source evidence.
- `AP-PLAN-001` makes issue fields mutable current state and reserves append-only
  history for evidence. `AP-GIT-001` folds same-unit corrections into the
  current task's unpublished commit; pushed or externally pinned revisions get
  follow-up commits.
- Source-backed `check` composes the consumer-local proof with exact source HEAD,
  catalog, protocol digest, canonical prose, and complete generated-file proof.
- Exact public Git revisions are the initial distribution surface; crates.io,
  prebuilt binaries, and hosted CI remain separate release gates.
