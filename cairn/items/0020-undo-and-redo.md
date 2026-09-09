---
id: 20
title: Undo and redo
type: feature
status: backlog
milestone: v0.2
depends_on:
- 14
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: m
area: app
---

## Problem

The first thing anyone does after a wrong move is press `u`. Without undo, a
mis-drop is a lost game, and mis-drops are easy with a cursor.

## Proposal

Snapshot history: `App` keeps `Vec<Game>` of prior states (a `Game` is a few
hundred bytes; 500 moves is nothing) and a redo stack cleared on any new move.
This is simpler and less bug-prone than inverse moves, and the engine stays
free of history. `u` undoes, `U` or `Ctrl-r` redoes.

Undo counts as a move for scoring (standard Klondike penalises it); that lands
with the scoring item, but `App` records the undo count now.

Undo across a `Draw` recycle must restore the exact stock/waste order, which
snapshots give for free and inverse moves would not.

## Acceptance criteria

- [ ] Undo after every kind of move restores the identical `Game`
- [ ] Redo after undo re-applies; a new move clears the redo stack
- [ ] Undo on a fresh deal is a no-op with a message, not an error
