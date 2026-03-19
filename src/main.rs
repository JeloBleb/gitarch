mod analysis;
mod cli;
mod output;
mod repo;

use output::*;

use chrono::{DateTime, NaiveDate};

use crate::{
    cli::{Cli, Commands},
    repo::{CommitInfo, FileStatus, parse_commit_info},
};

use clap::Parser;
fn main() -> anyhow::Result<()> {
    let command = Cli::parse();

    let commits = parse_commit_info(&command.repo)?;

    let filtered_commits: Vec<CommitInfo> = commits
        .iter()
        .filter(|commit| {
            let date = DateTime::from_timestamp(commit.timestamp, 0)
                .unwrap()
                .date_naive();
            command.since.is_none_or(|p| date >= p) && command.until.is_none_or(|p| date <= p)
        })
        .cloned()
        .collect();

    match command.command_type {
        Commands::Summary => print_summary(&filtered_commits, command.json),
        Commands::Decay { decay_threshold } => {
            print_decay(&filtered_commits, decay_threshold, command.json)
        }
        Commands::Coupling {
            max_changeset_size,
            coupling_percentage,
        } => print_coupling(
            &commits,
            max_changeset_size,
            coupling_percentage,
            command.json,
        ),
        Commands::Ownership => print_owners(&filtered_commits, command.json),
        Commands::Communication => print_communication(&filtered_commits, command.json),
        Commands::Churn => print_churn(&commits, &filtered_commits, command.json),
    };

    Ok(())
}
