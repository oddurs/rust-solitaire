---
id: 6
title: Card, Rank, Suit and Color types
type: feature
status: planned
milestone: v0.1
depends_on:
- 5
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: s
area: engine
---

## Problem

There is no vocabulary yet. Every rule in Klondike is stated in terms of rank
order, suit, and the red/black colour of a card, so those need to be types the
compiler checks rather than integers with a convention.

## Proposal

```rust
pub enum Suit { Clubs, Diamonds, Hearts, Spades }
pub enum Color { Red, Black }
pub enum Rank { Ace = 1, Two, ..., King = 13 }
pub struct Card { pub rank: Rank, pub suit: Suit }
```

- `Suit::color()` derives colour; it is never stored.
- `Rank` is `Ord` in ace-low order, and has `pred()`/`succ()` returning
  `Option<Rank>` so "one lower" is expressible without arithmetic on integers.
- `Card` is `Copy`, `Eq`, `Hash`, and `Debug` prints `A♠`. `Display` is left to
  the UI, which decides between Unicode and ASCII.
- `Card::all()` yields the 52 cards in a fixed order, which is what `Deck`
  seeds from.

No `rand`, no `serde` here. Derives only.

## Acceptance criteria

- [ ] 52 distinct cards, each appearing exactly once in `Card::all()`
- [ ] Hearts and diamonds are red; clubs and spades are black
- [ ] `Rank::King.succ()` is `None`, `Rank::Ace.pred()` is `None`
- [ ] Rank ordering matches Klondike (ace low)
