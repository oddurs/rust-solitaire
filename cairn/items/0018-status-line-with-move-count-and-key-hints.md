---
id: 18
title: Status line with move count and key hints
type: feature
status: planned
milestone: v0.1
depends_on:
- 16
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: s
area: ui
---

One line at the bottom: the seed, the move count, and the four or five keys
that matter right now (`space select`, `d draw`, `f foundation`, `n new`,
`q quit`). It doubles as the message line for `Illegal` reasons, which replace
the hints for a moment and then fade back.

Move count is maintained by `App`, counts applied moves only, and is the seed
for scoring in v0.2.

## Acceptance criteria

- [ ] Count increments on applied moves and not on rejected ones
- [ ] The line fits 80 columns
