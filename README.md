# gitarch

[![CI](https://github.com/JeloBleb/gitarch/actions/workflows/ci.yml/badge.svg)](https://github.com/JeloBleb/gitarch/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

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

## Installation

```bash
cargo install gitarch
```

Or build from source:

```bash
git clone https://github.com/JeloBleb/gitarch.git
cd gitarch
cargo install --path .
```

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
- **Summary** -- repo-wide stats (total commits, files, authors)

### Additional analyses
- **Communication** -- developer coupling inferred from shared file ownership
- **Owner summary** -- per-author stats (files owned, last active, commit count)

### Planned
- **Parallel analysis** -- rayon-based parallelism for large repos

## Usage

```bash
gitarch summary                        # repo-wide stats
gitarch ownership                      # primary owner per file
gitarch coupling                       # co-change pairs
gitarch decay                          # composite decay scores
gitarch churn                          # lines added/deleted per file
gitarch communication                  # developer coupling via shared files
gitarch owner-summary                  # per-author breakdown
```

### Example output

```
$ gitarch summary
+---------+-------+--------------+---------+
| Commits | Files | File Changes | Authors |
+---------+-------+--------------+---------+
| 29      | 12    | 115          | 2       |
+---------+-------+--------------+---------+

$ gitarch --top 5 coupling
+-------------------------------+----------+
| File Pair                     | Coupling |
+-------------------------------+----------+
| Cargo.lock and Cargo.toml     | 100%     |
| Cargo.lock and src/main.rs    | 100%     |
| Cargo.toml and src/main.rs    | 100%     |
| src/cli.rs and src/main.rs    | 93%      |
| src/main.rs and src/output.rs | 88%      |
+-------------------------------+----------+

$ gitarch --top 5 decay
+-----------------+-------------+
| File            | Decay Score |
+-----------------+-------------+
| .gitignore      | 0.28        |
| src/analysis.rs | 0.21        |
| Cargo.toml      | 0.14        |
| Cargo.lock      | 0.14        |
| AGENTS.md       | 0.09        |
+-----------------+-------------+
```

All commands support `--json` for machine-readable output:

```bash
gitarch --json decay | jq '.[] | select(.score > 0.5)'
```

### Global flags
- `--repo <path>` -- analyze a different repository (defaults to `.`)
- `--json` -- machine-readable JSON output
- `--top <N>` -- limit output to the top N results
- `--since <YYYY-MM-DD>` -- only include commits from this date onward
- `--until <YYYY-MM-DD>` -- only include commits up to this date
- `--path <dir>` -- scope analysis to a subdirectory
- `--include <pattern>` -- only include files matching suffix (e.g. `.rs`); repeatable
- `--exclude <pattern>` -- exclude files matching suffix; repeatable

### Subcommand flags
- `coupling --max-changeset-size <N>` -- ignore commits touching more than N
  files (default: 20)
- `coupling --min-coupling-percentage <N>` -- minimum coupling percentage to
  display (default: 15)
- `coupling --min-revision-count <N>` -- minimum number of revisions for a
  file to be included in coupling analysis (default: 5)
- `decay --decay-threshold <DAYS>` -- number of days until a file is
  considered fully stale (default: 180)

### Shell completions

Generate completions for your shell:

```bash
gitarch completions bash > ~/.local/share/bash-completion/completions/gitarch
gitarch completions zsh > ~/.zfunc/_gitarch
gitarch completions fish > ~/.config/fish/completions/gitarch.fish
```

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
- **clap_complete** -- shell completion generation
- **cliux** -- terminal table output
- **thiserror** -- typed errors in library code
- **anyhow** -- error handling in CLI layer
- **itertools** -- combinatorics for coupling analysis
- **serde** + **serde_json** -- JSON serialization for output
- **chrono** -- date parsing and formatting
- **rayon** -- parallel analysis (planned)

## License

[MIT](LICENSE)

## References

- Tornhill, Adam. *Your Code as a Crime Scene.*
- Tornhill, Adam. *Software Design X-Rays.*
- Gall, Hajek, Jazayeri (1998). "Detection of Logical Coupling Based on
  Product Release History."
