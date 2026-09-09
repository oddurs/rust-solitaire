---
id: 17
title: New game, quit, and a --seed flag
type: feature
status: planned
milestone: v0.1
depends_on:
- 16
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: s
area: app
---

## Problem

The binary has no way in and no way out. It also needs to expose the seed,
because reproducible deals are the reason the shuffle takes one.

## Proposal

`rust-solitaire [--seed N]`. No seed: draw one from the OS and show it in the
status line so the player can come back to it. Argument parsing with
`std::env::args` for now; `clap` is justified when there are more than two
flags, which v0.2 will bring, and adopting it then is a contained change.

`n` deals a new random game after a confirm-if-in-progress prompt; `q` quits.
Restarting the *same* seed is a v0.2 item.

## Acceptance criteria

- [ ] `--seed 42` twice gives the same deal
- [ ] The seed is visible while playing
- [ ] `--help` and an unparseable seed exit non-zero with a one-line message
