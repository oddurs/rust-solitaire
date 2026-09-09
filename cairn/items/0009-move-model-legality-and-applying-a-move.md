---
id: 9
title: Move model, legality, and applying a move
type: feature
status: planned
milestone: v0.1
depends_on:
- 8
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: l
area: engine
---

## Problem

The rules. Everything the player can do is a move; the engine must say
whether a move is legal, apply it if so, and refuse it otherwise, in a way the
UI cannot get wrong.

## Proposal

```rust
pub enum Location { Stock, Waste, Foundation(Suit), Tableau(usize) }

pub enum Move {
    Draw,                                             // stock -> waste, or recycle
    Transfer { from: Location, to: Location, count: usize },
}

impl Game {
    pub fn is_legal(&self, m: Move) -> Result<(), Illegal>;
    pub fn apply(&mut self, m: Move) -> Result<(), Illegal>;
    pub fn legal_moves(&self) -> Vec<Move>;
}
```

Rules encoded in `is_legal`:

- Foundation accepts an ace on empty, otherwise the same suit one rank higher.
  Only a single card moves to a foundation.
- Tableau accepts a king on empty, otherwise an opposite-colour card one rank
  lower than the top face-up card. A run of `count` face-up cards may move
  together if the run is already valid (alternating colours, descending).
- Waste and foundation give only their top card. Stock gives nothing except
  through `Draw`.
- Moving the last face-up card off a pile turns the next card face up.

`Illegal` is an enum with a reason (`EmptySource`, `WrongColor`, `WrongRank`,
`NeedsKing`, `NeedsAce`, `TooManyCards`, ...) so the UI can explain, not just
beep.

`legal_moves()` is deliberately the brute-force product of sources and
destinations filtered by `is_legal`; it is small enough that clarity beats
cleverness, and it becomes the basis of hints and the solver.

Draw semantics (how many cards, and recycling) are in their own item because
draw-three changes them.

## Acceptance criteria

- [ ] Table-driven tests for every rule above, each with a legal and an illegal case
- [ ] `apply` on an illegal move leaves the game unchanged and returns the reason
- [ ] Moving a run keeps its order
- [ ] Turning the exposed card face up happens on tableau-to-anything moves
- [ ] `check_invariants()` holds after every applied move in the tests
