---
name: fresh-eyes
description: Check recent work again with fresh eyes, looking for blunders, mistakes, errors, oversights, omissions, logical issues, misconceptions, confusion, bugs, brittle assumptions, missing validation, and other concrete problems. Use when the user says fresh_eyes, fresh eyes, check everything again, reread your changes, review what you just wrote, sanity-check recent edits, inspect modified code, or look for anything wrong before moving on.
---

# Fresh Eyes

## Overview

Reread recent work as if seeing it for the first time. Prioritize concrete defects, omissions, and confusing reasoning over broad refactors or speculative polish.

## Workflow

1. Rebuild context before reviewing:
   - read applicable project instructions
   - inspect `git status`
   - inspect diffs for files just written or modified
   - read surrounding code, tests, docs, plans, prompts, commands, and call sites needed to understand the change
2. Review the changed work first, then nearby existing code or documentation that interacts with it.
3. Look specifically for:
   - syntax errors, invalid config, invalid metadata, or broken command examples
   - logic bugs, edge cases, missing error handling, or mismatched assumptions
   - behavior that contradicts docs, project rules, tests, or existing conventions
   - omissions where a required file, state, test, command, option, dependency, or user-visible behavior was forgotten
   - misconceptions about APIs, data flow, permissions, control flow, async behavior, or ownership boundaries
   - duplicated code or repeated wording that should be centralized or simplified
   - confusing names, unclear workflow steps, ambiguous output, or surprising defaults
   - overbroad changes, unrelated churn, or accidental loss of functionality
   - missing verification for the exact behavior touched
4. Separate findings by confidence:
   - **Bug**: likely incorrect and should be fixed
   - **Concern**: plausible issue worth checking or tightening
   - **Omission**: something required appears missing or incomplete
   - **Confusion**: naming, structure, docs, prompts, or behavior are likely to mislead future readers or agents
   - **DRY Opportunity**: duplication or simplification that improves clarity without losing behavior
   - **Non-Issue**: something suspicious that is acceptable after inspection
5. Do not edit files unless the user explicitly asks for fixes. If fixes are requested, keep them scoped to confirmed problems and preserve functionality.

## Review Standards

Be concrete. Every issue should point to a file, line, command, behavior, or exact wording.

Bias toward high-signal comments. Do not list harmless preferences, style nits, or hypothetical rewrites unless they reduce real confusion or risk.

Preserve behavior and intent. Do not recommend simplification that removes features, acceptance criteria, supported edge cases, useful explicitness, or important safety checks.

If nothing meaningful is wrong, say that directly and mention the verification performed and any residual risk.

## Output Shape

Use this structure:

1. **Findings**: ordered by severity, with file references and rationale.
2. **DRY And Clarity Opportunities**: only if they are worth acting on.
3. **Checked And Acceptable**: suspicious areas reviewed and left unchanged.
4. **Verification**: commands, validators, or inspections performed.
5. **Suggested Next Fixes**: concise list only when action is warranted.

When used after the assistant made changes, review the assistant's own work with the same skepticism as any other code or plan.
