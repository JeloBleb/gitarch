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

### Planned
- **Communication** -- developer coupling inferred from shared file ownership
- **Evolution** -- structural event detection (file births, deaths, renames)
- **Health** -- aggregate project health score combining all analyses
- **Architectural grouping** -- aggregate file-level analyses to logical
  component level via regex mapping

## Usage

```bash
gitarch ownership .
gitarch coupling .
gitarch decay .
gitarch churn .
gitarch summary .
gitarch health .
```

Use `--json` for machine-readable output.

## Architecture

```
src/
  main.rs           # clap CLI entry point
  cli.rs            # subcommand definitions
  repo.rs           # git2 data access layer
  analysis/
    metrics.rs      # raw data extraction (ownership, counts, timestamps, churn)
    coupling.rs     # change coupling + coupling percentage
    decay.rs        # decay scoring (consumes metrics)
    evolution.rs    # structural event detection (planned)
    health.rs       # aggregate project health (planned)
  output/
    terminal.rs     # human-readable terminal output
    json.rs         # structured JSON output (LLM-friendly)
```

Data flow: `git2 repo -> Vec<CommitInfo> -> metrics -> analysis -> output`

## Tech Stack

- **git2** -- libgit2 bindings for direct repository access
- **clap** (derive) -- CLI parsing
- **thiserror** -- typed errors in library code
- **anyhow** -- error handling in CLI layer
- **itertools** -- combinatorics for coupling analysis
- **serde** -- JSON serialization for output
- **rayon** -- parallel analysis (planned)

## Build Order

1. ~~Scaffolding -- CLI, git2 repo access, commit walker~~
2. ~~Core metrics -- ownership, revision counts, churn, timestamps~~
3. ~~Coupling -- raw co-change counts~~
4. ~~Decay -- composite decay scoring~~
5. Module reorganization (metrics.rs split)
6. Coupling percentage + CLI filters
7. Summary, author churn, absolute churn, communication
8. Wire up CLI + terminal output
9. JSON output
10. Evolution -- structural event detection
11. Health -- aggregate project health
12. Tests

## References

- Tornhill, Adam. *Your Code as a Crime Scene.*
- Tornhill, Adam. *Software Design X-Rays.*
- Gall, Hajek, Jazayeri (1998). "Detection of Logical Coupling Based on
  Product Release History."
