---
id: 38
title: Homebrew tap
type: chore
status: backlog
milestone: v0.3
labels:
- release
depends_on:
- 36
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: s
area: tooling
---

A formula in `oddurs/homebrew-tap` (or a tap dedicated to this project,
whichever exists first) that downloads the release tarball by checksum. Bump
it from the release workflow with a small script rather than by hand.

## Acceptance criteria

- [ ] `brew install oddurs/tap/rust-solitaire` works on both macOS architectures
- [ ] The formula is updated automatically on release
