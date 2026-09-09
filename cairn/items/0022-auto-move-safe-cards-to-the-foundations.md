---
id: 22
title: Auto-move safe cards to the foundations
type: feature
status: backlog
milestone: v0.2
depends_on:
- 9
- 16
created: 2026-09-08
updated: 2026-09-08
priority: p1
effort: m
area: engine
---

## Problem

The end of a Klondike game is dozens of obvious foundation moves. Players
expect one key to do them.

## Proposal

`Game::safe_foundation_moves()` returns foundation moves that can never hurt:
a card is safe to send up when both cards of the opposite colour one rank
lower are already on foundations (the standard "safe autoplay" rule), aces and
twos always. `a` in the UI applies them repeatedly until none remain, one
snapshot per card so undo steps back through them.

A `--autoplay` option to do this after every move is worth having and is a
one-line loop once the primitive exists; add it here if it is not disruptive.

## Acceptance criteria

- [ ] The safe rule is tested with a case where the naive rule would strand a card
- [ ] Repeated auto-move terminates
- [ ] Each auto-moved card is one undo step
