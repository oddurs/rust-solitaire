---
id: 34
title: Persistent statistics
type: feature
status: backlog
milestone: v0.3
depends_on:
- 30
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: m
area: app
---

## Problem

Nothing is remembered between runs. Games played, games won, best time, best
score, current and longest streak are the reason people keep playing.

## Proposal

A small JSON or TOML file in the platform data directory (`directories`,
already present after the config item), written on win and on abandoning a
game, read at startup. Per draw mode, since draw-three stats are not
comparable to draw-one. A `--stats` flag prints them and exits; `S` in the
game shows them in an overlay.

Corrupt or unreadable stats are reported once and the game still starts; never
refuse to play because a file is bad.

## Acceptance criteria

- [ ] Round-trips through a temp directory in tests
- [ ] A truncated file logs a warning and yields empty stats
- [ ] Abandoned games count as played, not won
