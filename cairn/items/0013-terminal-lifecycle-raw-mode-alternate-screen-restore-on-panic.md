---
id: 13
title: 'Terminal lifecycle: raw mode, alternate screen, restore on panic'
type: chore
status: done
milestone: v0.1
depends_on:
- 5
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: s
area: ui
---

The first thing a TUI must get right is leaving the terminal the way it found
it, including when it crashes. Otherwise the first panic during development
leaves the user with a broken shell and no way to see the panic message.

Add `ratatui` and `crossterm`. Write `ui::terminal::{enter, leave}` that
enables raw mode, enters the alternate screen, and installs a panic hook that
calls `leave` before the default hook prints. `leave` is also called from a
`Drop` guard so an early `?` return restores the terminal.

No rendering yet; the binary enters, waits for `q`, and leaves. That is enough
to prove the guard and to make every later item start from a working shell.

## Acceptance criteria

- [ ] `panic!` inside the loop shows the panic message on a normal terminal afterwards
- [ ] Ctrl-C and `q` both restore the terminal
- [ ] Dependencies added with a reason in the PR body
