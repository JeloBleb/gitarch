# Changelog

## 0.1.0

Initial release.

### Features
- **Ownership** -- per-file ownership map with primary owner detection
- **Coupling** -- co-change file pair analysis with configurable filters
- **Decay** -- composite decay scoring (staleness + owner inactivity + concentration)
- **Churn** -- lines added/deleted per file with revision counts
- **Communication** -- developer coupling inferred from shared file ownership
- **Owner summary** -- per-author statistics
- **Summary** -- repo-wide stats (commits, files, authors)
- **Shell completions** -- bash, zsh, fish, PowerShell via `completions` subcommand
- Global flags: `--json`, `--top`, `--since`, `--until`, `--path`, `--include`, `--exclude`
- Direct git2 repository access (no log file export)
- JSON output for LLM consumption
