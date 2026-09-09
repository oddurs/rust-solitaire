---
id: 10
title: Stock draw and waste recycling
type: feature
status: done
milestone: v0.1
depends_on:
- 9
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: s
area: engine
---

## Problem

`Draw` needs a definition: move one card from stock to waste, and when the
stock is empty, turn the waste back over to become the stock. Whether that
recycle is unlimited is a rule choice.

## Proposal

v0.1 is draw-one with unlimited recycles, the most common casual rule.
`Game` carries a `DrawRule { count: 1 | 3 }` from the start so draw-three
(v0.2) is a value change and not a refactor, but only `count: 1` is reachable
from the binary until then.

Recycling reverses the waste so that the draw order repeats exactly, which is
what players expect and what makes a deal's outcome a function of the seed and
the move list alone.

`Draw` on an empty stock and empty waste is `Illegal::EmptySource`, not a
no-op, so the UI can say so.

## Acceptance criteria

- [ ] 24 draws empties the stock into the waste in order
- [ ] The 25th draw recycles; the 26th produces the same card as the 1st
- [ ] Draw with both empty is illegal

## 2026-09-08

Shipped as `DrawMode { One, Three }` rather than a count field: two values, both named, nothing to validate. `Game::new(deck, mode)`; `Game::deal(deck)` is the draw-one shortcut.
