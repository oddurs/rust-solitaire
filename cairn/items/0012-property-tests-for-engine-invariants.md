---
id: 12
title: Property tests for engine invariants
type: test
status: planned
milestone: v0.1
labels:
- testing
depends_on:
- 9
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: m
area: engine
---

The example-based tests in the move item pin each rule. What they do not catch
is the interaction: a sequence of legal moves that leaves the table in a state
the rules never anticipated.

`proptest` (dev-dependency only) generates a seed and a sequence of indices
into `legal_moves()`, applies them, and asserts after every step:

- 52 cards, no duplicates, across all piles
- every pile's face-down cards are below its face-up cards
- foundations are strictly ascending single-suit from the ace
- every face-up tableau run alternates colour and descends by one

Also: any `Move` in `legal_moves()` is accepted by `apply`, and any `Move` not
in it is rejected. That equivalence is what makes `legal_moves()` trustworthy
as the basis of hints and the solver.

Keep the case count modest (256) so `scripts/task check` stays under a few
seconds; property tests that slow the hook get skipped, and skipped tests are
worse than none.

## Acceptance criteria

- [ ] Runs as part of `scripts/task test`
- [ ] A deliberately broken rule (mutate one branch locally) is caught
