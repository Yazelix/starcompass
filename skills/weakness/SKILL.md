---
name: weakness
description: Identify the weakest, worst, most fragile, or most strategically limiting parts of a project from source evidence, then explain which areas most need fresh ideas and creative or clever improvements. Use when the user asks what is weak, broken, ugly, risky, overcomplicated, fragile, underdesigned, or most deserving of innovative redesign after reviewing a codebase or system.
---

# Weakness

## Overview

Find the system's real pressure points. The goal is candid diagnosis grounded in evidence, with creative opportunity framing where a fresh design could materially improve the project.

## Workflow

1. Build enough context before judging. Read the active project instructions, README, relevant docs, architecture files, recent changes, issue tracker context, and core source paths. If a priming skill is available and appropriate, use it or perform the same investigation directly.
2. Identify candidate weaknesses from evidence:
   - code that is hard to reason about, duplicate, or overly coupled
   - workflows with many manual steps, hidden state, or surprising failure modes
   - user-facing behavior that is brittle, slow, confusing, or hard to validate
   - architecture that depends on local-only assumptions or unclear ownership boundaries
   - tests, docs, or contracts that fail to protect important behavior
   - stale surfaces, compatibility paths, or abstractions that add cost without leverage
3. Distinguish true weaknesses from ordinary complexity. Do not mark something weak just because it is large, unfamiliar, or locally inconvenient.
4. Rank the strongest findings by severity and leverage. Prefer root causes over symptoms.
5. For each ranked weakness, explain:
   - what is weak
   - evidence from the repository
   - why it matters
   - what kind of fresh, creative, or clever improvement it calls for
   - likely difficulty and risk
   - confidence from `0%` to `100%`
6. Include non-findings when useful: areas that looked suspicious but should not be treated as major weaknesses after inspection.

## Evaluation Criteria

Prioritize weaknesses with several of these traits:

- **User impact**: affects real usage, reliability, onboarding, performance, or trust
- **Blast radius**: touches core flows, shared contracts, or many future changes
- **Hidden cost**: creates recurring debugging, manual repair, or coordination work
- **Design smell**: mixes responsibilities, duplicates authority, or hides state transitions
- **Weak verification**: important behavior lacks a direct, high-signal test or validator
- **Creative leverage**: an improved model, interface, contract, or automation could simplify several problems at once

## Output Shape

Use this structure:

1. **Weakness Map**: short ranked list of the most important weak areas.
2. **Detailed Findings**: one section per weakness with evidence, impact, creative improvement direction, difficulty, risk, and confidence.
3. **Most Promising Creative Targets**: the few areas where inventive redesign or clever automation would pay off most.
4. **Caveats**: limits of the investigation and areas needing more runtime/user evidence.

## Tone And Discipline

Be direct, but not performative. Avoid generic advice like "add more tests" unless tied to a specific unprotected contract. Avoid vague labels like "technical debt" without explaining the mechanism that makes it costly.

When suggesting creative directions, separate diagnosis from prescription. The weakness analysis can point toward possible redesigns, but do not turn every finding into a full implementation plan unless the user asks for next steps.

Do not make repository edits while using this skill unless the user separately asks to implement a fix or create follow-up issues.
