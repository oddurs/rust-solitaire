# rust-solitaire

[![CI](https://github.com/oddurs/rust-solitaire/actions/workflows/ci.yml/badge.svg)](https://github.com/oddurs/rust-solitaire/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A terminal solitaire game written in Rust.

## Why

Solitaire is a small, well-understood game with real rules, real state, and a
real UI. That makes it a good vehicle for a clean, tested Rust codebase that is
pleasant to work in for both people and coding agents. The repository is set up
so that the workflow is enforced by tooling, not by convention.

## Status

The project is at its foundation stage. The binary currently prints a banner
and exits. Game logic and the terminal UI land through pull requests; see
[CHANGELOG.md](CHANGELOG.md) for what has shipped.

## Install

Requires a Rust toolchain via [rustup](https://rustup.rs). The pinned version
in `rust-toolchain.toml` is installed automatically on first build.

```sh
git clone https://github.com/oddurs/rust-solitaire.git
cd rust-solitaire
cargo install --path .
```

## Quickstart

```sh
rust-solitaire
```

Or run from the source tree without installing:

```sh
cargo run
```

## Development

Run this once after cloning. It wires the tracked git hooks and runs the full
check so you know the toolchain works:

```sh
scripts/setup
```

All work happens on a branch in its own worktree and lands on `main` through a
pull request. `scripts/agent` drives that loop:

```sh
scripts/agent start feat/deal-cards   # branch + worktree, prints the path
cd ../.worktrees/rust-solitaire/feat/deal-cards
# ...edit...
scripts/agent commit "feat: deal a shuffled deck into seven tableau columns"
scripts/agent pr                      # runs the checks, pushes, opens the PR
scripts/agent done                    # after merge: cleans up worktree + branch
```

`scripts/task` is the one interface to the toolchain. CI, the git hooks, and
`scripts/agent` all call it, so they can never disagree:

| Target | What it runs |
|---|---|
| `scripts/task fmt` | `cargo fmt` |
| `scripts/task fmt:check` | `cargo fmt --check` |
| `scripts/task lint` | `cargo clippy` with warnings denied |
| `scripts/task test` | `cargo test` |
| `scripts/task build` | `cargo build` |
| `scripts/task check` | all of the above |

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow.

## License

[MIT](LICENSE)
