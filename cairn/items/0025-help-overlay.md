---
id: 25
title: Help overlay
type: feature
status: backlog
milestone: v0.2
depends_on:
- 16
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: s
area: ui
---

`?` opens a centred box over the board listing the bindings and the rules in
one screen; any key closes it. The content is generated from the same binding
table the terminal layer uses, so it cannot drift from the code.

While it is open, the timer (once it exists) pauses.

## Acceptance criteria

- [ ] Rendered from the binding table, not a hand-written string
- [ ] Fits in 80x24 and scrolls if the terminal is shorter
