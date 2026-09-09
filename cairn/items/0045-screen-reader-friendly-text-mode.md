---
id: 45
title: Screen-reader friendly text mode
type: feature
status: backlog
milestone: later
depends_on:
- 15
created: 2026-09-08
updated: 2026-09-08
priority: p3
effort: l
area: ui
---

A non-TUI mode that describes the table in plain lines and takes moves as
typed commands (`w f`, `t3 t5 2`). Also the easiest way to play over a
serial console. The engine and `App` are ready for it; the terminal layer is
what gets swapped.
