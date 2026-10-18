---
name: opinion
description: Give a candid, evidence-based opinion of the current project — whether it is a good idea, useful, well designed and architected, and pragmatic — and what would make it more useful, compelling, and intuitive for both humans and AI coding agents. Use when the user asks what you actually think of the project, for an honest assessment or critique, or invokes the opinion skill.
---

# Opinion

Answer this question about the current project:

> Now tell me what you actually THINK of the project-- is it even a good idea?
> Is it useful? Is it well designed and architected? Pragmatic? What could we
> do to make it more useful and compelling and intuitive/user-friendly to both
> humans AND to AI coding agents?

## How to answer

- If the project is not already primed in this conversation, read its
  `AGENTS.md`, `README.md`, and core sources first. Ground every judgment in
  what you read; cite files with `path:line` where it helps.
- Give your real view. Say plainly what is good, what is weak, and what you
  would cut. Do not hedge into neutrality or flatter.
- Cover each question: the idea, usefulness and who it serves, design and
  architecture, pragmatism (cost of process, scope, and maintenance against
  the value delivered), then improvements.
- Treat humans and AI coding agents as two distinct audiences. For agents,
  judge how easy the repository is to orient in, how much instruction text it
  costs, and how clear its owners, commands, and checks are.
- Rank improvements by impact and give your top recommendation with the reason.
  Mark which ones would need a scope or contract decision from the user.
- This is an assessment only: make no edits, beads, or other writes.
