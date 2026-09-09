---
id: 35
title: Solver, and a --winnable flag
type: feature
status: backlog
milestone: v0.3
depends_on:
- 12
- 21
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: xl
area: engine
---

## Problem

Roughly one in five draw-one deals cannot be won even with perfect information,
and rather more under draw-three. Offering an unwinnable deal to a player who
cannot know it is unkind. The engine was separated from the terminal so this
could exist.

## Proposal

A depth-first search with transposition table over `Game` states, using
`legal_moves()`, safe auto-moves to prune, and a move-ordering heuristic
(foundation first, expose face-down cards next). Perfect-information (the
solver sees face-down cards), which is what "winnable" means for a fixed seed.
Bound it: a node budget and a time budget, returning `Winnable`, `Unwinnable`
or `Unknown`.

`--winnable` at the command line draws seeds until the solver returns
`Winnable` within budget, then deals it. The solver is `pub` in the library so
tests can pick known-winnable seeds for the UI items instead of guessing.

This is the largest item in the plan. If the naive search is too slow, the
fallback is the well-known trick of solving draw-one only and treating
draw-three as unknown. Record whichever happens here.

## Acceptance criteria

- [ ] Known-winnable and known-unwinnable seeds (from the literature or hand-verified) are classified correctly
- [ ] Budget is respected; `Unknown` is returned rather than hanging
- [ ] `--winnable` never takes more than a couple of seconds on a laptop
