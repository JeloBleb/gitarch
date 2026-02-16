# gitarch

Extracts implicit knowledge from git repository commit history.

## Features

- **Ownership** -- per-file ownership and bus factor analysis
- **Coupling** -- files that frequently change together, revealing hidden dependencies
- **Decay** -- distinguishes stable code from abandoned code with no active owner
- **Evolution** -- architecture timeline showing structural changes over time
- **Health** -- aggregate project health metrics derived from commit patterns

## Usage

```bash
gitarch ownership .
gitarch coupling .
gitarch decay --months 6
gitarch evolution
gitarch health
```

Use `--json` for machine-readable output.

## Architecture

```
src/
  main.rs           # clap CLI entry point
  cli.rs            # subcommand definitions
  repo.rs           # git2 data access layer
  analysis.rs       # analysis module root
  analysis/
    ownership.rs    # file ownership / bus factor
    coupling.rs     # change coupling
    decay.rs        # ownership decay
    evolution.rs    # architecture timeline
    health.rs       # project health metrics
  output/
    terminal.rs     # human-readable output
    json.rs         # machine-readable JSON output
```

Data flow: `git2 repo -> Vec<CommitInfo> -> analysis modules -> output formatters`

## Tech Stack

- **git2** -- libgit2 bindings for repository access
- **clap** (derive) -- CLI parsing
- **thiserror** -- typed errors in library code
- **anyhow** -- error handling in CLI layer
- **itertools** -- combinatorics for coupling analysis
- **rayon** -- parallel analysis (planned)
- **serde** -- structured output (planned)

## Build Order

1. ~~Scaffolding -- CLI, git2 repo access, commit walker~~
2. ~~Ownership -- per-file ownership map~~
3. ~~Coupling -- change coupling analysis~~
4. Decay -- ownership decay over time
5. Evolution -- structural event detection
6. Health -- aggregate metrics (combines other analyses)

## References

- Tornhill, Adam. "Your Code as a Crime Scene."
- Gall, Hajek, Jazayeri (1998). "Detection of Logical Coupling Based on Product Release History."
