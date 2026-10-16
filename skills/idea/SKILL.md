---
name: idea
description: Generate high-quality project improvement ideas by first producing exactly 30 brief candidates, then critically filtering them, rejecting weak or poorly justified options, and expanding only the strongest ideas into concrete actionable plans with rationale, downsides, and confidence scores. Use when the user asks for best ideas, improvement ideas, roadmap options, project recommendations, strategic technical opportunities, or a systematic idea critique.
---

# Idea

## Overview

Produce a broad idea set first, then apply a hard filter. The goal is not to defend every idea; it is to find the few that remain excellent after scrutiny.

## Workflow

1. Build enough context before ideating. For code projects, read the project instructions, README, manifests, architecture files, relevant source, tests, and issue tracker context as needed. If a priming skill is available and appropriate, use it or perform the same investigation directly.
2. Generate exactly 30 candidate ideas as one-liners. Keep them specific to the project, not generic engineering advice.
3. Evaluate every candidate systematically. For each idea, decide `Keep` or `Reject` and give the reason.
4. Reject aggressively. Discard ideas that are speculative, low-impact, too broad, duplicative, already implemented, inconsistent with project rules, likely to add maintenance burden, or unsupported by observed project evidence.
5. Expand only the kept ideas. For each kept idea, explain:
   - the concrete plan
   - why it improves the project
   - possible downsides and tradeoffs
   - implementation notes or code snippets where they clarify the plan
   - confidence that it improves the project, from `0%` to `100%`

## Evaluation Criteria

Keep an idea only when it scores well on most of these dimensions:

- **Evidence**: supported by repository files, docs, tests, issues, or observed gaps
- **Impact**: meaningfully improves user value, reliability, maintainability, performance, or contributor experience
- **Fit**: aligns with the project's architecture, conventions, roadmap, and stated constraints
- **Feasibility**: can be implemented with a realistic scope and clear verification path
- **Leverage**: fixes a root cause, removes complexity, or unlocks multiple future improvements
- **Risk**: has acceptable migration, compatibility, and maintenance costs

## Output Shape

Use this structure:

1. **30 Candidate Ideas**: numbered list, one line each.
2. **Critical Evaluation**: compact table or numbered review with `Keep` or `Reject` and the reason.
3. **Kept Ideas In Detail**: one section per kept idea.

For detailed plans, be concrete enough that implementation could start from the description. Include file paths, command names, data structures, API shapes, or concise code snippets where relevant. Do not invent exact code against files that were not inspected; mark such snippets as illustrative when they are not verified against the codebase.

## Quality Bar

Prefer a small final set of strong ideas over a large set of weak recommendations. If no idea survives the filter, say so and explain what evidence would be needed to produce better candidates.

Do not make repository edits while using this skill unless the user separately asks to implement one of the kept ideas.
