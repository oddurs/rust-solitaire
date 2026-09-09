---
id: 21
title: Draw-three mode
type: feature
status: backlog
milestone: v0.2
depends_on:
- 10
- 17
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: m
area: engine
---

## Problem

Draw-three is the traditional rule and the one many players consider the real
game. The engine already carries `DrawRule { count }`; this makes `3`
reachable and correct.

## Proposal

`--draw 3` on the command line (this is the flag that tips argument parsing
over to `clap`; do that here). Engine: `Draw` moves up to three cards, keeping
their order, and only the top of the waste is playable. Recycling turns the
whole waste over so the same triples come up again. UI: the waste shows the
last three cards fanned so the player can see what is coming.

Optionally limit recycles (`--passes N`); leave it unlimited by default and
note the decision in this item if it is added.

## Acceptance criteria

- [ ] With fewer than three cards left, `Draw` moves what remains
- [ ] Recycle order matches the well-known draw-three behaviour (tests against a worked example)
- [ ] Waste rendering shows up to three cards; only the top is selectable
