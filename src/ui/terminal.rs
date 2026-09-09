//! Entering and leaving the terminal's game mode, and guaranteeing the way
//! back out.
//!
//! The one thing a TUI must never do is leave the shell in raw mode. Three
//! exits are covered: a normal return (the [`Screen`] guard drops), an early
//! `?` return (same guard), and a panic (the hook `ratatui::try_init`
//! installs restores the terminal before the panic message prints, so the
//! message is actually readable).

use std::io;
use std::ops::{Deref, DerefMut};

use ratatui::DefaultTerminal;

/// The terminal in game mode: raw input, alternate screen. Dropping it puts
/// the shell back the way it was.
pub struct Screen {
    terminal: DefaultTerminal,
}

impl Screen {
    /// Switch the terminal into game mode.
    ///
    /// # Errors
    ///
    /// When raw mode or the alternate screen cannot be enabled, typically
    /// because stdout is not a terminal.
    pub fn enter() -> io::Result<Screen> {
        Ok(Screen {
            terminal: ratatui::try_init()?,
        })
    }
}

impl Deref for Screen {
    type Target = DefaultTerminal;

    fn deref(&self) -> &DefaultTerminal {
        &self.terminal
    }
}

impl DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut DefaultTerminal {
        &mut self.terminal
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
