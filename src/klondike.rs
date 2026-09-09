//! Klondike: the table, the moves, and the rules that decide between them.
//!
//! A [`Game`] owns every card. The only way to change it is [`Game::apply`],
//! which refuses anything [`Game::is_legal`] would refuse, so the table can
//! never reach a state the rules did not produce.

use std::fmt;

use crate::card::{Card, Rank, Suit};
use crate::deck::Deck;

/// Number of tableau piles.
pub const TABLEAU_PILES: usize = 7;

/// How many cards a draw turns over from the stock.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DrawMode {
    /// One card at a time, unlimited passes: the casual rule.
    #[default]
    One,
    /// Three at a time, only the top playable: the traditional rule.
    Three,
}

impl DrawMode {
    /// Cards turned per draw.
    #[must_use]
    pub const fn count(self) -> usize {
        match self {
            DrawMode::One => 1,
            DrawMode::Three => 3,
        }
    }
}

/// A tableau pile: some face-down cards with a face-up run on top.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pile {
    cards: Vec<Card>,
    face_up: usize,
}

impl Pile {
    /// Every card, bottom first.
    #[must_use]
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// The face-up run, bottom first. Empty when the pile is empty.
    #[must_use]
    pub fn face_up(&self) -> &[Card] {
        &self.cards[self.cards.len() - self.face_up..]
    }

    /// How many cards are still face down.
    #[must_use]
    pub fn face_down_count(&self) -> usize {
        self.cards.len() - self.face_up
    }

    /// The top card, if any. Always face up.
    #[must_use]
    pub fn top(&self) -> Option<Card> {
        self.cards.last().copied()
    }

    /// Number of cards in the pile.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Whether the pile has no cards at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

/// Somewhere a card can be.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Location {
    Stock,
    Waste,
    Foundation(Suit),
    Tableau(usize),
}

/// Something the player can do.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    /// Turn cards from the stock onto the waste, or turn the waste back over.
    Draw,
    /// Move `count` cards from the top of `from` onto `to`.
    Transfer {
        from: Location,
        to: Location,
        count: usize,
    },
}

/// Why a move was refused. Meant to be shown to the player.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Illegal {
    /// The source has no cards.
    EmptySource,
    /// Cards cannot be taken from the stock except by drawing.
    NotASource,
    /// Nothing can be placed on the stock or the waste.
    NotADestination,
    /// Source and destination are the same pile.
    SameLocation,
    /// No tableau pile has that index.
    NoSuchPile,
    /// More cards than the source can give, or more than one to a foundation.
    TooManyCards,
    /// Only an ace starts a foundation.
    NeedsAce,
    /// Only a king starts an empty tableau pile.
    NeedsKing,
    /// A foundation takes one suit only.
    WrongSuit,
    /// Tableau runs alternate colours.
    WrongColor,
    /// The rank does not continue the pile.
    WrongRank,
    /// Both stock and waste are empty.
    NothingToDraw,
}

impl fmt::Display for Illegal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Illegal::EmptySource => "nothing there to move",
            Illegal::NotASource => "draw from the stock instead",
            Illegal::NotADestination => "cards cannot go there",
            Illegal::SameLocation => "that is where it already is",
            Illegal::NoSuchPile => "no such pile",
            Illegal::TooManyCards => "cannot move that many cards",
            Illegal::NeedsAce => "only an ace can start a foundation",
            Illegal::NeedsKing => "only a king can start an empty pile",
            Illegal::WrongSuit => "wrong suit for that foundation",
            Illegal::WrongColor => "colours must alternate",
            Illegal::WrongRank => "rank must be one lower",
            Illegal::NothingToDraw => "nothing left to draw",
        })
    }
}

impl std::error::Error for Illegal {}

/// What an applied move did beyond moving cards. Scoring reads this.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    /// A face-down tableau card was turned face up.
    pub revealed: bool,
    /// The waste was turned back over to become the stock.
    pub recycled: bool,
}

/// A game of Klondike in progress.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Game {
    stock: Vec<Card>,
    waste: Vec<Card>,
    foundations: [Vec<Card>; 4],
    tableau: [Pile; TABLEAU_PILES],
    draw_mode: DrawMode,
}

impl Game {
    /// Deal a game with the casual draw-one rule.
    #[must_use]
    pub fn deal(deck: Deck) -> Game {
        Game::new(deck, DrawMode::One)
    }

