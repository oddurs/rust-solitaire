---
id: 23
title: 'Hints: show a legal move'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 9
- 16
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: m
area: app
---

## Problem

Stuck players either want to know a move exists or want to know they should
give up. `legal_moves()` already answers both.

## Proposal

`?` is taken by help, so `H` (or `Tab`) highlights the source and destination
of the next legal move, cycling on repeat, ordered by usefulness: foundation
moves first, then moves that expose a face-down card, then tableau
rearrangements, then `Draw` last. If the only move is `Draw`, say so. If
`is_stuck()`, say that instead.

Hints do not apply the move; the player still has to make it, which keeps the
game theirs.

## Acceptance criteria

- [ ] Hint ordering tested against a hand-built position
- [ ] Cycling wraps and never repeats within a cycle
- [ ] Stuck position reports stuck rather than a hint
