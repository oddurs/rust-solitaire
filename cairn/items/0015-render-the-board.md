---
id: 15
title: Render the board
type: feature
status: planned
milestone: v0.1
depends_on:
- 14
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: l
area: ui
---

## Problem

There is no picture yet. The board must be readable at a glance in an 80x24
terminal and show state the rules depend on: which cards are face down, what
is selected, where the cursor is.

## Proposal

One `ratatui` widget per pile kind, composed in a `Board` widget:

- Row 1: stock (card back or empty slot), waste (top card), gap, four
  foundations labelled with their suit when empty.
- Rows 2+: seven tableau columns, each card on its own line with the rank and
  suit, face-down cards as a back pattern, the top card fully drawn.
- Cursor: inverse video on the location. Selection: a bracket or colour on the
  run being moved.
- Message line at the bottom for `Illegal` reasons and status.

Cards are 3 cells wide (`A♠`, `10♥`) with red suits in red and black suits in
the default foreground, so the game is playable on a monochrome terminal by
glyph alone. Unicode suits by default; the ASCII fallback is a v0.2 item.

Size: a message replaces the board when the terminal is under 80x24 rather
than a garbled layout. Empty piles draw as a dotted outline so a destination is
visibly a place.

Rendering reads `App::view()` only; it never touches `Game` directly.

## Acceptance criteria

- [ ] A fresh deal renders correctly at 80x24 (snapshot via `TestBackend`)
- [ ] Cursor, selection, face-down and empty piles are each visually distinct
- [ ] A too-small terminal shows the size message and nothing else