    /// Deal a game: seven piles of 1..=7 cards, top card up, the rest to the
    /// stock. Cards are dealt one per pile per round, left to right, as at a
    /// table.
    #[must_use]
    pub fn new(mut deck: Deck, draw_mode: DrawMode) -> Game {
        let mut tableau: [Pile; TABLEAU_PILES] = Default::default();
        for round in 0..TABLEAU_PILES {
            for pile in tableau.iter_mut().skip(round) {
                if let Some(card) = deck.draw() {
                    pile.cards.push(card);
                }
            }
        }
        for pile in &mut tableau {
            pile.face_up = usize::from(!pile.cards.is_empty());
        }
        Game {
            stock: deck.cards().to_vec(),
            waste: Vec::new(),
            foundations: Default::default(),
            tableau,
            draw_mode,
        }
    }

    /// The stock, bottom first; the last card is the next to be drawn.
    #[must_use]
    pub fn stock(&self) -> &[Card] {
        &self.stock
    }

    /// The waste, bottom first; only the last card is playable.
    #[must_use]
    pub fn waste(&self) -> &[Card] {
        &self.waste
    }

    /// The foundation for a suit, ace first.
    #[must_use]
    pub fn foundation(&self, suit: Suit) -> &[Card] {
        &self.foundations[suit.index()]
    }

    /// The seven tableau piles.
    #[must_use]
    pub fn tableau(&self) -> &[Pile; TABLEAU_PILES] {
        &self.tableau
    }

    /// The draw rule this game was dealt under.
    #[must_use]
    pub fn draw_mode(&self) -> DrawMode {
        self.draw_mode
    }

    /// Every foundation is complete.
    #[must_use]
    pub fn is_won(&self) -> bool {
        self.foundations.iter().all(|f| f.len() == 13)
    }

    /// Whether `m` may be played now, or why not.
    ///
    /// # Errors
    ///
    /// The [`Illegal`] reason, suitable for showing to the player.
    pub fn is_legal(&self, m: Move) -> Result<(), Illegal> {
        match m {
            Move::Draw => {
                if self.stock.is_empty() && self.waste.is_empty() {
                    Err(Illegal::NothingToDraw)
                } else {
                    Ok(())
                }
            }
            Move::Transfer { from, to, count } => {
                if from == to {
                    return Err(Illegal::SameLocation);
                }
                let run = self.source_run(from, count)?;
                self.accepts(to, run)
            }
        }
    }

    /// Play `m`, or leave the game untouched and say why not.
    ///
    /// # Errors
    ///
    /// The [`Illegal`] reason; nothing has changed when this returns `Err`.
    pub fn apply(&mut self, m: Move) -> Result<Outcome, Illegal> {
        self.is_legal(m)?;
        match m {
            Move::Draw => Ok(self.draw()),
            Move::Transfer { from, to, count } => {
                let cards = self.take(from, count);
                self.put(to, cards);
                Ok(Outcome {
                    revealed: self.reveal(from),
                    recycled: false,
                })
            }
        }
    }

