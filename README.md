# skills

Agent skills I use with Codex and Claude Code. Each folder in `skills/` is one
skill: a `SKILL.md` with frontmatter, plus optional references, scripts, and
Codex display metadata in `agents/openai.yaml`.

| Skill | Use it to |
| --- | --- |
| `prime` | Read a repository's instructions, README, and architecture before working, or only what one bead needs |
| `prime-beads` | Review and refine a Beads plan, keeping beads small enough for one session |
| `create-beads` | Turn chosen work into right-sized beads, with epics for larger work |
| `list-beads` | List in-progress Beads issues and the 10 highest-priority ready ones |
| `handoff` | End a session with a handoff on the owning bead for a fresh agent |
| `audit-to-beads` | Audit a codebase and record improvements as Beads, without editing code |
| `fresh-eyes` | Reread recent work for bugs, omissions, and confusion, without editing |
| `fresh-pass` | Reread recent work, simplify it, and fix what turns up |
| `fix-or-bead` | One bounded review pass: fix in-scope regressions, bead the rest |
| `simplify-and-refactor-code-isomorphically` | Shrink code while proving behavior stays the same |
| `opinion` | Get a candid assessment of a project |
| `weakness` | Find a project's weakest and most limiting parts |
| `idea` | Generate 30 improvement ideas and keep only the strongest |
| `readme` | Bring README and docs in line with the current code |
| `ui` | Design and build polished frontend UI |
| `stop-slop` | Remove AI writing patterns from prose |

## Install

```sh
git clone https://github.com/luccahuguet/skills ~/pjs/personal/skills
~/pjs/personal/skills/install.sh
```

`install.sh` symlinks every skill into `~/.codex/skills` and `~/.claude/skills`
for each host that is installed, so an edit here reaches both. It moves any
existing folder with the same name to `~/.local/state/skills-backup/` first and
can be rerun safely.

## Credits

- `stop-slop` is by [Hardik Pandya](https://hvpandya.com) and keeps its own MIT
  license in `skills/stop-slop/LICENSE`.
- `simplify-and-refactor-code-isomorphically` was written independently,
  inspired by ideas Jeffrey Emanuel shared publicly.
- `audit-to-beads` is inspired by `shadcn/improve`.

## License

Except for `skills/stop-slop`, this repository is licensed under either of
[MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
