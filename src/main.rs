mod analysis;
mod cli;
mod output;
mod repo;

use output::*;

use crate::{
    cli::{Cli, Commands},
    repo::{CommitInfo, FileStatus, parse_commit_info},
};

use clap::Parser;
fn main() {
    let command = Cli::parse();

    let commits = parse_commit_info(&command.repo).unwrap();

    match command.command_type {
        Commands::Summary => print_summary(&commits, command.json),
        Commands::Decay => print_decay(&commits, command.json),
        Commands::Coupling {
            max_changeset_size,
            coupling_percentage,
        } => print_coupling(
            &commits,
            max_changeset_size,
            coupling_percentage,
            command.json,
        ),
        Commands::Ownership => print_owners(&commits, command.json),
        Commands::Communication => print_communication(&commits, command.json),
        Commands::Churn { since, until } => print_churn(&commits, since, until, command.json),
    };
}