    /// Every move that [`Game::apply`] would accept right now, transfers
    /// first and `Draw` last. Deliberately brute force: sources times
    /// destinations is a few hundred checks at most, and it is the ground
    /// truth that hints and the solver build on.
    #[must_use]
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for (from, max_count) in self.sources() {
            for count in 1..=max_count {
                for to in Self::destinations() {
                    let m = Move::Transfer { from, to, count };
                    if self.is_legal(m).is_ok() {
                        moves.push(m);
                    }
                }
            }
        }
        if self.is_legal(Move::Draw).is_ok() {
            moves.push(Move::Draw);
        }
        moves
    }

    /// No transfer is possible now, nor after any number of draws. Advisory:
    /// the player may still want to undo. A full pass through the stock is
    /// simulated on a copy, so this costs as much as the stock is long.
    #[must_use]
    pub fn is_stuck(&self) -> bool {
        if self.is_won() {
            return false;
        }
        let mut probe = self.clone();
        let passes = probe.stock.len() + probe.waste.len() + 1;
        for _ in 0..=passes {
            if probe.legal_moves().iter().any(|m| *m != Move::Draw) {
                return false;
            }
            if probe.apply(Move::Draw).is_err() {
                break;
            }
        }
        true
    }

    /// Check that the table could have been produced by the rules.
    ///
    /// # Errors
    ///
    /// A description of the first broken invariant.
    pub fn check_invariants(&self) -> Result<(), String> {
        let mut seen = std::collections::HashSet::new();
        let all = self
            .stock
            .iter()
            .chain(&self.waste)
            .chain(self.foundations.iter().flatten())
            .chain(self.tableau.iter().flat_map(|p| p.cards.iter()));
        let mut total = 0;
        for card in all {
            total += 1;
            if !seen.insert(*card) {
                return Err(format!("duplicate card {card:?}"));
            }
        }
        if total != 52 {
            return Err(format!("{total} cards on the table, expected 52"));
        }
        for (i, pile) in self.tableau.iter().enumerate() {
            if pile.face_up > pile.cards.len() {
                return Err(format!("pile {i} has more face-up than cards"));
            }
            if !pile.cards.is_empty() && pile.face_up == 0 {
                return Err(format!("pile {i} has cards but none face up"));
            }
            if !is_run(pile.face_up()) {
                return Err(format!("pile {i} face-up cards are not a run"));
            }
        }
        for suit in Suit::ALL {
            for (i, card) in self.foundation(suit).iter().enumerate() {
                if card.suit != suit || usize::from(card.rank.value()) != i + 1 {
                    return Err(format!("foundation {suit:?} is out of order at {card:?}"));
                }
            }
        }
        Ok(())
    }

    // ── internals ────────────────────────────────────────────────────────

    /// The cards a transfer would take, bottom first.
    fn source_run(&self, from: Location, count: usize) -> Result<&[Card], Illegal> {
        if count == 0 {
            return Err(Illegal::TooManyCards);
        }
        let available: &[Card] = match from {
            Location::Stock => return Err(Illegal::NotASource),
            Location::Waste => top_only(&self.waste),
            Location::Foundation(suit) => top_only(self.foundation(suit)),
            Location::Tableau(i) => self.pile(i)?.face_up(),
        };
        if available.is_empty() {
            return Err(Illegal::EmptySource);
        }
        if count > available.len() {
            return Err(Illegal::TooManyCards);
        }
        Ok(&available[available.len() - count..])
    }

    /// Whether `to` takes `run`, whose first card lands on the pile.
    fn accepts(&self, to: Location, run: &[Card]) -> Result<(), Illegal> {
        let bottom = run[0];
        match to {
            Location::Stock | Location::Waste => Err(Illegal::NotADestination),
            Location::Foundation(suit) => {
                if run.len() != 1 {
                    return Err(Illegal::TooManyCards);
                }
                if bottom.suit != suit {
                    return Err(Illegal::WrongSuit);
                }
                match self.foundation(suit).last() {
                    None if bottom.rank == Rank::Ace => Ok(()),
                    None => Err(Illegal::NeedsAce),
                    Some(top) if top.rank.succ() == Some(bottom.rank) => Ok(()),
                    Some(_) => Err(Illegal::WrongRank),
                }
            }
            Location::Tableau(i) => match self.pile(i)?.top() {
                None if bottom.rank == Rank::King => Ok(()),
                None => Err(Illegal::NeedsKing),
                Some(top) if top.color() == bottom.color() => Err(Illegal::WrongColor),
                Some(top) if top.rank.pred() == Some(bottom.rank) => Ok(()),
                Some(_) => Err(Illegal::WrongRank),
            },
        }
    }

    fn pile(&self, i: usize) -> Result<&Pile, Illegal> {
        self.tableau.get(i).ok_or(Illegal::NoSuchPile)
    }

    /// Remove `count` cards from a legal source. Callers have checked legality.
    fn take(&mut self, from: Location, count: usize) -> Vec<Card> {
        let stack: &mut Vec<Card> = match from {
            Location::Waste => &mut self.waste,
            Location::Foundation(suit) => &mut self.foundations[suit.index()],
            Location::Tableau(i) => {
                let pile = &mut self.tableau[i];
                pile.face_up -= count;
                &mut pile.cards
            }
            Location::Stock => unreachable!("stock is never a transfer source"),
        };
        stack.split_off(stack.len() - count)
    }

    /// Place cards on a legal destination. Callers have checked legality.
    fn put(&mut self, to: Location, cards: Vec<Card>) {
        match to {
            Location::Foundation(suit) => self.foundations[suit.index()].extend(cards),
            Location::Tableau(i) => {
                let pile = &mut self.tableau[i];
                pile.face_up += cards.len();
                pile.cards.extend(cards);
            }
            Location::Stock | Location::Waste => {
                unreachable!("stock and waste are never transfer destinations");
            }
        }
    }

    /// Turn the top card of a tableau pile face up if nothing is. True if it did.
    fn reveal(&mut self, from: Location) -> bool {
        if let Location::Tableau(i) = from {
            let pile = &mut self.tableau[i];
            if pile.face_up == 0 && !pile.cards.is_empty() {
                pile.face_up = 1;
                return true;
            }
        }
        false
    }

    /// Turn cards from the stock, or recycle the waste. Legality checked.
    fn draw(&mut self) -> Outcome {
        if self.stock.is_empty() {
            self.stock = std::mem::take(&mut self.waste);
            self.stock.reverse();
            return Outcome {
                revealed: false,
                recycled: true,
            };
        }
        for _ in 0..self.draw_mode.count() {
            match self.stock.pop() {
                Some(card) => self.waste.push(card),
                None => break,
            }
        }
        Outcome::default()
    }

    /// Transfer sources with the most cards each could give.
    fn sources(&self) -> Vec<(Location, usize)> {
        let mut v = vec![(Location::Waste, usize::from(!self.waste.is_empty()))];
        v.extend(
            self.tableau
                .iter()
                .enumerate()
                .map(|(i, p)| (Location::Tableau(i), p.face_up)),
        );
        v.extend(Suit::ALL.into_iter().map(|s| {
            (
                Location::Foundation(s),
                usize::from(!self.foundation(s).is_empty()),
            )
        }));
        v
    }

    fn destinations() -> impl Iterator<Item = Location> {
        Suit::ALL
            .into_iter()
            .map(Location::Foundation)
            .chain((0..TABLEAU_PILES).map(Location::Tableau))
    }
}

