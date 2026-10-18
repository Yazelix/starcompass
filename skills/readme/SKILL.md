---
name: readme
description: Update README.md and related project documentation so they accurately describe the current project state after recent changes, including new commands, options, features, configuration, workflows, and verification steps. Use when the user asks to refresh docs, update README, document recent changes, synchronize docs with code, or rewrite documentation in present-tense current-state language without phrases like "we added", "new", "now", or migration-style wording.
---

# README

## Overview

Refresh documentation from source evidence. Describe the project as it currently works, not as a history of recent edits.

## Workflow

1. Read the active project instructions first, including `AGENTS.md` files that apply to the working tree.
2. Inspect the current documentation surface:
   - `README.md`
   - `docs/`
   - command reference files
   - configuration examples
   - contracts or architecture docs that mention changed behavior
3. Identify recent changes from reliable local evidence:
   - `git status`
   - `git diff`
   - recent commits when relevant
   - changed manifests, command implementations, config schemas, tests, and help text
   - issue tracker context if the repo requires it
4. Map changed behavior to documentation updates. Include commands, options, config keys, features, defaults, workflows, installation/setup details, and verification commands that users need to know.
5. Edit the README and only the related documentation that should reflect the current behavior. Keep unrelated prose churn out.
6. Rewrite in present-tense current-state language:
   - Prefer "The command accepts `--flag`" over "The command now accepts `--flag`"
   - Prefer "Yazelix uses ..." over "Yazelix was changed to use ..."
   - Avoid "new", "recently", "now", "we added", "this change", "previously", and "no longer" unless documenting an explicit migration or historical compatibility contract
7. Preserve the repository's documentation style, punctuation rules, heading structure, examples, and naming conventions.
8. Verify docs against source. Search for the documented commands, flags, config keys, paths, and defaults in the implementation before calling the update complete.

## Documentation Quality Bar

Documentation updates should be:

- **Accurate**: every command, option, path, and default is backed by inspected code or checked docs
- **Complete**: user-facing behavior from the recent changes is documented where users will look for it
- **Current-state**: prose reads as if the behavior has always existed
- **Scoped**: edits are limited to documentation affected by the changes
- **Actionable**: examples show real invocations, config snippets, or workflows where useful

## Style Rules

Follow local documentation conventions even when they differ from generic Markdown habits. If the repo has README-specific rules, apply them exactly.

Avoid unsupported marketing language. Prefer short operational descriptions, command examples, option tables, and configuration snippets that match the implementation.

Do not document planned behavior as present behavior. If a feature is partial, experimental, or gated, say that directly using the project's terminology.

## Final Response

Summarize which documentation files changed, what user-facing surfaces were documented, and what verification was performed. Mention any docs that could not be verified from local source evidence.
