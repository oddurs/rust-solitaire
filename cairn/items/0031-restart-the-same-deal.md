---
id: 31
title: Restart the same deal
type: feature
status: backlog
milestone: v0.2
depends_on:
- 17
- 20
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: s
area: app
---

`r` re-deals the current seed from the start. Distinct from undo-to-the-start
only in that it clears the history, and distinct from `n` in that it is the
same cards. With a confirm prompt when a game is in progress, like `n`.

## Acceptance criteria

- [ ] After `r`, the first draw yields the same card as it did originally
- [ ] History and counters reset
