---
id: 32
title: Snapshot tests for rendering
type: test
status: backlog
milestone: v0.2
labels:
- testing
depends_on:
- 15
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: m
area: ui
---

The board item has one snapshot. Rendering will change in almost every v0.2
item, and reviewing pixel-exact terminal output by eye does not scale.

Use `ratatui::backend::TestBackend` and `insta` (dev-dependency) to snapshot
the buffer as text for a fixed set of positions: fresh deal, mid-game with a
selection, a won game, too-small terminal, ASCII mode. Positions are built by
replaying a fixed move list on a fixed seed, so a snapshot is reproducible
from the engine alone.

`cargo insta review` is the workflow for intentional changes; a PR that
changes a snapshot must say why.

## Acceptance criteria

- [ ] Five snapshots checked in, all passing under `scripts/task test`
- [ ] CONTRIBUTING.md explains how to update a snapshot
