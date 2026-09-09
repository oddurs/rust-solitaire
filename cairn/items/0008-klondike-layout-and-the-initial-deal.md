---
id: 8
title: Klondike layout and the initial deal
type: feature
status: done
milestone: v0.1
depends_on:
- 7
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: m
area: engine
---

## Problem

Klondike has a specific table: a stock, a waste, four foundations, and seven
tableau piles dealt 1..7 with only the top card face up. That state, and its
invariants, is the thing every rule reads and writes.

## Proposal

```rust
pub struct Game {
    stock: Vec<Card>,              // face down, top is last
    waste: Vec<Card>,              // face up, top is last
    foundations: [Vec<Card>; 4],   // indexed by Suit
    tableau: [Pile; 7],
}
pub struct Pile { cards: Vec<Card>, face_up: usize }  // last `face_up` cards are visible
```

`Game::deal(deck: Deck) -> Game` deals left to right, one card per pile per
round, so pile `i` has `i + 1` cards with the last one face up; the remaining
24 go to the stock.

Fields are private. Read access is through methods that return slices, so the
UI can render but cannot corrupt state. The invariant "52 cards, no
duplicates, every face-down card is below every face-up card in its pile" is
checked by a `debug_assert!`-backed `Game::check_invariants()` used by tests.

Foundations are indexed by suit rather than "first empty one", which removes a
whole class of ambiguity from moves and rendering.

## Acceptance criteria

- [ ] After a deal: pile sizes 1..7, one face-up card each, stock of 24, empty waste and foundations
- [ ] `check_invariants()` passes after a deal
- [ ] No public field on `Game` or `Pile`
