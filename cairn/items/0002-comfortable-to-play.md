---
id: 2
key: v0.2
title: Comfortable to play
type: milestone
status: backlog
depends_on:
- 1
created: 2026-09-08
updated: 2026-09-08
priority: p2
due: 2026-11-03
---

Undo, draw-three, hints, auto-move, score, help, mouse, and a board that survives a resize.

What people expect from a solitaire program once it works: undo, draw-three,
hints, auto-move, a timer and score, a help screen, mouse, and a terminal that
can be resized without breaking the board.

Each of these is cheap on its own and worthless before v0.1 exists, which is why
they wait. The ordering rule inside the milestone: anything that changes how the
engine is called (undo, draw-three) lands before anything that only adds
surface (themes, mouse), so the surface is built on the final shape.
