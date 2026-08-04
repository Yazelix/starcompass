---
id: AP-GIT-001
version: 1
title: Safe repository history
---

Repository history is shared user state. Preserve unrelated work, follow the
local branch and promotion model, and use the least destructive operation that
achieves the requested result.

Required practice:

- Inspect status and repository instructions before editing.
- Treat existing and concurrent changes as user-owned unless proven otherwise.
- Do not reset, discard, force-push, rewrite published history, or create a
  branch without authority from the user or repository policy.
- Verify the intended diff before committing and the remote state after pushing.
- Make rollbacks additive through a reviewed revert unless policy says otherwise.
