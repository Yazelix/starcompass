---
id: AP-PLAN-001
version: 6
title: Durable planning state
---

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
