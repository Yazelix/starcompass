---
name: fresh-pass
description: Take a fresh eyes and simplification pass over recent work and fix everything found — bugs, omissions, confusion, duplication, and needless complexity — then verify. Use when the user says fresh pass, fresh eyes and simplify, "take a fresh eyes and simplification pass fixing anything you find", review and fix, polish pass, or clean up what you just did.
---

# Fresh Pass

Reread recent work as if seeing it for the first time, simplify it, and fix what you find. Unlike `fresh-eyes`, this skill edits: fixing is the point. It looks for two kinds of finding only, correctness gaps and deletions, and adds code only to fix a defect.

## Workflow

1. **Rebuild context.** Read applicable project instructions (CLAUDE.md, AGENTS.md). Inspect `git status` and the diff for everything written or modified in this session (or the target the user names). Read surrounding code, tests, docs, and call sites needed to judge the change.
2. **Correctness gaps.**
   - syntax errors, invalid config or metadata, broken commands or examples
   - logic bugs, unhandled edge cases, missing error handling, brittle assumptions
   - behavior that contradicts docs, project rules, tests, or existing conventions
   - omissions: forgotten files, tests, options, dependencies, docs, or user-visible behavior
   - misconceptions about APIs, data flow, async behavior, permissions, or ownership
   - wrong or misleading names, docs, messages, or comments
   - unrelated churn or accidental loss of functionality
   - a check that cannot fail: trust a new negative or absence check (`! grep`, "prints nothing", "no match") only after showing it fail on a seeded violation, then remove the seed
3. **Deletions.** For each new function, type, parameter, or option, answer yes or no, then delete or inline what nothing needs:
   - Does this have a second caller today?
   - Is this option ever set?
   - Does an existing helper or the standard library already do this? If yes, name its path.

   Also delete:
   - leftovers: whatever existed only for a concept the diff removes or narrows — a single-field struct, a single-call wrapper, an always-true parameter, an unused derive, `Result` on infallible code, a generic with one instantiation, a test asserting a no-op
   - duplicates: code or wording repeated in the diff or elsewhere, and comments that restate the code
   - dead code, unused variables, imports, styles, or config
   - wasted work: redundant computation, repeated I/O, needless re-renders or loops
4. **Fix.** Bug fixes add whatever the defect needs. Simplification fixes only delete or inline; never add a helper, abstraction, or option to simplify. Keep each fix scoped; preserve behavior, features, supported edge cases, and safety checks. Do not simplify away anything the task required. When a finding is uncertain or a fix would change intended behavior, leave it and report it instead.
5. **Verify.** Rerun the project's relevant checks (tests, typecheck, lint, build, or a run of the changed behavior) after fixing. If a fix breaks something, fix or revert it.

This is one pass. To repeat passes until one changes nothing, use the `converge` skill.

## Standards

Every finding cites `path:line` and a specific problem. Skip style, harmless preferences, and speculative rewrites. Sound work has no findings; do not invent some. Review your own earlier work in this session with the same skepticism as anyone else's.

## Report

Keep it short:

1. **Fixed**: each change, with `path:line` and why.
2. **Left alone**: anything noticed but deliberately not changed, with the reason.
3. **Verification**: the commands run and their results; say plainly if anything still fails.

If nothing needed fixing, say so and name the checks performed.
