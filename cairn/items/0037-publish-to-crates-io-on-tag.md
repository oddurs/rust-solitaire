---
id: 37
title: Publish to crates.io on tag
type: chore
status: backlog
milestone: v0.3
labels:
- release
depends_on:
- 36
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: s
area: tooling
---

`cargo install rust-solitaire` should work. Add a `publish` job to
`release.yml` gated on the `CARGO_REGISTRY_TOKEN` secret, after the binaries
job. `cargo publish --dry-run` runs in CI on every PR from now on so packaging
breakage is caught before a tag.

Check the crate name is free on crates.io before assuming it; if it is not,
this item records the chosen alternative.

## Acceptance criteria

- [ ] Dry run passes in CI
- [ ] A tag publishes, and `cargo install` of that version works
