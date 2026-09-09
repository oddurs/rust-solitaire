---
id: 24
title: Standard scoring and a timer
type: feature
status: backlog
milestone: v0.2
depends_on:
- 18
- 20
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: m
area: app
---

## Problem

A move counter is not a score. People compare solitaire results using the
Windows-era standard scoring, and a timer is what makes a game a challenge.

## Proposal

Standard scoring, kept in `App` (not the engine, which stays about legality):

| Event | Points |
|---|---|
| waste to tableau | 5 |
| waste to foundation | 10 |
| tableau to foundation | 10 |
| turn over a tableau card | 5 |
| foundation to tableau | -15 |
| recycle the waste (draw-one) | -100 |
| undo | -2 |

Time bonus on win: `700_000 / seconds` when over 30 seconds, as the original
did. Score never goes below zero. The timer starts on the first move, not on
deal, and pauses while the help overlay is up.

Vegas scoring is a `later` item; its rules are different enough to be a mode,
not an option.

## Acceptance criteria

- [ ] Each event above has a test
- [ ] Timer starts on first move and is shown as `mm:ss`
- [ ] Score and time appear on the status line and the win screen
