---
name: fresh-pass
description: Take a fresh eyes and simplification pass over recent work and fix everything found — bugs, omissions, confusion, duplication, and needless complexity — then verify. Use when the user says fresh pass, fresh eyes and simplify, "take a fresh eyes and simplification pass fixing anything you find", review and fix, polish pass, or clean up what you just did.
---

# Fresh Pass

Reread recent work as if seeing it for the first time, simplify it, and fix what you find. Unlike `fresh-eyes`, this skill edits: fixing is the point.

## Workflow

1. **Rebuild context.** Read applicable project instructions (CLAUDE.md, AGENTS.md). Inspect `git status` and the diff for everything written or modified in this session (or the target the user names). Read surrounding code, tests, docs, and call sites needed to judge the change.
2. **Fresh eyes: look for defects.**
   - syntax errors, invalid config or metadata, broken commands or examples
   - logic bugs, unhandled edge cases, missing error handling, brittle assumptions
   - behavior that contradicts docs, project rules, tests, or existing conventions
   - omissions: forgotten files, tests, options, dependencies, docs, or user-visible behavior
   - misconceptions about APIs, data flow, async behavior, permissions, or ownership
   - confusing names, ambiguous wording, surprising defaults, typos, run-on sentences
   - unrelated churn or accidental loss of functionality
3. **Simplification: look for needless complexity.**
   - duplicated code or wording that should be shared or centralized
   - reinvented helpers when an existing utility in the codebase already does it
   - dead code, unused variables, imports, styles, or config left behind
   - indirection, abstraction, or options nothing needs
   - code that doesn't match the surrounding style, idiom, or comment density
   - wasted work: redundant computation, repeated I/O, needless re-renders or loops
4. **Fix.** Apply every confirmed finding directly. Keep each fix scoped; preserve behavior, features, supported edge cases, and safety checks. Do not simplify away anything the task required. When a finding is uncertain or a fix would change intended behavior, leave it and report it instead.
5. **Verify.** Rerun the project's relevant checks (tests, typecheck, lint, build, or a run of the changed behavior) after fixing. If a fix breaks something, fix or revert it.
6. **Repeat once if warranted.** If the fixes were substantial, take one more quick pass over the new diff; stop when a pass finds nothing worth changing.

## Standards

Be concrete: every change ties to a file and a specific problem. Skip harmless preferences and speculative rewrites. Review your own earlier work in this session with the same skepticism as anyone else's.

## Report

Keep it short:

1. **Fixed**: each change, with `file:line` and why.
2. **Left alone**: anything noticed but deliberately not changed, with the reason.
3. **Verification**: the commands run and their results; say plainly if anything still fails.

If nothing needed fixing, say so and name the checks performed.
