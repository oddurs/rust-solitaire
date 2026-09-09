//! Klondike solitaire: the rules, with no terminal attached.
//!
//! The binary in `src/main.rs` draws the table; everything that decides what a
//! legal move is lives here and is tested without a TTY.

pub mod app;
pub mod card;
pub mod deck;
pub mod klondike;
