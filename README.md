# gitarch

Git archaeology tool. Extracts implicit knowledge from repository history that no existing tool surfaces.

## What It Does

Git repos contain a massive amount of information beyond the code itself -- who understands what, which files are secretly coupled, when architectural decisions happened, which modules have been abandoned. `gitarch` extracts all of this from the commit history.

## Commands

### `gitarch knowledge <path>`

Knowledge map with bus factor analysis. For every file or directory, computes who actually understands it -- weighted by lines changed, recency, and whether the contributor is still active.

```
$ gitarch knowledge src/
src/auth/          alice (0.72)  bob (0.28)
src/api/handler.rs alice (0.45)  carol (0.31)  bob (0.24)
src/db/            bob (0.91)    BUS FACTOR: 1
src/utils.rs       NO ACTIVE OWNER (last touch: 8 months ago)
```

### `gitarch coupling [--min-score <f>]`

Change coupling analysis. Finds files that always change together even if they're in different directories -- revealing hidden architectural dependencies.

```
$ gitarch coupling --min-score 0.5
src/auth/login.rs    <->  src/middleware/session.rs    0.83 (24/29 commits)
src/api/users.rs     <->  src/db/user_queries.rs       0.71 (15/21 commits)
src/config.rs        <->  tests/integration/setup.rs   0.64 (9/14 commits)
```

### `gitarch decay [--months <n>]`

Ownership decay detection. Distinguishes "stable code nobody needs to touch" from "abandoned code nobody owns anymore."

```
$ gitarch decay --months 6
DECAY  src/payments/stripe.rs
  Primary owner: carol (87% of changes)
  Last contribution: 2025-08-14 (5 months ago)
  Carol's last commit to repo: 2025-08-20
  Status: LIKELY DEPARTED -- no replacement owner

OK     src/utils/crypto.rs
  Primary owner: alice (94% of changes)
  Last contribution: 2025-03-01 (11 months ago)
  Module change frequency: 0.1 commits/month (stable)
  Status: STABLE -- low change rate is normal
```

### `gitarch evolution`

Architecture evolution timeline. Detects structural events -- new modules, refactors, dependency additions, large deletions.

```
$ gitarch evolution
2024-01  ####  Project created (47 files)
2024-03  ##    Added src/api/ (REST API layer)
2024-05  #     Added sqlx dependency
2024-06  ###   Refactor: src/handlers/ -> src/api/handlers/
2024-09  #     Added src/workers/ (background jobs)
2024-11  ##    Deleted src/legacy/ (34 files removed)
2025-02  #     Added docker/ directory
```

### `gitarch health`

Project health dashboard. Aggregate metrics derived from commit patterns.

- Commit frequency trend (accelerating / decelerating / stable)
- Merge commit patterns (PR merge time estimates)
- Test-to-code ratio over time
- Average commit size (big-bang vs. small-and-frequent)
- Active contributor count trend
- Day-of-week / time-of-day activity patterns

## Architecture

```
src/
  main.rs              # clap CLI entry point
  cli/                 # subcommand definitions
  repo.rs              # git data access layer (git2)
  analysis/
    knowledge.rs       # knowledge map / bus factor
    coupling.rs        # change coupling
    decay.rs           # ownership decay detection
    evolution.rs       # architecture timeline
    health.rs          # project health metrics
  output/
    terminal.rs        # formatted terminal output
    json.rs            # machine-readable JSON output
    tui.rs             # ratatui interactive dashboard (later)
```

## Tech Stack

- **git2** -- libgit2 bindings for git data access (blame, diffs, revwalks)
- **clap** v4 (derive) -- CLI argument parsing
- **rayon** -- parallel analysis across commits
- **serde** + **serde_json** -- structured output
- **indicatif** -- progress bars for long analyses
- **ratatui** -- TUI dashboard for `health` command (later)
- **axum** -- local web dashboard (later)

## Build Order

1. **Scaffolding** -- CLI skeleton, git2 repo access, commit walker, basic stats
2. **Knowledge map** -- the most valuable single feature; exercises full pipeline
3. **Change coupling** -- reuses commit walker, different aggregation
4. **Ownership decay** -- combines knowledge map with time analysis
5. **Architecture evolution** -- structural event detection
6. **Project health** -- aggregate metrics, TUI dashboard

## Usage

```bash
# Analyze the current repo
gitarch knowledge .

# Analyze a specific repo with JSON output
gitarch knowledge /path/to/repo --json

# Find coupled files with at least 50% co-change rate
gitarch coupling --min-score 0.5

# Find abandoned modules (no owner activity in 6 months)
gitarch decay --months 6

# Show architecture evolution
gitarch evolution

# Project health dashboard
gitarch health
```

## References

- Gall, Hajek, Jazayeri (1998). "Detection of Logical Coupling Based on Product Release History."
- Tornhill, Adam. "Your Code as a Crime Scene." (change coupling, knowledge maps)
- Hercules (src-d/hercules) -- abandoned Go-based git analysis tool. gitarch covers similar ground in Rust with different features.
