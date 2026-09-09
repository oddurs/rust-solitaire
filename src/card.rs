//! The vocabulary every rule is written in: suits, colours, ranks and cards.
//!
//! Nothing here knows how a card is drawn on screen. `Debug` prints `A♠` for
//! convenience in test output; the UI decides between Unicode and ASCII.

use std::fmt;

/// One of the four suits, in the order foundations are laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    /// Every suit, in foundation order.
    pub const ALL: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    /// Red for hearts and diamonds, black otherwise. Derived, never stored.
    #[must_use]
    pub const fn color(self) -> Color {
        match self {
            Suit::Diamonds | Suit::Hearts => Color::Red,
            Suit::Clubs | Suit::Spades => Color::Black,
        }
    }

    /// Position of this suit in [`Suit::ALL`], used to index foundations.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// The Unicode suit symbol.
    #[must_use]
    pub const fn symbol(self) -> char {
        match self {
            Suit::Clubs => '♣',
            Suit::Diamonds => '♦',
            Suit::Hearts => '♥',
            Suit::Spades => '♠',
        }
    }

    /// The ASCII letter used where the symbol cannot be shown.
    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
        }
    }
}

/// The colour of a suit. Tableau runs alternate colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Black,
}

/// Card rank, ace low. Ordering follows Klondike: ace < two < … < king.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    /// Every rank, ace first.
    pub const ALL: [Rank; 13] = [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];

    /// The rank's numeric value, ace being 1 and king 13.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// The rank with the given value, if there is one.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Rank> {
        if value == 0 || value > 13 {
            return None;
        }
        Some(Rank::ALL[(value - 1) as usize])
    }

    /// The rank one lower, or `None` for the ace.
    #[must_use]
    pub const fn pred(self) -> Option<Rank> {
        Rank::from_value(self.value() - 1)
    }

    /// The rank one higher, or `None` for the king.
    #[must_use]
    pub const fn succ(self) -> Option<Rank> {
        Rank::from_value(self.value() + 1)
    }

    /// The short label: `A`, `2` … `10`, `J`, `Q`, `K`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
}

/// A playing card. Cheap to copy; there are only ever 52 distinct values.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// A card of the given rank and suit.
    #[must_use]
    pub const fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    /// The colour of the card's suit.
    #[must_use]
    pub const fn color(self) -> Color {
        self.suit.color()
    }

    /// All 52 cards in a fixed order: suits in [`Suit::ALL`] order, ace to king
    /// within each suit. This is what an unshuffled deck contains.
    pub fn all() -> impl Iterator<Item = Card> {
        Suit::ALL
            .into_iter()
            .flat_map(|suit| Rank::ALL.into_iter().map(move |rank| Card::new(rank, suit)))
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank.label(), self.suit.symbol())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn there_are_52_distinct_cards() {
        let cards: Vec<Card> = Card::all().collect();
        let distinct: HashSet<Card> = cards.iter().copied().collect();
        assert_eq!(cards.len(), 52);
        assert_eq!(distinct.len(), 52);
    }

    #[test]
    fn colours_follow_suits() {
        assert_eq!(Suit::Hearts.color(), Color::Red);
        assert_eq!(Suit::Diamonds.color(), Color::Red);
        assert_eq!(Suit::Clubs.color(), Color::Black);
        assert_eq!(Suit::Spades.color(), Color::Black);
    }

    #[test]
    fn rank_neighbours_stop_at_the_ends() {
        assert_eq!(Rank::King.succ(), None);
        assert_eq!(Rank::Ace.pred(), None);
        assert_eq!(Rank::Ace.succ(), Some(Rank::Two));
        assert_eq!(Rank::King.pred(), Some(Rank::Queen));
        assert_eq!(Rank::Ten.succ(), Some(Rank::Jack));
    }

    #[test]
    fn ranks_order_ace_low() {
        assert!(Rank::Ace < Rank::Two);
        assert!(Rank::Ten < Rank::Jack);
        assert!(Rank::Queen < Rank::King);
        for (i, rank) in Rank::ALL.iter().enumerate() {
            assert_eq!(usize::from(rank.value()), i + 1);
            assert_eq!(Rank::from_value(rank.value()), Some(*rank));
        }
        assert_eq!(Rank::from_value(0), None);
        assert_eq!(Rank::from_value(14), None);
    }

    #[test]
    fn suit_index_matches_foundation_order() {
        for (i, suit) in Suit::ALL.iter().enumerate() {
            assert_eq!(suit.index(), i);
        }
    }

    #[test]
    fn debug_is_compact() {
        assert_eq!(format!("{:?}", Card::new(Rank::Ace, Suit::Spades)), "A♠");
        assert_eq!(format!("{:?}", Card::new(Rank::Ten, Suit::Hearts)), "10♥");
    }
}