/// The top card as a slice, or nothing: waste and foundations give one card.
fn top_only(cards: &[Card]) -> &[Card] {
    &cards[cards.len().saturating_sub(1)..]
}

/// Whether the cards form a valid face-up run: alternating colours, each one
/// rank below the last. Empty and single cards are runs.
fn is_run(cards: &[Card]) -> bool {
    cards
        .windows(2)
        .all(|w| w[0].color() != w[1].color() && w[0].rank.pred() == Some(w[1].rank))
}

#[cfg(test)]
pub(crate) mod fixtures {
    //! Hand-built positions for tests, described as strings like `"7♥ K♠"`.

    use super::*;

    pub(crate) fn card(s: &str) -> Card {
        let (rank, suit) = s.split_at(s.len() - '♠'.len_utf8());
        let rank = Rank::ALL
            .into_iter()
            .find(|r| r.label() == rank)
            .unwrap_or_else(|| panic!("bad rank in {s}"));
        let suit = Suit::ALL
            .into_iter()
            .find(|su| su.symbol().to_string() == suit)
            .unwrap_or_else(|| panic!("bad suit in {s}"));
        Card::new(rank, suit)
    }

    pub(crate) fn cards(s: &str) -> Vec<Card> {
        s.split_whitespace().map(card).collect()
    }

    /// A position. Tableau piles are `(face_down, face_up)`, bottom first.
    /// The cards named need not add up to 52; invariant checks are skipped
    /// for positions built here unless the test asks.
    pub(crate) fn position(
        stock: &str,
        waste: &str,
        foundations: [&str; 4],
        tableau: [(&str, &str); TABLEAU_PILES],
    ) -> Game {
        let mut game = Game {
            stock: cards(stock),
            waste: cards(waste),
            foundations: foundations.map(cards),
            tableau: Default::default(),
            draw_mode: DrawMode::One,
        };
        for (i, (down, up)) in tableau.into_iter().enumerate() {
            let mut pile = cards(down);
            let up = cards(up);
            let n = up.len();
            pile.extend(up);
            game.tableau[i] = Pile {
                cards: pile,
                face_up: n,
            };
        }
        game
    }

    pub(crate) const EMPTY: [(&str, &str); TABLEAU_PILES] = [("", ""); TABLEAU_PILES];
}

#[cfg(test)]
mod tests {
    use super::fixtures::{EMPTY, card, position};
    use super::*;

    fn fresh() -> Game {
        Game::deal(Deck::shuffled(1))
    }

