---
id: 40
title: Dependency audit in CI
type: chore
status: backlog
milestone: v0.3
depends_on:
- 33
created: 2026-09-08
updated: 2026-09-08
priority: p2
effort: s
area: tooling
---

By v0.3 the dependency tree has `rand`, `ratatui`, `crossterm`, `serde`,
`toml`, `directories` and their transitive closure. `cargo deny` checks
advisories, licences and duplicate versions. Add `deny.toml` and a `deny`
target in `scripts/task` that `check` includes, so it is one more thing the
seam runs rather than a separate workflow.

## Acceptance criteria

- [ ] `scripts/task check` runs it; CI needs no extra job
- [ ] Licence allowlist is MIT/Apache-2.0/BSD/ISC/Unicode; anything else is a deliberate exception noted in `deny.toml`
