---
id: 1
key: v0.1
title: Playable Klondike
type: milestone
status: planned
created: 2026-09-08
updated: 2026-09-08
priority: p2
due: 2026-10-06
---

A correct Klondike engine you can play to a win in the terminal, on a reproducible deal.

The smallest thing that is honestly a solitaire game: a correct Klondike rules
engine you can play to a win in the terminal with the keyboard, on a seeded deal
you can reproduce.

Everything here is a release blocker or it is not here. No undo, no scoring, no
options: those are v0.2, and they are easier to build once the engine and the
board exist and have been played with for a week.

The engine ships as a library with zero terminal dependencies so that every rule
is testable without a TTY and so the same engine can drive other front ends or
a solver later.

Four weeks because the engine is a known quantity (52 cards, four rule families)
and the UI is one screen. The date is a forcing function for scope, not a
promise.
