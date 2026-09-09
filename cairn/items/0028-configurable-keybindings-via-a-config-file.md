---
id: 28
title: Configurable keybindings via a config file
type: feature
status: backlog
milestone: v0.2
depends_on:
- 16
- 25
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: m
area: app
---

## Problem

vi users and arrow-key users disagree, and both are right. The binding table
is one function; making it data lets people change it without a fork.

## Proposal

`~/.config/rust-solitaire/config.toml` (via `directories` for the platform
path), parsed with `serde` and `toml`:

```toml
[keys]
draw = ["d", "s"]
select = ["space", "enter"]
```

Unset keys keep their defaults. An unknown action or key name is an error at
startup with the line number. The help overlay reflects the loaded bindings.
Themes join this file once it exists.

Three dependencies for one feature is the most this project has taken on at
once; the PR body should justify each.

## Acceptance criteria

- [ ] Missing file is silent; malformed file is a one-line error and exit
- [ ] A rebound key works and shows in the help overlay
- [ ] Tests parse a sample config from a string
