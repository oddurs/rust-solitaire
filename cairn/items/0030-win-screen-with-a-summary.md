---
id: 30
title: Win screen with a summary
type: feature
status: backlog
milestone: v0.2
depends_on:
- 11
- 24
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: s
area: ui
---

When `is_won()`, overlay the board with the result: moves, time, score, seed,
and `n` for a new game or `q` to quit. The board stays visible underneath
because a finished table is satisfying to look at.

Something small and celebratory is allowed (the classic cascade is a `later`
item; here it is text).

## Acceptance criteria

- [ ] Appears exactly once, on the move that wins
- [ ] Shows the same numbers the status line showed
