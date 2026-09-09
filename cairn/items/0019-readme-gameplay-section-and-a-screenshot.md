---
id: 19
title: README gameplay section and a screenshot
type: docs
status: planned
milestone: v0.1
depends_on:
- 17
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: s
area: docs
---

The README says the binary prints a banner, because that was true. When v0.1
lands it must describe the game instead: what Klondike is in two sentences, the
key table, `--seed`, and a screenshot of the board at 80x24.

A PNG in `doc/` is fine for now; an animated recording is a v0.3 item. Keep the
Status section honest about what is missing (undo, scoring), pointing at the
roadmap.

## Acceptance criteria

- [ ] Key table matches the code (copy it from the binding function's doc comment)
- [ ] Screenshot shows a real deal from a named seed
- [ ] Status section rewritten; no reference to the banner
