---
name: fix-or-bead
description: Run one bounded fresh-eyes pass over recent changes, fix only confirmed regressions introduced by the diff or blockers to the original goal, create self-contained Beads for concrete actionable out-of-scope findings, then stop. Use when the user says fix-or-bead, asks for a bounded fresh-eyes pass, wants regressions fixed and everything else beaded, or wants to prevent review-driven scope creep while preserving useful findings.
---

# Fix Or Bead

## Purpose

Complete one review, repair, and triage cycle without letting the review expand the original task. Preserve useful discoveries as Beads instead of absorbing them into the active change.

## Establish The Boundary

Before reviewing:

1. Read applicable repository instructions, the original request or active Bead, its acceptance criteria, and explicit non-goals.
2. Inspect `git status`, the current diff, its base, and the directly affected call sites and tests.
3. Restate the original goal in one sentence. If it is unclear, infer the narrowest defensible goal from the active Bead and recent request, state that assumption, and do not broaden it.

Use exactly three dispositions:

- **Fix now**: a confirmed regression caused by the reviewed diff, or a confirmed defect that prevents the original acceptance criteria.
- **Bead**: a concrete, actionable finding outside the original goal that has meaningful correctness, user, maintenance, security, or performance impact.
- **Reject**: speculation, style preference, harmless oddity, duplicate work, unsupported concern, or low-value churn.

Do not reopen or widen the active Bead for a follow-up finding.

## One-Pass Workflow

1. Review the changed work once with fresh-eyes standards. Inspect nearby code only where needed to understand direct interaction; do not turn the pass into a repository-wide audit.
2. Require concrete evidence for every finding: a file and symbol or line, an observable failure or violated contract, and a clear causal explanation.
3. Classify every finding before editing. Lock the `Fix now` set; do not add adjacent improvements while implementing it.
4. Ensure the repository's required planning state exists before behavior edits. Update the active Bead when repository rules require it, but keep its original scope intact.
5. Implement the smallest complete repairs for the locked set. If a repair requires a new subsystem, architectural change, or out-of-scope behavior, do not implement it; create a Bead instead.
6. Run focused verification for each repair and the proportional repository gates. A failure introduced by the repair stays in scope; an unrelated failure is recorded, not silently fixed.
7. For each `Bead` finding, search open and closed Beads for duplicates, then use the repository's documented CLI (`br` or `bd`) to create one independently implementable issue. Serialize tracker writes.
8. Stop after reporting the fixed findings, created Beads, rejected candidates, verification, and residual risk. Do not perform another fresh-eyes round unless the user explicitly requests a new pass.

## Bead Quality

Create no Bead without verified evidence. Write follow-ups with the `create-beads` template (`../create-beads/references/bead-body-template.md`) and keep each within its Bead Size rule. Each follow-up must include:

- why the issue matters and the exact current evidence
- intended outcome and the smallest likely owning surface
- explicit in-scope and out-of-scope boundaries
- implementation notes without prescribing speculative architecture
- exact verification commands from the repository
- stop conditions and duplicate/dependency context

Do not create Markdown plans or TODO files. If no Beads CLI is available, report the actionable finding and the missing tracker capability instead of inventing another backlog.

## Simplification And Commit Discipline

Simplification is not implicit. When the user invokes a simplification skill alongside this one, keep it to the reviewed diff and one bounded round. Accept only behavior-preserving candidates that clearly reduce production code, concepts, duplicated ownership, or upstream conflict surface; bead behavior-changing ideas.

Do not create one commit per finding. Follow repository commit rules and user authorization, fold related corrections into the existing unpublished change when safe, and keep the final stack organized by behavior rather than review chronology. Never rewrite shared history without authorization.

## Stop Conditions

Stop immediately when:

- the original goal and reviewed diff cannot be identified safely
- a proposed fix no longer fits `Fix now`
- a follow-up is speculative or duplicates an existing Bead
- required verification fails for an unrelated baseline reason
- the single bounded pass, scoped repairs, follow-up Beads, and verification are complete
