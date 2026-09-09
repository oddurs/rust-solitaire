//! A deck of 52 cards and the seeded shuffle that makes every deal reproducible.

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::card::Card;

/// A stack of cards. The top of the deck is the end of the vector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// The 52 cards in [`Card::all`] order, unshuffled.
    #[must_use]
    pub fn new() -> Deck {
        Deck {
            cards: Card::all().collect(),
        }
    }

    /// A full deck shuffled by `seed`. The same seed always yields the same
    /// order, which is what makes a deal something you can come back to.
    ///
    /// The shuffle is Fisher–Yates driven by [`StdRng`]. If a `rand` major
    /// upgrade ever changes that generator, every seed will silently mean a
    /// different deal; see the note on the deck item in the backlog.
    #[must_use]
    pub fn shuffled(seed: u64) -> Deck {
        let mut deck = Deck::new();
        let mut rng = StdRng::seed_from_u64(seed);
        deck.cards.shuffle(&mut rng);
        deck
    }

    /// Take the top card, if any.
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Cards remaining, bottom first.
    #[must_use]
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Number of cards remaining.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Whether the deck has been drawn empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}

/// A seed drawn from the operating system, for deals nobody asked for by
/// number. Shown to the player so the deal can be replayed later.
#[must_use]
pub fn random_seed() -> u64 {
    rand::random()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn same_seed_same_order() {
        assert_eq!(Deck::shuffled(42), Deck::shuffled(42));
        assert_ne!(Deck::shuffled(42), Deck::shuffled(43));
    }

    #[test]
    fn shuffling_keeps_every_card_once() {
        let deck = Deck::shuffled(7);
        let distinct: HashSet<Card> = deck.cards().iter().copied().collect();
        assert_eq!(deck.len(), 52);
        assert_eq!(distinct.len(), 52);
    }

    #[test]
    fn a_new_deck_is_in_canonical_order() {
        let deck = Deck::new();
        let expected: Vec<Card> = Card::all().collect();
        assert_eq!(deck.cards(), expected.as_slice());
    }

    #[test]
    fn drawing_takes_from_the_top_until_empty() {
        let mut deck = Deck::new();
        let top = *deck.cards().last().unwrap();
        assert_eq!(deck.draw(), Some(top));
        assert_eq!(deck.len(), 51);
        while deck.draw().is_some() {}
        assert!(deck.is_empty());
        assert_eq!(deck.draw(), None);
    }
}