    fn transfer(from: Location, to: Location, count: usize) -> Move {
        Move::Transfer { from, to, count }
    }

    const T0: Location = Location::Tableau(0);
    const T1: Location = Location::Tableau(1);
    const W: Location = Location::Waste;

    // ── deal ─────────────────────────────────────────────────────────────

    #[test]
    fn deal_lays_out_the_table() {
        let game = fresh();
        for (i, pile) in game.tableau().iter().enumerate() {
            assert_eq!(pile.len(), i + 1);
            assert_eq!(pile.face_up().len(), 1);
            assert_eq!(pile.face_down_count(), i);
        }
        assert_eq!(game.stock().len(), 24);
        assert!(game.waste().is_empty());
        assert!(Suit::ALL.iter().all(|s| game.foundation(*s).is_empty()));
        assert_eq!(game.check_invariants(), Ok(()));
    }

    #[test]
    fn deal_is_deterministic_per_seed() {
        assert_eq!(Game::deal(Deck::shuffled(9)), Game::deal(Deck::shuffled(9)));
        assert_ne!(
            Game::deal(Deck::shuffled(9)),
            Game::deal(Deck::shuffled(10))
        );
    }

    // ── foundations ──────────────────────────────────────────────────────

    #[test]
    fn foundation_rules() {
        let mut t = EMPTY;
        t[0] = ("", "A♠");
        t[1] = ("", "2♠");
        t[2] = ("", "2♥");
        t[3] = ("", "3♠");
        let mut game = position("", "", ["", "", "", ""], t);
        let fs = Location::Foundation(Suit::Spades);
        let fh = Location::Foundation(Suit::Hearts);

        assert_eq!(game.is_legal(transfer(T1, fs, 1)), Err(Illegal::NeedsAce));
        assert_eq!(game.is_legal(transfer(T0, fh, 1)), Err(Illegal::WrongSuit));
        assert_eq!(game.is_legal(transfer(T0, fs, 1)), Ok(()));
        game.apply(transfer(T0, fs, 1)).unwrap();

        assert_eq!(game.is_legal(transfer(T3, fs, 1)), Err(Illegal::WrongRank));
        assert_eq!(game.is_legal(transfer(T2, fs, 1)), Err(Illegal::WrongSuit));
        assert_eq!(game.is_legal(transfer(T1, fs, 1)), Ok(()));
        game.apply(transfer(T1, fs, 1)).unwrap();
        assert_eq!(game.foundation(Suit::Spades), &[card("A♠"), card("2♠")]);
    }

    const T2: Location = Location::Tableau(2);
    const T3: Location = Location::Tableau(3);

    #[test]
    fn only_one_card_goes_to_a_foundation() {
        let mut t = EMPTY;
        t[0] = ("", "2♥ A♠");
        let game = position("", "", ["", "", "", ""], t);
        assert_eq!(
            game.is_legal(transfer(T0, Location::Foundation(Suit::Spades), 2)),
            Err(Illegal::TooManyCards)
        );
    }

    // ── tableau ──────────────────────────────────────────────────────────

    #[test]
    fn tableau_rules() {
        let mut t = EMPTY;
        t[0] = ("", "8♠");
        t[1] = ("", "7♥");
        t[2] = ("", "7♠");
        t[3] = ("", "6♥");
        t[4] = ("", "K♦");
        t[5] = ("", "Q♣");
        // t[6] empty
        let game = position("", "", ["", "", "", ""], t);
        let t6 = Location::Tableau(6);
        assert_eq!(game.is_legal(transfer(T1, T0, 1)), Ok(()));
        assert_eq!(game.is_legal(transfer(T2, T0, 1)), Err(Illegal::WrongColor));
        assert_eq!(game.is_legal(transfer(T3, T0, 1)), Err(Illegal::WrongRank));
        assert_eq!(game.is_legal(transfer(T5, t6, 1)), Err(Illegal::NeedsKing));
        assert_eq!(game.is_legal(transfer(T4, t6, 1)), Ok(()));
        assert_eq!(
            game.is_legal(transfer(T4, T4, 1)),
            Err(Illegal::SameLocation)
        );
        assert_eq!(
            game.is_legal(transfer(t6, T0, 1)),
            Err(Illegal::EmptySource)
        );
        assert_eq!(
            game.is_legal(transfer(T0, Location::Tableau(7), 1)),
            Err(Illegal::NoSuchPile)
        );
    }

