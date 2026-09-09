//! Property tests over the public engine API: whatever sequence of legal moves
//! is played, the table stays one the rules could have produced, and
//! `legal_moves` agrees exactly with `apply`.

use proptest::prelude::*;
use rust_solitaire::card::Suit;
use rust_solitaire::deck::Deck;
use rust_solitaire::klondike::{DrawMode, Game, Location, Move, TABLEAU_PILES};

fn location() -> impl Strategy<Value = Location> {
    prop_oneof![
        Just(Location::Stock),
        Just(Location::Waste),
        any::<u8>().prop_map(|i| Location::Foundation(Suit::ALL[usize::from(i % 4)])),
        (0..TABLEAU_PILES + 1).prop_map(Location::Tableau),
    ]
}

fn arbitrary_move() -> impl Strategy<Value = Move> {
    prop_oneof![
        Just(Move::Draw),
        (location(), location(), 0..8usize).prop_map(|(from, to, count)| Move::Transfer {
            from,
            to,
            count
        }),
    ]
}

fn draw_mode() -> impl Strategy<Value = DrawMode> {
    prop_oneof![Just(DrawMode::One), Just(DrawMode::Three)]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Play up to 60 legal moves chosen by index; every intermediate table
    /// satisfies the invariants and every listed move is accepted.
    #[test]
    fn legal_play_preserves_invariants(
        seed in any::<u64>(),
        mode in draw_mode(),
        picks in proptest::collection::vec(any::<usize>(), 1..60),
    ) {
        let mut game = Game::new(Deck::shuffled(seed), mode);
        prop_assert_eq!(game.check_invariants(), Ok(()));
        for pick in picks {
            let moves = game.legal_moves();
            if moves.is_empty() {
                prop_assert!(game.is_won());
                break;
            }
            let m = moves[pick % moves.len()];
            let before = game.clone();
            prop_assert!(game.apply(m).is_ok(), "{:?} was listed but refused", m);
            prop_assert_ne!(&game, &before, "{:?} changed nothing", m);
            prop_assert_eq!(game.check_invariants(), Ok(()));
        }
    }

    /// `is_legal` and membership in `legal_moves` are the same predicate,
    /// including for moves that name nonsense locations or counts.
    #[test]
    fn legal_moves_is_exactly_what_apply_accepts(
        seed in any::<u64>(),
        mode in draw_mode(),
        warmup in 0..30usize,
        probes in proptest::collection::vec(arbitrary_move(), 1..40),
    ) {
        let mut game = Game::new(Deck::shuffled(seed), mode);
        for _ in 0..warmup {
            let moves = game.legal_moves();
            let Some(m) = moves.first() else { break };
            game.apply(*m).unwrap();
        }
        let listed = game.legal_moves();
        for m in probes.iter().copied().chain(listed.iter().copied()) {
            let before = game.clone();
            let accepted = game.clone().apply(m).is_ok();
            prop_assert_eq!(accepted, listed.contains(&m), "disagreement on {:?}", m);
            prop_assert_eq!(game.is_legal(m).is_ok(), accepted);
            prop_assert_eq!(&game, &before, "is_legal mutated the game");
        }
    }

    /// A refused move leaves the table untouched.
    #[test]
    fn refused_moves_change_nothing(
        seed in any::<u64>(),
        probes in proptest::collection::vec(arbitrary_move(), 1..40),
    ) {
        let mut game = Game::deal(Deck::shuffled(seed));
        for m in probes {
            let before = game.clone();
            if game.apply(m).is_err() {
                prop_assert_eq!(&game, &before);
            }
        }
    }
}
