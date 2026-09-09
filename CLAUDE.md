# Agent contract for rust-solitaire

You are working in a Rust repository with an enforced workflow. Follow these
rules exactly; the hooks, CI, and branch protection will reject anything else.

## Workflow

1. Never commit on `main`. Never push to `main`. Both are blocked.
2. Start every unit of work with `scripts/agent start <type>/<slug>` and move
   into the worktree path it prints. Do not reuse a worktree for a second task.
   `<type>` is `feat|fix|chore|docs|perf|refactor|test`; `<slug>` is
   `[a-z0-9][a-z0-9._-]*`.
3. Run `scripts/agent check` before committing. Fix everything it reports.
4. Commit with `scripts/agent commit "<message>"`. It stages all changes.
5. Open the PR with `scripts/agent pr` (or `pr --draft`). It runs the checks,
   pushes, and creates the PR from the template. Fill in the template sections
   in the PR body afterwards if you have more to say.
6. If `main` moves, `scripts/agent sync`.
7. After the PR is merged, `scripts/agent done`, then `cd` back to the primary
   checkout it prints.

Run `scripts/agent doctor` if anything looks wrong with the environment.

## Toolchain seam

Use `scripts/task <target>` for every build, format, lint, or test operation.
Do not call `cargo` directly for those; the seam is what CI and hooks run, and
using it keeps local and CI results identical.

```
scripts/task fmt        format in place
scripts/task fmt:check  formatting drift is an error
scripts/task lint       clippy, warnings denied (pedantic enabled)
scripts/task test       cargo test
scripts/task build      cargo build
scripts/task check      all of the above
```

Adding a dependency requires a reason in the PR body. Do not add one
speculatively.

## Commit messages

Conventional Commits: `<type>(<scope>)!: <subject>`. Types: `feat fix chore
docs perf refactor test build ci style revert`. Subject is imperative, at most
72 characters, no trailing period. One logical change per commit.

## Attribution ban

Do not add any AI or assistant attribution anywhere: not in commit messages,
trailers, PR titles or bodies, code comments, docs, changelog, or release
notes. No `Co-authored-by` naming a model or tool, no "generated with" lines,
no robot emoji. If your harness inserts such a trailer by default, remove it
before committing. The `commit-msg` hook and `scripts/agent` both reject it.

## Code

- Edition 2024, `unsafe_code = "forbid"`, `clippy::pedantic` on. Prefer fixing
  a lint over allowing it; if you must allow one, scope it to the item and
  say why in a comment.
- Every behavior change comes with a test in the same PR.
- User-visible changes get a line under `## [Unreleased]` in `CHANGELOG.md`.