    const T4: Location = Location::Tableau(4);
    const T5: Location = Location::Tableau(5);

    #[test]
    fn a_run_moves_together_and_keeps_its_order() {
        let mut t = EMPTY;
        t[0] = ("", "9♣ 8♥ 7♠");
        t[1] = ("", "10♦");
        let mut game = position("", "", ["", "", "", ""], t);
        assert_eq!(
            game.is_legal(transfer(T0, T1, 4)),
            Err(Illegal::TooManyCards)
        );
        assert_eq!(game.is_legal(transfer(T0, T1, 2)), Err(Illegal::WrongColor));
        game.apply(transfer(T0, T1, 3)).unwrap();
        assert_eq!(
            game.tableau()[1].face_up(),
            &[card("10♦"), card("9♣"), card("8♥"), card("7♠")]
        );
        assert!(game.tableau()[0].is_empty());
    }

    #[test]
    fn a_partial_run_can_move_from_the_middle() {
        let mut t = EMPTY;
        t[0] = ("", "9♣ 8♥ 7♠");
        t[1] = ("", "9♠");
        let mut game = position("", "", ["", "", "", ""], t);
        game.apply(transfer(T0, T1, 2)).unwrap();
        assert_eq!(game.tableau()[0].face_up(), &[card("9♣")]);
        assert_eq!(game.tableau()[1].len(), 3);
    }

    #[test]
    fn moving_the_last_face_up_card_reveals_the_next() {
        let mut t = EMPTY;
        t[0] = ("K♠", "7♥");
        t[1] = ("", "8♣");
        t[2] = ("", "8♠");
        let mut game = position("", "", ["", "", "", ""], t);
        let outcome = game.apply(transfer(T0, T1, 1)).unwrap();
        assert!(outcome.revealed);
        assert_eq!(game.tableau()[0].face_up(), &[card("K♠")]);
        assert_eq!(game.tableau()[0].face_down_count(), 0);

        // Moving a card with face-up cards still beneath it reveals nothing.
        let outcome = game.apply(transfer(T1, T2, 1)).unwrap();
        assert!(!outcome.revealed);
        assert_eq!(game.tableau()[1].face_up(), &[card("8♣")]);
    }

    #[test]
    fn waste_and_foundation_give_one_card() {
        let mut t = EMPTY;
        t[0] = ("", "8♣");
        t[1] = ("", "3♠");
        let mut game = position("", "9♠ 7♦", ["", "2♦", "", ""], t);
        assert_eq!(
            game.is_legal(transfer(W, T0, 2)),
            Err(Illegal::TooManyCards)
        );
        assert_eq!(game.is_legal(transfer(W, T0, 1)), Ok(()));
        let fd = Location::Foundation(Suit::Diamonds);
        assert_eq!(game.is_legal(transfer(fd, T1, 1)), Ok(()));
        game.apply(transfer(fd, T1, 1)).unwrap();
        assert!(game.foundation(Suit::Diamonds).is_empty());
        assert_eq!(game.is_legal(transfer(T1, fd, 1)), Err(Illegal::NeedsAce));
    }

    #[test]
    fn stock_is_never_a_source_and_nothing_lands_on_stock_or_waste() {
        let mut t = EMPTY;
        t[0] = ("", "8♣");
        let game = position("A♠", "", ["", "", "", ""], t);
        assert_eq!(
            game.is_legal(transfer(Location::Stock, T0, 1)),
            Err(Illegal::NotASource)
        );
        assert_eq!(
            game.is_legal(transfer(T0, Location::Stock, 1)),
            Err(Illegal::NotADestination)
        );
        assert_eq!(
            game.is_legal(transfer(T0, W, 1)),
            Err(Illegal::NotADestination)
        );
    }

    #[test]
    fn an_illegal_move_changes_nothing() {
        let mut game = fresh();
        let before = game.clone();
        let err = game.apply(transfer(T0, T1, 1)).err();
        if err.is_some() {
            assert_eq!(game, before);
        }
        assert_eq!(
            game.apply(transfer(Location::Stock, T0, 1)),
            Err(Illegal::NotASource)
        );
        assert_eq!(game, before);
    }

    // ── draw ─────────────────────────────────────────────────────────────

