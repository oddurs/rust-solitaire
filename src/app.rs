//! The state machine between the keyboard and the rules.
//!
//! [`App`] owns a [`Game`] plus everything the player sees that is not a
//! card: where the cursor is, what has been picked up, the message line, the
//! move count. It is driven entirely by [`Input`], which the terminal layer
//! produces from keys, and it answers with [`Effect`]s only the outer loop
//! can carry out. No key codes here, and no drawing: tests drive it with
//! inputs and read its state back.

use crate::card::Suit;
use crate::deck::{Deck, random_seed};
use crate::klondike::{DrawMode, Game, Location, Move, TABLEAU_PILES};

/// What the player asked for, independent of which key said it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    /// Pick up the cards under the cursor, or drop what is held.
    Select,
    /// Put held cards back and clear the message.
    Cancel,
    /// Turn cards from the stock.
    Draw,
    /// Send the card under the cursor to its foundation.
    ToFoundation,
    /// Deal a fresh random game.
    NewGame,
    Quit,
}

/// Something only the outer loop can do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    Quit,
}

/// Where the cursor is. On a tableau pile, `depth` is how many cards from
/// the top would be picked up: 1 is the top card alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub location: Location,
    pub depth: usize,
}

/// Cards picked up and not yet dropped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    pub from: Location,
    pub count: usize,
}

/// Reading order of the top row: stock, waste, then the four foundations.
const TOP_ROW: [Location; 6] = [
    Location::Stock,
    Location::Waste,
    Location::Foundation(Suit::Clubs),
    Location::Foundation(Suit::Diamonds),
    Location::Foundation(Suit::Hearts),
    Location::Foundation(Suit::Spades),
];

/// Screen column of each top-row location; the tableau pile below it is the
/// one with the same index. Column 2 is the gap between waste and foundations.
const fn top_row_column(location: Location) -> usize {
    match location {
        Location::Stock => 0,
        Location::Waste => 1,
        Location::Foundation(suit) => 3 + suit.index(),
        Location::Tableau(i) => i,
    }
}

/// The top-row location nearest to a tableau column.
const fn top_row_at_column(column: usize) -> Location {
    match column {
        0 => Location::Stock,
        1 | 2 => Location::Waste,
        3 => Location::Foundation(Suit::Clubs),
        4 => Location::Foundation(Suit::Diamonds),
        5 => Location::Foundation(Suit::Hearts),
        _ => Location::Foundation(Suit::Spades),
    }
}

/// A game in play, with the player's cursor, selection and messages.
#[derive(Clone, Debug)]
pub struct App {
    game: Game,
    seed: u64,
    cursor: Cursor,
    selection: Option<Selection>,
    message: Option<String>,
    moves: u32,
}

impl App {
    /// Start a game from `seed` under the given draw rule.
    #[must_use]
    pub fn new(seed: u64, draw_mode: DrawMode) -> App {
        App::with_game(Game::new(Deck::shuffled(seed), draw_mode), seed)
    }

    /// Wrap an existing game; `seed` is only shown to the player.
    #[must_use]
    pub fn with_game(game: Game, seed: u64) -> App {
        App {
            game,
            seed,
            cursor: Cursor {
                location: Location::Stock,
                depth: 1,
            },
            selection: None,
            message: None,
            moves: 0,
        }
    }

    /// The table.
    #[must_use]
    pub fn game(&self) -> &Game {
        &self.game
    }

