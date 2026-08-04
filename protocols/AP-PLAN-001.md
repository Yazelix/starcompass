---
id: AP-PLAN-001
version: 5
title: Durable planning state
---

Keep the outcomes and constraints that later work needs in the project's
durable planning system or canonical documentation. An issue represents a
chosen goal, decision, material defect, or schedulable follow-up. Review and
implementation methods belong to that issue.

An agent run is one uninterrupted execution ending when control returns to the
user, including automatic continuations. A run may inspect any planning state
read-only, but it may create, claim, update, implement, or close at most one
Bead or repository-designated equivalent issue.

Required practice:

- When work is tracked by a Bead, bind the run to that one owning issue before
  its first issue write or implementation edit.
- After the owning issue is complete, blocked, or handed off, stop and return
  control to the user. Do not begin another issue, including work newly
  unblocked by the completion.
- Leave follow-up findings as reported findings for a later run. If issue
  creation is the requested outcome, the one issue created is that run's owning
  issue.
- After review, fresh-eyes, simplification, or verification, update the owning
  issue's editable fields to describe the accepted state instead of pass
  chronology.
- In a later run, create a separate issue only for a material finding outside
  the prior owning scope or one worth scheduling on its own. Name it after the
  outcome or finding.
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