    #[test]
    fn draw_one_cycles_the_stock_in_order() {
        let mut game = fresh();
        let stock: Vec<Card> = game.stock().to_vec();
        for i in 0..24 {
            game.apply(Move::Draw).unwrap();
            assert_eq!(game.waste().last(), Some(&stock[23 - i]));
        }
        assert!(game.stock().is_empty());
        assert_eq!(game.waste().len(), 24);

        let outcome = game.apply(Move::Draw).unwrap();
        assert!(outcome.recycled);
        assert!(game.waste().is_empty());
        assert_eq!(game.stock(), stock.as_slice());

        game.apply(Move::Draw).unwrap();
        assert_eq!(game.waste().last(), Some(&stock[23]));
        assert_eq!(game.check_invariants(), Ok(()));
    }

    #[test]
    fn draw_three_turns_three_and_recycles_the_same_way() {
        let mut game = Game::new(Deck::shuffled(3), DrawMode::Three);
        let stock: Vec<Card> = game.stock().to_vec();
        game.apply(Move::Draw).unwrap();
        assert_eq!(game.waste(), &[stock[23], stock[22], stock[21]]);
        for _ in 0..7 {
            game.apply(Move::Draw).unwrap();
        }
        assert!(game.stock().is_empty());
        assert_eq!(game.waste().len(), 24);
        game.apply(Move::Draw).unwrap();
        assert_eq!(game.stock(), stock.as_slice());
    }

    #[test]
    fn draw_three_with_fewer_left_takes_what_remains() {
        let mut t = EMPTY;
        t[0] = ("", "K♠");
        let mut game = position("A♠ 2♠", "", ["", "", "", ""], t);
        game.draw_mode = DrawMode::Three;
        game.apply(Move::Draw).unwrap();
        assert_eq!(game.waste().len(), 2);
        assert!(game.stock().is_empty());
    }

    #[test]
    fn draw_with_nothing_is_illegal() {
        let mut t = EMPTY;
        t[0] = ("", "K♠");
        let game = position("", "", ["", "", "", ""], t);
        assert_eq!(game.is_legal(Move::Draw), Err(Illegal::NothingToDraw));
    }

    // ── end states ───────────────────────────────────────────────────────

    fn full_suit(suit: Suit) -> String {
        Rank::ALL
            .iter()
            .map(|r| format!("{}{}", r.label(), suit.symbol()))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn a_full_set_of_foundations_is_a_win() {
        let f = Suit::ALL.map(full_suit);
        let won = position("", "", [&f[0], &f[1], &f[2], &f[3]], EMPTY);
        assert!(won.is_won());
        assert!(!won.is_stuck());
        assert_eq!(won.check_invariants(), Ok(()));
        assert!(!fresh().is_won());
    }

    #[test]
    fn stuck_means_no_transfer_now_or_after_any_draw() {
        let mut t = EMPTY;
        t[0] = ("", "8♠");
        t[1] = ("", "8♣");
        let dead = position("3♥ 5♦", "", ["", "", "", ""], t);
        assert!(dead.is_stuck());

        // The same table with one playable card in the stock is not stuck.
        let alive = position("7♥ 5♦", "", ["", "", "", ""], t);
        assert!(!alive.is_stuck());

        // Neither call changed anything.
        assert_eq!(dead.waste().len(), 0);
        assert_eq!(alive.waste().len(), 0);
    }

    // ── legal_moves ──────────────────────────────────────────────────────

    #[test]
    fn legal_moves_agree_with_apply() {
        let mut game = fresh();
        for _ in 0..40 {
            let moves = game.legal_moves();
            for m in &moves {
                assert_eq!(
                    game.clone().apply(*m).err(),
                    None,
                    "{m:?} listed but refused"
                );
            }
            // A move not listed is refused.
            let unlisted = transfer(T0, T1, 1);
            if !moves.contains(&unlisted) {
                assert!(game.clone().apply(unlisted).is_err());
            }
            // Prefer a transfer; fall back to drawing.
            let next = moves.iter().find(|m| **m != Move::Draw).or(moves.first());
            let Some(next) = next else { break };
            game.apply(*next).unwrap();
            assert_eq!(game.check_invariants(), Ok(()));
        }
    }

    #[test]
    fn illegal_reasons_read_well() {
        assert_eq!(
            Illegal::NeedsKing.to_string(),
            "only a king can start an empty pile"
        );
    }
}
