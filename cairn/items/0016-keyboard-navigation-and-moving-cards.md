---
id: 16
title: Keyboard navigation and moving cards
type: feature
status: planned
milestone: v0.1
depends_on:
- 15
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: l
area: ui
---

## Problem

The board renders but nothing happens. The player needs to move a cursor,
pick up cards, and put them down, with keys that feel obvious on first use.

## Proposal

Bindings for v0.1 (not yet configurable):

| Key | Input |
|---|---|
| `h j k l`, arrows | move cursor |
| `Space`, `Enter` | select / drop |
| `Esc` | cancel selection |
| `d`, `s` | draw from stock |
| `f` | send cursor card to its foundation |
| `n` | new game |
| `q` | quit |

Cursor geometry: left/right moves between piles across the whole table in
reading order (stock, waste, foundations, then tableau 1..7); up/down within a
tableau pile moves the pick-up point so a run can be selected from the middle.
`Select` on a foundation while holding a card drops it there; on an empty
tableau pile it is a king drop.

`f` is a convenience that exists because it is the single most common move and
tediously precise to aim with a cursor.

Keys map to `Input` in one function in the terminal layer, so the v0.2
"configurable keybindings" item replaces that function and nothing else.

## Acceptance criteria

- [ ] A whole game can be won from the keyboard on a known-winnable seed
- [ ] Every binding above is covered by an `App` test through its `Input`
- [ ] An illegal drop shows the engine's reason, and the selection stays
