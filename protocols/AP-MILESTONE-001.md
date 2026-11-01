---
id: AP-MILESTONE-001
version: 1
title: Ordered milestones
---

A milestone is an epic labelled `milestone`. Agents may propose milestones and
their order; the user approves each one before anyone creates it.

Order comes from `blocks` dependencies between milestone epics. The next
milestone is the first open one whose milestone blockers are all closed; break
ties by priority, then creation date. Never order by priority alone or by due
date; an optional due date is for display only.

A milestone ships when its epic closes; the close reason or external reference
may name the release tag. Keep the next milestone's child issues current as
work lands.
