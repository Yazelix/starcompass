---
name: converge
description: Finish a change to code, tests, or scripts through a review loop in which each round is a brand-new subagent that takes a fresh eyes and simplification pass over the diff and fixes what it finds, until a round changes nothing. Use when the user says converge, finish through converge, run the feedback loop, review until clean, or when a project rule says such changes finish only after a fresh, independent review pass makes no further changes.
---

# Converge

Finish implementation through fresh review rounds until one comes back clean. The main session orchestrates; it does not review the diff itself.

## When It Applies

- Any change to code, tests, or scripts ends with at least one clean round, however easy the work looks. On an easy change the first round usually changes nothing, and the loop ends there.
- Docs-only and planning-only work (bead edits, wording the owner reviews anyway) skips the loop. Say so; the owner's review is the check.

## Setup

1. **Input.** A bead ID, or the current diff when there is none. With a bead, prime on it in focused mode (the `prime` skill): rules, the bead and its latest handoff, and the files it names.
2. **Base.** Record the base revision: the commit before the work under review. Every round reviews `git diff <base>` plus untracked files, so do not commit between rounds.
3. **Limits.** Collect the project's verification commands and build or run limits (job counts, test threads, muted runs, per-worktree target dirs) from its AGENTS.md or README. Every round gets them.
4. **Implement.** If the change is not written yet, spawn an implementer subagent with the bead ID, the handoff, where the rules live, and the limits; it writes the change and runs the checks. In a fresh, short session the main agent may write a small change itself. The review rounds are never optional for code.
5. **New surface.** Once the change is written, run this skill's `scripts/new_surface.py <base>` inside the repository. It prints one line per dependency (Cargo.toml, package.json, pyproject.toml, requirements*.txt) and per `pub` or `export` line that the diff and untracked files add, and nothing when there are none. Every round gets this report unchanged; it is review input, not a gate.
6. **Codex.** Codex joins as a second-model reviewer when the bead is labelled `deep-review` or the diff changes more than about 200 lines of code, tests, or scripts (`git diff --numstat <base>`).

## Rounds

Take a fingerprint of the candidate before each round:

```sh
{ git diff --binary <base>; git ls-files -z --others --exclude-standard | xargs -0 -r sha256sum; } | sha256sum
```

**Claude round.** Spawn a brand-new general-purpose subagent, never a fork: a fork inherits this context and is not fresh. Give it the bead ID, the base, the limits, and this request, with the owner's prompt verbatim:

> take a fresh eyes and simplification pass over this diff, fixing anything you find
>
> The diff is `git diff <base>` plus untracked files. Use the `fresh-pass` skill for one pass. Read the bead with `br show <id>` for the intended outcome. Remove each item in this new-surface report, or say in your report why it stays: <report, or "none">. Run the verification commands within these limits: <limits>. Report what you changed and why, what you left alone, and the check results.

Do not tell it what earlier rounds found. It sees the bead, the diff, and the new-surface report only.

**Codex round.** Run Codex unattended, with stdin closed and a timeout, in the background:

```sh
timeout 30m codex exec -s workspace-write -C <repo> -o <scratch>/codex-round-<n>.md "<the same request>" </dev/null > <scratch>/codex-round-<n>.log 2>&1
```

Check the log within the first two minutes; no output means the round stalled, and a stall or timeout fails the round. Read the result file for the report.

**After each round,** fingerprint again. If it changed, start another round. If it did not change and the round's checks passed, the round is clean.

## Convergence

- Without Codex, the first clean Claude round converges.
- With Codex, run Claude rounds until one is clean, then a Codex round. If Codex changes nothing, both models are clean on the same candidate and the loop converges. If it changes something, return to Claude rounds and finish with Codex again.
- Nits never block. A round whose only changes are text (comments, docs, wording) restarts the loop at most twice per slice; after that, accept the text fixes without another round.
- After about 5 rounds without converging, stop and report what keeps changing, round by round. Do not keep looping.
- Only one round edits or builds at a time. Read-only helpers may run in parallel.

## Finish

1. Record on the bead: each round's reviewer and what it changed, or that it was clean; the converged round; and the final check results.
2. Commit under the project's git rules.
3. Close the bead only when its acceptance needs no owner sign-off. Otherwise leave it open and say what awaits the owner.

## Report

Keep it short: the rounds (reviewer, changes or clean), whether the loop converged or stopped at the cap, the final checks, and the bead's state.
