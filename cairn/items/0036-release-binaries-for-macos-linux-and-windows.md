---
id: 36
title: Release binaries for macOS, Linux and Windows
type: chore
status: backlog
milestone: v0.3
labels:
- release
created: 2026-09-08
updated: 2026-09-08
priority: p0
effort: m
area: tooling
---

`release.yml` currently creates a GitHub release with notes and nothing
attached. Add a build matrix (`x86_64` and `aarch64` for macOS and Linux,
`x86_64` for Windows), strip, package as `tar.gz`/`zip`, generate a
`SHA256SUMS`, and upload as release assets. Linux builds target `musl` so they
run anywhere.

Keep the workflow calling `scripts/task check` first; a release that has not
passed the seam does not ship. Actions stay SHA-pinned.

## Acceptance criteria

- [ ] A `v0.3.0-rc.1` tag produces five assets and a checksum file
- [ ] The macOS binary runs from a fresh download
- [ ] README install section lists the download option