    /// The seed this deal came from.
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Where the cursor is.
    #[must_use]
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    /// What is picked up, if anything.
    #[must_use]
    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }

    /// The message line, usually why the last move was refused.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Moves applied so far. Refused moves do not count.
    #[must_use]
    pub fn moves(&self) -> u32 {
        self.moves
    }

    /// Respond to one input.
    pub fn handle(&mut self, input: Input) -> Vec<Effect> {
        match input {
            Input::Up => self.up(),
            Input::Down => self.down(),
            Input::Left => self.sideways(-1),
            Input::Right => self.sideways(1),
            Input::Select => self.select(),
            Input::Cancel => {
                self.selection = None;
                self.message = None;
            }
            Input::Draw => self.play(Move::Draw),
            Input::ToFoundation => self.send_to_foundation(),
            Input::NewGame => self.new_game(random_seed()),
            Input::Quit => return vec![Effect::Quit],
        }
        Vec::new()
    }

    /// Deal a fresh game from `seed`, keeping the draw rule.
    pub fn new_game(&mut self, seed: u64) {
        *self = App::new(seed, self.game.draw_mode());
    }

    // ── cursor ───────────────────────────────────────────────────────────

    fn up(&mut self) {
        if let Location::Tableau(i) = self.cursor.location {
            let run = self.game.tableau()[i].face_up().len();
            if self.cursor.depth < run {
                self.cursor.depth += 1;
            } else {
                self.cursor = Cursor {
                    location: top_row_at_column(i),
                    depth: 1,
                };
            }
        }
    }

    fn down(&mut self) {
        match self.cursor.location {
            Location::Tableau(_) if self.cursor.depth > 1 => self.cursor.depth -= 1,
            Location::Tableau(_) => {}
            top => {
                self.cursor = Cursor {
                    location: Location::Tableau(top_row_column(top).min(TABLEAU_PILES - 1)),
                    depth: 1,
                };
            }
        }
    }

    fn sideways(&mut self, step: isize) {
        let location = match self.cursor.location {
            Location::Tableau(i) => Location::Tableau(wrap(i, step, TABLEAU_PILES)),
            top => {
                let at = TOP_ROW.iter().position(|l| *l == top).unwrap_or(0);
                TOP_ROW[wrap(at, step, TOP_ROW.len())]
            }
        };
        self.cursor = Cursor { location, depth: 1 };
    }

    /// Keep the cursor depth within the run it points at; runs shrink after
    /// a move.
    fn clamp_depth(&mut self) {
        if let Location::Tableau(i) = self.cursor.location {
            let run = self.game.tableau()[i].face_up().len();
            self.cursor.depth = self.cursor.depth.clamp(1, run.max(1));
        }
    }

    // ── moves ────────────────────────────────────────────────────────────

    fn select(&mut self) {
        let here = self.cursor.location;
        match self.selection {
            None if here == Location::Stock => self.play(Move::Draw),
            None => {
                let count = self.cursor.depth;
                if self.has_cards(here) {
                    self.selection = Some(Selection { from: here, count });
                    self.message = None;
                } else {
                    self.message = Some("nothing there to pick up".to_owned());
                }
            }
            Some(held) if held.from == here => {
                self.selection = None;
                self.message = None;
            }
            Some(held) => self.play(Move::Transfer {
                from: held.from,
                to: here,
                count: held.count,
            }),
        }
    }

    fn send_to_foundation(&mut self) {
        let here = self.cursor.location;
        let top = match here {
            Location::Waste => self.game.waste().last().copied(),
            Location::Tableau(i) => self.game.tableau()[i].top(),
            _ => None,
        };
        match top {
            Some(card) => self.play(Move::Transfer {
                from: here,
                to: Location::Foundation(card.suit),
                count: 1,
            }),
            None => self.message = Some("nothing there to send up".to_owned()),
        }
    }

    /// Apply a move. Success clears the selection; refusal keeps it and
    /// explains itself on the message line.
    fn play(&mut self, m: Move) {
        match self.game.apply(m) {
            Ok(_) => {
                self.moves += 1;
                self.selection = None;
                self.message = None;
                self.clamp_depth();
            }
            Err(reason) => self.message = Some(reason.to_string()),
        }
    }

    fn has_cards(&self, location: Location) -> bool {
        match location {
            Location::Stock => !self.game.stock().is_empty(),
            Location::Waste => !self.game.waste().is_empty(),
            Location::Foundation(suit) => !self.game.foundation(suit).is_empty(),
            Location::Tableau(i) => !self.game.tableau()[i].is_empty(),
        }
    }
}

