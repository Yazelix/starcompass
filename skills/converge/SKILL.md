---
name: converge
description: Finish a change to code, tests, or scripts through one full review by fresh read-only reviewers, Claude and Codex in parallel, after which the author fixes or rebuts each finding and the reviewer that found it re-checks only those fixes. Use when the user says converge, finish through converge, run the feedback loop, review until clean, or when a project rule says such changes finish only after a fresh, independent review.
---

# Converge

Finish implementation through the review shape in `AP-PROOF-001`: one full review, then narrow re-checks of the fixes. The main session orchestrates; it does not review the diff itself.

## When It Applies

- Any change to code, tests, or scripts gets the full review, however easy the work looks. On an easy change it usually finds nothing, and the loop ends there.
- Docs-only and planning-only work (bead edits, wording the owner reviews anyway) skips the loop. Say so; the owner's review is the check.

## Setup

1. **Input.** A bead ID, or the current diff when there is none. With a bead, prime on it in focused mode (the `prime` skill): rules, the bead and its latest handoff, and the files it names.
2. **Base.** Record the base revision: the commit before the work under review. Reviews read `git diff <base>` plus untracked files, so do not commit until the loop ends.
3. **Limits.** Collect the project's verification commands and build or run limits (job counts, test threads, muted runs, per-worktree target dirs) from its AGENTS.md or README.
4. **Implement.** If the change is not written yet, spawn an implementer subagent with the bead ID, the handoff, where the rules live, and the limits; it writes the change and runs the checks. In a fresh, short session the main agent may write a small change itself. Whoever wrote the change is the author for the rest of the loop. The review is never optional for code.
5. **New surface.** Once the change is written, run this skill's `scripts/new_surface.py <base>` inside the repository. It prints one line per dependency (Cargo.toml, package.json, pyproject.toml, requirements*.txt) and per `pub` or `export` line that the diff and untracked files add, and nothing when there are none. The full review gets this report unchanged; it is review input, not a gate.
6. **Codex.** Codex reviews beside Claude on every change to code, tests, or scripts. The two models catch different defects: on 141 Eon beads reviewed by both, they shared only about a fifth of blocking findings; Codex mostly found code bugs, Claude mostly found wrong contract or proof text, missing records, and untested routes. The one exception is a change that only moves tests or refactors without changing behaviour, shown by unchanged test results; if Claude reports a behaviour change, Codex reviews in full too. Without Codex installed, Claude reviews alone.

Reviewers never edit or build: only the author changes the tree and runs the checks, so the two reviewers can run at once without colliding.

## Snapshots

Snapshot the candidate as a Git tree without touching the index or the working tree:

```sh
GIT_INDEX_FILE=<scratch>/snap.idx sh -c 'git read-tree HEAD && git add -A && git write-tree'
```

Take one before the full review and one after each set of fixes. `git diff <before> <after>` is the fix diff a re-check reads.

## Full Review

Run the checks first; the review starts from a candidate that passes them. Snapshot it, then start both reviewers at once, in the background.

**Claude.** Spawn a brand-new general-purpose subagent, never a fork: a fork inherits this context and is not fresh. Give it this request, adapted from the owner's prompt so that reviewers report instead of fixing:

> take a fresh eyes and simplification pass over this diff
>
> The diff is `git diff <base>` plus untracked files. Use the `fresh-eyes` skill, and look for simplifications too. Read the bead with `br show --no-db <id>` for the intended outcome. Say whether each item in this new-surface report should go or stay, and why: <report, or "none">. Do not edit files or run builds. Report each finding with its file and line, what is wrong, the fix you suggest, and whether the fix is text-only (comments, docs, wording).

**Codex.** Run it unattended and read-only, with stdin closed and a timeout:

```sh
timeout 30m codex exec -s read-only -C <repo> -o <scratch>/codex-<step>.md "<the same request>" </dev/null > <scratch>/codex-<step>.log 2>&1
```

The read-only sandbox cannot take `br`'s lock files, which is why the requests use `br show --no-db`. Check the log within the first two minutes; no output means the run stalled, and a stall or timeout fails it. Read the result file for the report.

Neither reviewer hears what any other review found.

## Fixes

The author answers every finding: fix it, or rebut it with a reason. Nits never block; the author may drop them without one. Then the author runs the checks and the main session takes a new snapshot.

## Re-check

For each reviewer whose findings led to a fix that is not text-only, or to any rebuttal, start a fresh instance of the same model, read-only as above, with this request:

> re-check the fixes for your earlier review
>
> Read the bead with `br show --no-db <id>`. Your findings and the author's answer to each: <findings and answers>. The fixes are `git diff <before> <after>`. Do not edit files or run builds. Report whether each fix resolves its finding, whether any fix adds a defect, whether each rebuttal holds, and whether any fix changes behaviour beyond its finding or crosses a trust boundary (security, data loss, permissions).

A failed fix or rebuttal goes back to the author and then to another re-check of the new fix diff. A fix flagged as going beyond its finding or crossing a trust boundary reruns the full review on the current candidate; the author may also call one.

## Convergence

- The loop converges when every re-check due has accepted all fixes and rebuttals, or none was due, no rerun is pending, and the checks pass on the final candidate.
- A step is one round of re-checks, run in parallel across reviewers, or one full rerun. After the full review, at most two more steps per slice. If anything is still open after the second, stop and report to the owner what remains, step by step. Do not keep looping.

## Finish

1. Record on the bead: each step's reviewers and their findings, the author's fixes and rebuttals, each re-check's verdict, and the final check results.
2. Commit under the project's git rules.
3. Close the bead only when its acceptance needs no owner sign-off. Otherwise leave it open and say what awaits the owner.

## Report

Keep it short: each step (reviewers, findings, fixes or rebuttals, verdict), whether the loop converged or stopped at the cap, the final checks, and the bead's state.
