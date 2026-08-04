---
id: AP-GIT-001
version: 3
title: Safe repository history
---

Repository history is shared user state. Preserve unrelated work, follow the
local branch and promotion model, and use the least destructive operation that
achieves the requested result.

Required practice:

- Inspect status and repository instructions before editing.
- Treat existing and concurrent changes as user-owned unless proven otherwise.
- Fold a correction into the current task's unpublished commit when it belongs
  to the same unit of work. Refresh and reverify dependent local commits and
  generated artifacts.
- Use a follow-up commit after a push, promotion, release, external pin, or any
  other point where someone outside the current local work can rely on the
  revision.
- Do not reset, discard, force-push, rewrite published history, or create a
  branch without authority from the user or repository policy.
- Verify the intended diff before committing and the remote state after pushing.
- Make rollbacks additive through a reviewed revert unless policy says otherwise.
