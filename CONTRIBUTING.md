# Contributing

## Setup

```sh
git clone https://github.com/oddurs/rust-solitaire.git
cd rust-solitaire
scripts/setup
```

`scripts/setup` sets `core.hooksPath` to the tracked `.githooks/` and runs the
full check. Run `scripts/agent doctor` any time to verify your environment.

## Workflow

`main` only advances through merged pull requests. Branch protection enforces
this on the server; the `pre-push` hook enforces it locally.

1. Start a branch in its own worktree. Never share a checkout between two
   pieces of work.

   ```sh
   scripts/agent start <type>/<slug>
   cd ../.worktrees/rust-solitaire/<type>/<slug>
   ```

   `<type>` is one of `feat`, `fix`, `chore`, `docs`, `perf`, `refactor`,
   `test`. `<slug>` is lowercase `[a-z0-9._-]`.

2. Make the change. Run `scripts/agent check` as often as you like.

3. Commit with a Conventional Commit message:

   ```sh
   scripts/agent commit "feat(deck): shuffle with a seeded RNG"
   ```

4. Open the pull request. This runs the full check, pushes, and creates the PR
   from the template:

   ```sh
   scripts/agent pr           # or: scripts/agent pr --draft
   ```

5. If `main` moves under you, rebase: `scripts/agent sync`.

6. After the PR is squash-merged, clean up:

   ```sh
   scripts/agent done
   ```

## Commit messages

[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/):

```
<type>(<scope>)!: <subject>
```

- `type`: `feat`, `fix`, `chore`, `docs`, `perf`, `refactor`, `test`, `build`,
  `ci`, `style`, `revert`.
- `scope` is optional. `!` marks a breaking change.
- Subject is imperative, 72 characters or fewer, no trailing period.
- No AI or assistant attribution of any kind: no `Co-authored-by` trailers
  naming a model or tool, no "generated with" lines. The `commit-msg` hook
  rejects them.

PRs are squash-merged, so the PR title becomes the commit on `main` and must
follow the same format.

## Checks

Everything goes through `scripts/task`:

```sh
scripts/task fmt        # format in place
scripts/task fmt:check  # formatting drift fails
scripts/task lint       # clippy, warnings denied
scripts/task test       # cargo test
scripts/task build      # cargo build
scripts/task check      # all of the above; what CI and pre-push run
```

A PR must be green on the `required` CI check and up to date with `main`
before it can merge. The repository currently has a single maintainer, so
required approvals are set to 0; review is still expected for anything
non-trivial, and the setting will go to 1 when a second maintainer joins.

## Changelog

Add a line under `## [Unreleased]` in `CHANGELOG.md` for any user-visible
change.
