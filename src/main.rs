//! A terminal solitaire game.

fn main() {
    println!("{}", greeting());
}

/// The banner shown when the game starts.
fn greeting() -> &'static str {
    "rust-solitaire: a terminal solitaire game"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_names_the_game() {
        assert!(greeting().starts_with("rust-solitaire"));
    }
}
