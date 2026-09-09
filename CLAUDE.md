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

<!-- cairn:begin -->
## Roadmap and issues

This project tracks its roadmap and issues with `cairn`. Every item is a Markdown file under `cairn/items`, described by the schema in `cairn.toml`.

**Do not create ad-hoc TODO, PLAN or NOTES files.** Create a cairn item instead, so the work appears on the board and in the generated roadmap.

### The loop

1. `cairn next` — what is ready to start. It excludes anything blocked by unfinished dependencies and puts work already in progress first.
2. `cairn claim <ID>` — take it before you start, so no one duplicates the work. `cairn claim --next` picks and claims the top-ranked unclaimed item in one step, and prints its body so you can begin immediately.
3. Do the work. Record what you learn: `cairn set <ID> <field>=<value>` for fields, `cairn note <ID> "<TEXT>"` for anything that needs a sentence — why you chose something, what you tried, what to watch for.
4. `cairn close <ID>` when it is done, or `cairn release <ID>` to hand it back.
5. `cairn check` before you report finished. It must pass.

### Commands

```sh
cairn next --json                 # ready work, ranked
cairn claim --next                # take the next ready item
cairn search <TEXT> --json        # titles, bodies and labels
cairn list --json                 # all open items
cairn list --filter 'blocked=false,priority=p0'
cairn show <ID> --json            # one item, including its body
cairn new "<TITLE>" --type <TYPE> --milestone <MILESTONE>
cairn set <ID> status=<STATUS>    # also labels+=x, or any field below
cairn note <ID> "<TEXT>"          # append reasoning; never replaces
cairn close <ID>
cairn check                       # validate; run before finishing
cairn render                      # regenerate ROADMAP.md
```

### Schema

- **Types**: `feature`, `bug`, `chore`, `docs`, `test`, `milestone`
- **Statuses**: `backlog` (open), `planned` (open), `doing` (active), `blocked` (active), `done` (done), `dropped` (dropped)
- **`milestone`**: names a `milestone` item, by key — the release this ships in
- **`due`**: date, YYYY-MM-DD — when a milestone is meant to land
- **`priority`**: one of p0, p1, p2, p3 — p0 blocks the milestone it is in
- **`effort`**: one of s, m, l, xl — Rough size: s is an hour, xl is a week
- **`area`**: one of engine, ui, app, tooling, docs — engine is pure rules with no terminal; ui is rendering and input; app is the loop, config and persistence
- **Milestones**: `v0.1` (due 2026-10-06), `v0.2` (due 2026-11-03), `v0.3` (due 2026-12-08), `later`
- **Saved views** (`cairn list --view NAME`): `now`, `next`, `triage`

### Rules

1. Before starting work, find or create the item and set it to an active status.
2. Use the fields above rather than inventing new ones; add new fields to `cairn.toml` first.
3. Never hand-edit the generated roadmap file — change items and run `cairn render`.
4. `cairn check` must pass before the work is considered done.

<!-- cairn:end -->
