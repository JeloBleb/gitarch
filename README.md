# gitarch

A modern, streamlined replacement for
[code-maat](https://github.com/adamtornhill/code-maat). Extracts implicit
knowledge from git repository commit history -- ownership, coupling, decay,
churn, and project health -- with structured JSON output designed for LLM
consumption.

## Why gitarch?

code-maat is deprecated and its ideas have moved behind a commercial paywall
(CodeScene). gitarch aims to be the open-source alternative: fewer moving parts,
direct git2 repo access (no log file export), and a decay scoring system that
distinguishes stable code from abandoned code. JSON output makes it easy to pipe
results into LLM-based analysis for deeper, targeted code review.

### Open source contribution use case

Run gitarch on an unfamiliar open-source project to quickly find where you can
be useful. Decay scores reveal abandoned files that need attention but have no
active owner -- safe places to contribute without stepping on anyone's toes.
Coupling data shows what else you need to understand before touching a file.
Ownership data tells you who to ask for review. Pipe the JSON output into an
LLM to get a prioritized list of contribution opportunities tailored to a
newcomer.

## Features

### Core analyses
- **Ownership** -- per-file ownership map, primary owners, ownership
  concentration (bus factor)
- **Coupling** -- files that frequently change together, with coupling
  percentage and configurable filters
- **Decay** -- composite score (0.0--1.0) combining file staleness, owner
  inactivity, and ownership concentration to distinguish stable code from
  abandoned code. Unique to gitarch.

### Metrics
- **Revision counts** -- commit frequency per file (hotspot detection)
- **Churn** -- lines added/deleted per file, per author, and over time
- **Last modified** -- file age tracking
- **Author activity** -- per-author last active timestamps
- **Authors per file** -- distinct contributor count per file
- **Summary** -- repo-wide stats (total commits, files, authors)

### Additional analyses
- **Communication** -- developer coupling inferred from shared file ownership

### Planned
- **Hotspots** -- files ranked by revision count (change frequency)
- **Authors per file** -- distinct contributor count per file (bus factor)
- **Detailed ownership** -- full author breakdown per file, not just primary
- **Author summary** -- per-author stats (files owned, last active, commit count)
- **Age** -- file creation and last modified dates
- **`--top N` / `--limit`** -- limit output to the top N results
- **`--path` filter** -- scope any analysis to a subdirectory or glob pattern

## Usage

```bash
gitarch summary                        # repo-wide stats
gitarch ownership                      # primary owner per file
gitarch coupling                       # co-change pairs
gitarch decay                          # composite decay scores
gitarch churn                          # lines added/deleted per file
gitarch communication                  # developer coupling via shared files
```

### Global flags
- `--repo <path>` -- analyze a different repository (defaults to `.`)
- `--json` -- machine-readable JSON output
- `--since <YYYY-MM-DD>` -- only include commits from this date onward
- `--until <YYYY-MM-DD>` -- only include commits up to this date

### Subcommand flags
- `coupling --max-changeset-size <N>` -- ignore commits touching more than N
  files (default: 20)
- `coupling --coupling-percentage <N>` -- minimum coupling percentage to
  display (default: 15)
- `decay --decay-threshold <DAYS>` -- number of days until a file is
  considered fully stale (default: 180)

## Architecture

```
src/
  main.rs           # clap CLI entry point
  cli.rs            # subcommand definitions (clap derive structs)
  repo.rs           # git2 data access layer
  output.rs         # table + JSON output formatting
  analysis/
    metrics.rs      # raw data extraction (ownership, coupling, counts, timestamps, churn)
    derived.rs      # derived analysis (decay scoring, file concentration)
```

Data flow: `git2 repo -> Vec<CommitInfo> -> metrics -> derived analysis -> output`

## Tech Stack

- **git2** -- libgit2 bindings for direct repository access
- **clap** (derive) -- CLI parsing
- **cliux** -- terminal table output
- **thiserror** -- typed errors in library code
- **anyhow** -- error handling in CLI layer
- **itertools** -- combinatorics for coupling analysis
- **serde** + **serde_json** -- JSON serialization for output
- **chrono** -- date parsing and formatting
- **rayon** -- parallel analysis (planned)

## Build Order

1. ~~Scaffolding -- CLI, git2 repo access, commit walker~~
2. ~~Core metrics -- ownership, revision counts, churn, timestamps~~
3. ~~Coupling -- raw co-change counts~~
4. ~~Decay -- composite decay scoring~~
5. ~~Module reorganization (metrics.rs + derived.rs)~~
6. ~~CLI wiring + terminal table output~~
7. ~~Output improvements (sorting, filtering deleted files, noise reduction)~~
8. ~~JSON output~~
9. ~~Communication -- developer coupling~~
10. ~~Global `--since` / `--until` date filters~~
11. ~~Decay `--decay-threshold` flag~~
12. ~~`--version` flag~~
13. ~~Error handling -- `run()` pattern, `.context()` for user-friendly messages~~
14. ~~Decay score rounding in table output~~
15. Coupling percentage display in output
16. New subcommands -- hotspots, authors-per-file, detailed ownership, author summary, age
17. CLI QoL -- help text, `--top N`, `--path` filter
18. Tests

## References

- Tornhill, Adam. *Your Code as a Crime Scene.*
- Tornhill, Adam. *Software Design X-Rays.*
- Gall, Hajek, Jazayeri (1998). "Detection of Logical Coupling Based on
  Product Release History."
