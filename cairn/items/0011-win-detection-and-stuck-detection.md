---
id: 11
title: Win detection and stuck detection
type: feature
status: done
milestone: v0.1
depends_on:
- 10
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: s
area: engine
---

## Problem

The game needs to know when it is over: won when all four foundations hold
thirteen cards, and stuck when no move other than cycling the stock can change
anything.

## Proposal

`Game::is_won()` is the foundation count. `Game::is_stuck()` is
`legal_moves()` minus `Draw` being empty *and* a full cycle of the stock
producing no new legal move; the second half is what makes it honest rather
than a hint that fires early. It is allowed to be O(stock) because it is called
once per move at most.

Stuck is advisory. The UI tells the player; it never ends the game for them,
because they may want to undo.

## Acceptance criteria

- [ ] A hand-built won position reports `is_won()`
- [ ] A hand-built dead position reports `is_stuck()`; the same position with one legal move does not
- [ ] Neither method mutates the game
