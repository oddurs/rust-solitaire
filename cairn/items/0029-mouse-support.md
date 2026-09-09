---
id: 29
title: Mouse support
type: feature
status: backlog
milestone: v0.2
depends_on:
- 16
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: m
area: ui
---

Click a pile to select, click a destination to drop, double-click to send to
the foundation, click the stock to draw. Enable mouse capture in the terminal
lifecycle guard and translate `MouseEvent` positions into `Location`s via the
layout the board widget computed on the last draw (the widget records each
pile's `Rect`).

Keep it optional (`--no-mouse`) because mouse capture breaks terminal text
selection, which some people care about more.

## Acceptance criteria

- [ ] Hit-testing is a pure function of the recorded layout and is unit tested
- [ ] Every mouse action maps to an existing `Input`; no new game logic
