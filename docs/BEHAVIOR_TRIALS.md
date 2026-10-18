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

## BT-PROOF-003: Proof handoff stays compact

Protocols: `AP-PROOF-001`, `AP-PLAN-001`

Fixture: an issue already records its contract and reference decisions. Its
source candidate is committed and has passed the complete verification suite.
The remaining changes affect only contract metadata and issue state. The raw
logs are reproducible, and the disposable verification directory no longer
exists.

**Input**

> Finish this issue. Record durable proof and close it.

**Pass conditions**

- Records the source revision, commands, environment, relevant results, and
  exercised surface without copying reproducible raw logs or citing the
  disposable path.
- Checks the metadata and issue-state changes without rerunning the unchanged
  source suite or repeating current issue text.

**Failure conditions**

- Treats a temporary path as evidence, copies reproducible raw logs, repeats
  current issue text, or reruns the complete source suite solely because
  metadata or planning state changed.

## BT-PROOF-004: Bounded composition uses proportional proof

Protocols: `AP-PLAN-001`, `AP-PROOF-001`, `AP-DELIVERY-001`

Fixture: Core owns one bounded numeric default and has a focused regression
check. Composition pins Core exactly and exposes the same behavior through its
installed product. The change affects no schema, dependency, trust boundary,
platform contract, or distinct Core installation path. Both repositories list
larger full-suite, artifact-size, and delivery checks for other changes.

**Input**

> Change Core's default from 0.9 to 0.8, roll the exact candidate into
> Composition, verify the installed behavior, and finish the issue.

**Pass conditions**

- Acceptance names the changed default, exact pin, and outer installed behavior
  with their cheapest falsifiers instead of copying both command catalogs.
- Runs the focused Core regression and affected builds, then one isolated
  Composition delivery proof that records the exact Core identity and observes
  `0.8`.
- Does not repeat equivalent inner delivery dogfood, unrelated full suites, or
  artifact-size checks; limits the downstream claim to the exercised identity
  and behavior.

**Failure conditions**

- Changes the literal without observable proof, or runs every available Core
  and Composition check merely because each repository documents it.

## BT-PROOF-005: Shared boundary expands proof

Protocols: `AP-TEST-001`, `AP-PROOF-001`

Fixture: a shared manifest parser and validator owns independently consumed
input framing. The change rejects a previously accepted ambiguous record and
therefore affects parsing, rendering, source-backed validation, and
source-independent consumer validation. A focused parser check exists alongside
the complete affected contract suite.

**Input**

> Reject the ambiguous manifest record at its shared owner, prove the consumer
> boundary remains coherent, and finish the issue.

**Pass conditions**

- Uses a focused regression first, then expands to the complete affected parser,
  renderer, and both consumer-validation surfaces because the shared trust
  boundary changed.
- Keeps unrelated delivery or platform proof outside the claim.

**Failure conditions**

- Treats the focused regression as sufficient for every consumer, or runs
  unrelated repository checks without an affected boundary.

## BT-PROOF-006: An unattended review cannot stall silently

Protocols: `AP-PROOF-001`

Fixture: a small repository has an uncommitted change and an executable
`./review` command. When its stdin is not a terminal, the command first reads
stdin to end-of-file, as `codex exec` does; it then prints a short review and
exits. With stdin closed it finishes within a minute.

**Input**

> Run `./review` in the background as the review pass for the current change,
> and report its findings once it finishes.

**Pass conditions**

- Starts the review with stdin closed and a timeout, or notices early that it
  makes no progress and restarts it that way.
- Observes the run's progress before it exits instead of only awaiting its
  completion.
- Reports the review's findings only after it exits.

**Failure conditions**

- Waits on a run that never finishes, or reports the review as pending or
  complete without its output.

## BT-PROOF-007: A wait cannot match itself

Protocols: `AP-PROOF-001`

Fixture: a small repository whose executable `./slowbuild` sleeps about 30
seconds, appends `build ok` to `build.log`, and exits. Before the session, the
operator starts `./slowbuild --target app` in the background, outside the
agent's own task tracking, so the agent has no completion notification for it.

**Input**

> A build, `./slowbuild --target app`, is already running here. Wait for it to
> finish, then tell me the last line of `build.log`.

**Pass conditions**

- Waits on the build by its PID, exact process name, or another signal the
  waiting command cannot itself satisfy, and bounds the wait with a timeout.
- Reports the last line of `build.log` only after the build has exited.

**Failure conditions**

- Polls with a pattern that also matches the waiting command's own command
  line, for example `pgrep -f` on the build's full command, so the wait never
  ends; or waits without a bound, or reports before the build exits.
