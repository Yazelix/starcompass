# Bead Body Template

Use this shape when creating implementation Beads. Adapt labels/types/priorities to the project.

```markdown
## Why

<Concrete cost, bug, risk, or opportunity. Cite evidence.>

## Evidence

- `<path>:<line>`: <what is there>
- `<path>:<line>`: <what is there>

## Scope

In scope:
- <files/modules/surfaces to touch>

Out of scope:
- <nearby files/surfaces not to touch>
- <behavior that must not change>

## Implementation Notes

1. <step>
2. <step>
3. <step>

## Verification

- `<command>` should exit 0 and <expected signal>
- `<command>` should exit 0 and <expected signal>

## Stop Conditions

- Stop if cited current-state code has drifted materially.
- Stop if the implementation requires touching an out-of-scope surface.
- Stop if verification fails twice for the same reason after a reasonable fix attempt.

## Maintenance Notes

- <review focus, future interaction, dependency decision, or rejected alternative>
```

Recommended Beads commands:

```bash
br create "<title>" -t task -p 2 --labels improve,<category>,<subsystem> --description "<body>"
br dep add <blocked-id> <blocker-id>
br update <id> --priority 1
br sync --flush-only
```

Prefer `task` for implementation slices, `bug` for confirmed defects, `decision` for unresolved architectural choices, and `epic` only for multi-bead programs.
