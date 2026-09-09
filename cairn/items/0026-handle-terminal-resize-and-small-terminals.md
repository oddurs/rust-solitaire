---
id: 26
title: Handle terminal resize and small terminals
type: feature
status: backlog
milestone: v0.2
depends_on:
- 15
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: s
area: ui
---

`Resize` events must redraw at the new size, and the too-small message from the
board item must go away as soon as the terminal grows. Below 80 columns,
tableau cards can overlap vertically (show only the top two lines of each) to
buy space before giving up entirely; the exact thresholds get decided here and
written down.

## Acceptance criteria

- [ ] Shrinking then growing the terminal returns to a correct board without input
- [ ] Snapshots at 80x24, 100x30 and 60x20
