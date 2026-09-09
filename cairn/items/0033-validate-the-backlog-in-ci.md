---
id: 33
title: Validate the backlog in CI
type: chore
status: backlog
milestone: v0.2
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: s
area: tooling
---

`cairn check` and `cairn render --check` belong in `scripts/task lint` so a PR
cannot merge with an invalid item or a stale `ROADMAP.md`. They are not there
yet because CI would need to install `cairn`, and the honest options are a
`curl | sh` of its install script or waiting for it to reach crates.io.

Decide here: once `cairn-md` is on crates.io, add `cargo install cairn-md
--locked` to the CI job (cached by `rust-cache`) and the two commands to the
seam. Until then the hooks in `cairn.toml` keep the roadmap current locally.

## Acceptance criteria

- [ ] `scripts/task lint` fails on an invalid item and on a stale roadmap
- [ ] CI installs a pinned version
