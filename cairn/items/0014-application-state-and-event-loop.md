---
id: 14
title: Application state and event loop
type: feature
status: planned
milestone: v0.1
depends_on:
- 9
- 13
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: m
area: app
---

## Problem

The engine knows rules; the terminal knows keys. Something in between owns the
cursor, the current selection, the message line, and the decision of what an
input means in the current state. If that lives inside the draw code, it is
untestable; if it lives inside the engine, the engine learns about keyboards.

## Proposal

`app::App` is a plain state machine: `App::new(Game)`, `App::handle(Input) ->
Vec<Effect>`, `App::view() -> &ViewState`. `Input` is a small enum
(`Up`, `Down`, `Left`, `Right`, `Select`, `Cancel`, `Draw`, `NewGame`, `Quit`,
`Resize`) that the terminal layer translates keys into. Effects are things
only the outer loop can do (`Quit`).

Selection model: a cursor over locations; `Select` on nothing picks the
cursor's top run; `Select` again on a destination attempts the move and shows
the `Illegal` reason on the message line if it fails; `Cancel` clears.

The loop in `main`: poll an event, translate to `Input`, `App::handle`, draw.
No game logic in the loop, no key codes in `App`. `App` tests drive it with
`Input`s and assert on `ViewState`, which is the lever that makes the UI
testable without a terminal.

## Acceptance criteria

- [ ] `App` has no dependency on `crossterm` types
- [ ] Tests: select-then-select applies a legal move; an illegal one sets the message and keeps the selection
- [ ] `Quit` produces the effect and nothing else
