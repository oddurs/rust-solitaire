---
id: 5
title: Split the crate into a library and a thin binary
type: chore
status: planned
milestone: v0.1
labels:
- architecture
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: s
area: engine
---

## Problem

Everything lives in `src/main.rs`. A rules engine that is only reachable
through a binary cannot be unit-tested without a terminal, cannot be reused by
a solver or another front end, and drags `ratatui` into every test compile.

## Proposal

`src/lib.rs` exposes the engine (`card`, `deck`, `klondike`) and, later, the
`app` state machine. `src/main.rs` parses arguments, sets up the terminal, and
calls into the library. Nothing under `src/lib.rs` may depend on `ratatui` or
`crossterm`; a test asserts the lib target builds with no default features
once there are features to speak of.

Module layout to start with:

```
src/lib.rs        pub mod card; pub mod deck; pub mod klondike;
src/card.rs       Suit, Rank, Color, Card
src/deck.rs       Deck, shuffle
src/klondike.rs   Game, Move, rules
src/main.rs       fn main()
```

Split `klondike` into a directory module when it passes ~500 lines; not before.

## Acceptance criteria

- [ ] `cargo test --lib` runs engine tests without linking a terminal crate
- [ ] `src/main.rs` is under 50 lines and contains no game rules
- [ ] `scripts/task check` is green
