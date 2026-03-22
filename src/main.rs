mod analysis;
mod cli;
mod output;
mod repo;

use anyhow::Context;

use chrono::{DateTime, NaiveDate};

use crate::{cli::*, output::*, repo::*};

use clap::Parser;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let command = Cli::parse();

    let config = command.config;

    let commits = parse_commit_info(&command.repo).context("Failed to read respository")?;

    let filtered_commits: Vec<CommitInfo> = commits
        .iter()
        .filter(|commit| {
            let date = DateTime::from_timestamp(commit.timestamp, 0)
                .unwrap()
                .date_naive();
            config.since.is_none_or(|p| date >= p) && config.until.is_none_or(|p| date <= p)
        })
        .cloned()
        .collect();

    match command.command_type {
        Commands::Summary => print_summary(&filtered_commits, config),
        Commands::Decay { decay_threshold } => {
            print_decay(&filtered_commits, decay_threshold, config)
        }
        Commands::Coupling {
            max_changeset_size,
            coupling_percentage,
        } => print_coupling(
            &filtered_commits,
            max_changeset_size,
            coupling_percentage,
            config,
        ),
        Commands::Ownership => print_owners(&filtered_commits, config),
        Commands::Communication => print_communication(&filtered_commits, config),
        Commands::Churn => print_churn(&commits, &filtered_commits, config),
    };

    Ok(())
}
