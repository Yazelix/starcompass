---
id: AP-GIT-001
version: 4
title: Safe repository history
---

Repository history is shared user state. Inspect status and local policy;
preserve unrelated or concurrent work and use the least destructive sufficient
operation.

Fold same-task corrections into its unpublished commit, then refresh dependent
commits and generated artifacts. After a push, promotion, release, external pin,
or other reliance point, add a commit. Never reset, discard, force-push, rewrite
published history, or create a branch without authority. Verify the intended
diff before commit and remote state after push. Roll back with a reviewed revert
unless local policy says otherwise.
