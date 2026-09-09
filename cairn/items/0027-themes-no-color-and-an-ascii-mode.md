---
id: 27
title: Themes, NO_COLOR and an ASCII mode
type: feature
status: backlog
milestone: v0.2
depends_on:
- 15
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: m
area: ui
---

Not every terminal has colour or Unicode. Three switches:

- `NO_COLOR` (the convention at no-color.org) and `--no-color` drop colour;
  suits stay distinguishable by glyph.
- `--ascii` renders suits as `C D H S` and backs as `##`, for terminals whose
  fonts lack the suit glyphs.
- `--theme NAME` selects from two or three built-in palettes (default, light,
  high-contrast). A theme is a struct of `ratatui::Style`s; no config file
  parsing here, that is the keybindings item's job and themes can move into it
  afterwards.

## Acceptance criteria

- [ ] Snapshot tests for ASCII and no-colour output
- [ ] `NO_COLOR` set in the environment is honoured without a flag
