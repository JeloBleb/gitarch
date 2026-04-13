mod analysis;
mod cli;
mod output;
mod repo;

use anyhow::Context;

use chrono::{DateTime, NaiveDate};

use crate::{cli::*, output::*, repo::*};

use clap::{CommandFactory, Parser};
use clap_complete::generate;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let command = Cli::parse();

    if let Commands::Completions { shell } = &command.command_type {
        generate(
            *shell,
            &mut Cli::command(),
            "gitarch",
            &mut std::io::stdout(),
        );
        return Ok(());
    }

    let config = command.config;

    let commits =
        parse_commit_info(&command.repo, config.clone()).context("Failed to read respository")?;

    let filtered_commits: Vec<CommitInfo> = commits
        .iter()
        .cloned()
        .filter_map(|commit| {
            let date = DateTime::from_timestamp(commit.timestamp, 0)?.date_naive();
            let in_range =
                config.since.is_none_or(|p| date >= p) && config.until.is_none_or(|p| date <= p);
            in_range.then_some(commit)
        })
        .collect();

    match command.command_type {
        Commands::Summary => print_summary(&filtered_commits, config)?,
        Commands::Decay { decay_threshold } => {
            print_decay(&filtered_commits, decay_threshold, config)?
        }
        Commands::Coupling {
            max_changeset_size,
            min_coupling_percentage,
            min_revision_count,
        } => print_coupling(
            &filtered_commits,
            max_changeset_size,
            min_coupling_percentage,
            min_revision_count,
            config,
        )?,
        Commands::Ownership => print_owners(&filtered_commits, config)?,
        Commands::Communication => print_communication(&filtered_commits, config)?,
        Commands::Churn => print_churn(&commits, &filtered_commits, config)?,
        Commands::OwnerSummary => print_owner_summary(&filtered_commits, config)?,
        Commands::Completions { .. } => unreachable!(),
    };

    Ok(())
}