/// `at + step` modulo `len`, for cursors that wrap around a row.
fn wrap(at: usize, step: isize, len: usize) -> usize {
    let magnitude = step.unsigned_abs() % len;
    if step >= 0 {
        (at + magnitude) % len
    } else {
        (at + len - magnitude) % len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::klondike::fixtures::{EMPTY, card, position};

    const T0: Location = Location::Tableau(0);
    const T1: Location = Location::Tableau(1);

    fn app(tableau: [(&str, &str); TABLEAU_PILES]) -> App {
        App::with_game(position("A♠ 2♠", "", ["", "", "", ""], tableau), 0)
    }

    fn drive(app: &mut App, inputs: &[Input]) -> Vec<Effect> {
        inputs.iter().flat_map(|i| app.handle(*i)).collect()
    }

    fn go_to(app: &mut App, location: Location) {
        // Walk right along the appropriate row until the cursor arrives.
        if matches!(location, Location::Tableau(_)) {
            app.handle(Input::Down);
        }
        for _ in 0..8 {
            if app.cursor().location == location {
                return;
            }
            app.handle(Input::Right);
        }
        panic!("could not reach {location:?}");
    }

    #[test]
    fn select_then_select_applies_a_legal_move() {
        let mut t = EMPTY;
        t[0] = ("", "7♥");
        t[1] = ("", "8♣");
        let mut app = app(t);
        go_to(&mut app, T0);
        app.handle(Input::Select);
        assert_eq!(app.selection(), Some(Selection { from: T0, count: 1 }));
        go_to(&mut app, T1);
        app.handle(Input::Select);
        assert_eq!(app.selection(), None);
        assert_eq!(app.message(), None);
        assert_eq!(app.moves(), 1);
        assert_eq!(app.game().tableau()[1].face_up(), &[card("8♣"), card("7♥")]);
    }

    #[test]
    fn an_illegal_drop_explains_and_keeps_the_selection() {
        let mut t = EMPTY;
        t[0] = ("", "7♥");
        t[1] = ("", "8♥");
        let mut app = app(t);
        go_to(&mut app, T0);
        app.handle(Input::Select);
        go_to(&mut app, T1);
        app.handle(Input::Select);
        assert_eq!(app.selection(), Some(Selection { from: T0, count: 1 }));
        assert_eq!(app.message(), Some("colours must alternate"));
        assert_eq!(app.moves(), 0);
        app.handle(Input::Cancel);
        assert_eq!(app.selection(), None);
        assert_eq!(app.message(), None);
    }

    #[test]
    fn selecting_the_same_pile_again_puts_the_cards_back() {
        let mut t = EMPTY;
        t[0] = ("", "7♥");
        let mut app = app(t);
        go_to(&mut app, T0);
        drive(&mut app, &[Input::Select, Input::Select]);
        assert_eq!(app.selection(), None);
    }

    #[test]
    fn selecting_an_empty_pile_says_so() {
        let mut app = app(EMPTY);
        go_to(&mut app, T1);
        app.handle(Input::Select);
        assert_eq!(app.selection(), None);
        assert_eq!(app.message(), Some("nothing there to pick up"));
    }

    #[test]
    fn select_on_the_stock_draws() {
        let mut app = app(EMPTY);
        assert_eq!(app.cursor().location, Location::Stock);
        app.handle(Input::Select);
        assert_eq!(app.game().waste().len(), 1);
        app.handle(Input::Draw);
        assert!(app.game().stock().is_empty());
        app.handle(Input::Draw);
        assert_eq!(app.game().stock().len(), 2);
        assert!(app.game().waste().is_empty());
        assert_eq!(app.moves(), 3);
    }

    #[test]
    fn drawing_with_nothing_explains_itself() {
        let mut app = App::with_game(position("", "", ["", "", "", ""], EMPTY), 0);
        app.handle(Input::Draw);
        assert_eq!(app.message(), Some("nothing left to draw"));
        assert_eq!(app.moves(), 0);
    }

    #[test]
    fn to_foundation_sends_the_top_card_up() {
        let mut t = EMPTY;
        t[0] = ("", "2♥ A♦");
        let mut app = app(t);
        go_to(&mut app, T0);
        app.handle(Input::ToFoundation);
        assert_eq!(app.game().foundation(Suit::Diamonds), &[card("A♦")]);
        app.handle(Input::ToFoundation);
        assert_eq!(app.message(), Some("only an ace can start a foundation"));
    }

    #[test]
    fn quit_is_an_effect_and_nothing_else() {
        let mut app = app(EMPTY);
        let before = app.game().clone();
        assert_eq!(app.handle(Input::Quit), vec![Effect::Quit]);
        assert_eq!(app.game(), &before);
        assert_eq!(app.handle(Input::Left), Vec::new());
    }

    #[test]
    fn cursor_walks_the_rows_and_wraps() {
        let mut app = app(EMPTY);
        app.handle(Input::Right);
        assert_eq!(app.cursor().location, Location::Waste);
        app.handle(Input::Left);
        app.handle(Input::Left);
        assert_eq!(app.cursor().location, Location::Foundation(Suit::Spades));
        app.handle(Input::Down);
        assert_eq!(app.cursor().location, Location::Tableau(6));
        app.handle(Input::Right);
        assert_eq!(app.cursor().location, T0);
        app.handle(Input::Up);
        assert_eq!(app.cursor().location, Location::Stock);
    }

    #[test]
    fn up_deepens_the_pick_within_a_run_then_leaves_it() {
        let mut t = EMPTY;
        t[3] = ("K♠", "9♣ 8♥ 7♠");
        let mut app = app(t);
        go_to(&mut app, Location::Tableau(3));
        assert_eq!(app.cursor().depth, 1);
        app.handle(Input::Up);
        app.handle(Input::Up);
        assert_eq!(app.cursor().depth, 3);
        app.handle(Input::Down);
        assert_eq!(app.cursor().depth, 2);
        app.handle(Input::Up);
        app.handle(Input::Up);
        assert_eq!(app.cursor().location, Location::Foundation(Suit::Clubs));
        assert_eq!(app.cursor().depth, 1);
    }

    #[test]
    fn a_deep_pick_moves_the_whole_run() {
        let mut t = EMPTY;
        t[0] = ("", "9♣ 8♥ 7♠");
        t[1] = ("", "10♦");
        let mut app = app(t);
        go_to(&mut app, T0);
        drive(&mut app, &[Input::Up, Input::Up, Input::Select]);
        assert_eq!(app.selection(), Some(Selection { from: T0, count: 3 }));
        go_to(&mut app, T1);
        app.handle(Input::Select);
        assert_eq!(app.game().tableau()[1].len(), 4);
        assert!(app.game().tableau()[0].is_empty());
    }

    #[test]
    fn new_game_resets_everything_but_the_draw_rule() {
        let mut app = App::new(5, DrawMode::Three);
        drive(&mut app, &[Input::Draw, Input::Right, Input::Select]);
        assert_eq!(app.moves(), 1);
        app.new_game(6);
        assert_eq!(app.seed(), 6);
        assert_eq!(app.moves(), 0);
        assert_eq!(app.selection(), None);
        assert_eq!(app.game().draw_mode(), DrawMode::Three);
        assert_eq!(app.game(), &Game::new(Deck::shuffled(6), DrawMode::Three));
    }
}
