---
name: prime
description: Prime the agent on a repository before implementation, review, planning, or debugging by fully reading AGENTS.md and README.md, extracting repo-specific workflow rules, and investigating the codebase architecture, purpose, entrypoints, and verification surfaces. When given a bead or a handoff to continue ("prime, then continue ap-123 from its handoff"), primes in a focused mode that reads only the rules, the bead, and the files it touches, to keep context for the work. Use when the user asks to prime, onboard, understand a repo/project, "read the instructions first", "use code investigation agent mode", or otherwise wants deep project context before work begins.
---

# Prime

## Overview

Prime the working context before changing code. Read the project instructions and README in full, then build a concise architecture map from the repository itself.

## Modes

- **Focused**: the request names a bead or a handoff to continue, for example "prime, then continue ap-123 from its handoff". Keep context small; the session must leave room to finish the bead. Follow the Focused Workflow.
- **Full**: first-time onboarding with no specific bead. Follow the Workflow below.

## Focused Workflow

1. Read every applicable `AGENTS.md` completely. Repository rules are never skipped.
2. Read the bead and its comments with the repository's Beads CLI (`br show <id>`), plus its parent epic if it has one. The latest handoff comment is the starting point: treat recorded decisions as decided and do not retry approaches it rules out.
3. Check the tree: `git status`, and whether the revision the handoff names has moved since.
4. Read only the files, README sections, and tests the bead and handoff name, plus direct callers when you need them. Skip the architecture survey. If a wider question comes up, delegate it to a subagent when the instructions allow it and keep only the conclusion.
5. Summarize in a few lines: the bead's goal, the state from the handoff, the next step, open decisions, and the check that verifies the work. If the user asked to continue, proceed with the next step.

## Workflow

1. Locate the repository root and confirm the current working directory.
2. Read every `AGENTS.md` that applies to the working tree, starting at the current directory and walking upward if needed. Read each file completely, in chunks if necessary.
3. Read the repository `README.md` completely, in chunks if necessary.
4. Extract and honor repo-specific rules before taking further action:
   - workflow and issue-tracking requirements
   - file naming and formatting conventions
   - command, test, build, and verification policy
   - commit, push, and release constraints
   - safety rules about user files, generated artifacts, or external state
5. Investigate the codebase architecture using high-signal local inspection:
   - list top-level files and directories
   - inspect build/package manifests and lockfiles
   - identify languages, frameworks, runtime entrypoints, scripts, and tests
   - read core modules that define the program's purpose and control flow
   - search for names from the README, commands, configuration files, and public APIs
6. Use code investigation agent mode or explorer subagents only when the active instructions permit delegation. If delegation is unavailable or not permitted, perform the same investigation locally.
7. Summarize the priming result before implementation or review work:
   - what the project does
   - the main architecture and execution path
   - the most important repo rules to obey
   - relevant commands for validation
   - open questions, risks, or areas not yet understood

## Investigation Guidance

Prefer local source evidence over assumptions. Use `rg` and `rg --files` for searches. Read files directly instead of relying on stale memory. When a repo uses its own task tracker or project command wrapper, use the repo's documented commands.

Do not make code edits during priming unless the user has also asked for an implementation and the repo rules permit proceeding. If the project instructions require planning or user approval before edits, stop after the summary and ask for approval.

If `AGENTS.md` or `README.md` is missing, say so explicitly and continue with the nearest durable source of project instructions, such as `CONTRIBUTING.md`, docs, manifests, or task tracker context.

## Output Shape

Keep the final priming summary concise and operational. Do not paste long excerpts from instructions or README; cite the important constraints and source files instead.
