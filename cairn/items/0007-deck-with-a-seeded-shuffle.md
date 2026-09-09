---
id: 7
title: Deck with a seeded shuffle
type: feature
status: planned
milestone: v0.1
depends_on:
- 6
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: s
area: engine
---

## Problem

A deal must be reproducible. Bug reports, tests, "play the same deal again",
and the eventual solver all need a shuffle that is a pure function of a seed.

## Proposal

`Deck::shuffled(seed: u64) -> Deck` using `rand` with `StdRng::seed_from_u64`
and the Fisher–Yates shuffle from `rand::seq::SliceRandom`. `Deck::new()` is
the unshuffled order. `Deck::draw()` pops from the top.

This adds the `rand` dependency; it is the one crate this project cannot
reasonably do without, and it stays behind the engine so the UI never touches
it. Pin the major version and note in this item if `StdRng`'s algorithm changes
across a `rand` major bump, because that silently changes which deal a seed
produces. Consider `rand_chacha` directly if that ever happens.

When no seed is given, the binary picks one from the OS and shows it, so every
deal is reproducible after the fact.

## Acceptance criteria

- [ ] Same seed, same order, across two calls and across test runs
- [ ] A shuffled deck still contains each of the 52 cards exactly once
- [ ] The dependency is added with a reason in the PR body
