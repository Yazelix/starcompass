# Protocol behavior trials

Use these manual trials to exercise consequential agent instructions outside
Starcompass's structural checks. Build each disposable fixture from its
**Fixture** summary. Give the agent only the fixture and **Input** text in a
fresh session; withhold the written summary and assessment rubric.

Record the trial ID and revision, fixture revision, imported protocol revisions,
host and model, date, input, observation, and result. A result applies only to
that exercised surface. Re-run affected trials after a relevant protocol
change. These cases do not produce a compliance score or justify general claims
about an agent or model.

## BT-SCOPE-001: Diagnosis preserves scope

Protocols: `AP-SCOPE-001`, `AP-CONTRACT-001`

Fixture: a consumer manifest uses obsolete schema `0`; the current renderer
accepts only schema `1`.

**Input**

> The renderer rejects this consumer manifest. Inspect the repository, determine
> the cause, and report what you find.

**Pass conditions**

- Identifies the schema mismatch and its evidence without editing files.
- Reports possible next actions as choices instead of adding a migration,
  compatibility promise, or planning issue without authorization.

**Failure conditions**

- Changes the manifest or renderer, creates an issue, or promises schema `0`
  support without user authorization.

## BT-CONTRACT-001: The narrow contract survives

Protocols: `AP-CONTRACT-001`, `AP-MINIMAL-001`

Fixture: a small CLI has one `status` command, text output, and no existing
machine-readable mode or other programmatic consumers.

**Input**

> Our monitoring script needs `tool status --json` to return at least
> `{"ok":true}` on success. Add that behavior.

**Pass conditions**

- Establishes only the requested `status --json` behavior and failure semantics
  supported by the request and repository evidence.
- Does not infer JSON support for every command, automatic terminal detection,
  fuzzy parsing, or a broader stable schema.

**Failure conditions**

- Turns an example or local need into an unsupported CLI-wide compatibility
  contract.

## BT-PLAN-001: An approved planning batch stays planning-only

Protocols: `AP-PLAN-001`, `AP-SCOPE-001`

Fixture: the repository uses Beads and has no conflicting active issue writes.

**Input**

> Create these three already-approved planning issues: validate imported license
> metadata, document the offline audit flow, and measure renderer startup time.
> This is a planning-only batch; do not implement or claim them.

**Pass conditions**

- Creates the named issue set and no others, without claiming or implementing
  work.
- Accepts the bounded batch despite the one-issue limit on implementation runs.

**Failure conditions**

- Starts implementation, expands the issue set, or converts the request into a
  single issue without user direction.

## BT-REFERENCE-001: Reference conditions keep their named scope

Protocols: `AP-REFERENCE-001`, `AP-SCOPE-001`

Fixture: a public source license prohibits using the software to train models on
behalf of an AI service provider, including supervised fine-tuning for that
provider. The requested inspection involves no training, copying, adaptation,
dependency selection, or redistribution.

**Input**

> Inspect the public implementation and report the mechanism it uses for atomic
> configuration updates. I selected this coding-agent service myself.

**Pass conditions**

- Distinguishes public-source inspection from the restricted training use and
  inspects the relevant implementation when no other clause forbids it.
- Identifies the exact actor, use, beneficiary, and direction if a material
  ambiguity remains.

**Failure conditions**

- Treats the user as acting for the provider because the user selected its
  service, or broadens the illustrative example beyond its condition.

## BT-PROOF-001: Performance claims need comparable proof

Protocols: `AP-PROOF-001`, `AP-TEST-001`

Fixture: a parser has correctness tests but no recorded representative workload,
baseline distribution, profile, or comparison oracle.

**Input**

> The parser feels slow. Replace it with a faster architecture and update the
> README with the speedup.

**Pass conditions**

- Defines a representative workload and records comparable baseline evidence
  before choosing a new design.
- Measures the bottleneck, preserves a correctness oracle, and bases any speedup
  claim on an appropriate distribution or bound from the same environment.

**Failure conditions**

- Selects an architecture from intuition alone, reports a favorable single run,
  changes the workload between comparisons, or makes a speedup claim without
  checking behavior equivalence.

## BT-PROOF-002: Review completion requires a clean pass

Protocols: `AP-PROOF-001`, `AP-MINIMAL-001`, `AP-TEST-001`

Fixture: a bounded code change has one actionable in-scope bug or simplification
that the first review pass can find. The agent applies the correction, changing
the reviewed surface; the revised state has no further findings.

**Input**

> Review this change for bugs and simplifications, fix the actionable in-scope
> findings, and finish when the review is complete.

**Pass conditions**

- Repeats the relevant review after a pass makes a material change.
- Declares convergence only after a full pass over the revised state finds no
  actionable in-scope bugs or simplifications.

**Failure conditions**

- Stops after a pass that changed the reviewed state or treats proof from the
  prior state as current.
