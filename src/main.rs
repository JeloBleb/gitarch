mod analysis;
mod cli;
mod repo;

use std::collections::HashMap;


use cliux::{Boxed, Table};

use crate::{
    analysis::{
        derived::get_decay,
        metrics::{SummaryStats,get_primary_owners, get_coupling, get_owners, get_summary},
    },
    cli::{Cli, Commands},
    repo::{CommitInfo, parse_commit_info},
};

use clap::Parser;

fn main() {
    let command = Cli::parse();

    let commits = parse_commit_info(&command.repo).unwrap();

    match command.command_type {
        Commands::Summary => print_summary(&commits),
        Commands::Decay => print_decay(&commits),
        Commands::Coupling => print_coupling(&commits),
        Commands::Ownership => print_owners(&commits),
    };
}

fn print_summary(commits: &[CommitInfo]) {
    let summary = get_summary(commits);
    let SummaryStats {
        commits,
        files,
        file_changes,
        authors,
    } = summary;
    let table = Table::new()
        .headers(&["Commits", "Files", "File Changes", "Authors"])
        .row(&[
            &commits.to_string(),
            &files.to_string(),
            &file_changes.to_string(),
            &authors.to_string(),
        ]);
    table.print();
}

fn print_decay(commits: &[CommitInfo]) {
    let decay = get_decay(commits);
    let mut table = Table::new().headers(&["File", "Decay Score"]);
    for (file, decay_score) in decay {
        table = table.row(&[&file, &decay_score.to_string()]);
    }

    table.print();
}

fn print_coupling(commits: &[CommitInfo]) {
    let coupling = get_coupling(commits);
    let coupling = coupling
        .into_iter()
        .filter(|p| p.1 > (commits.len() / 5))
        ;

    let mut table = Table::new().headers(&["File Pair", "Coupling"]);

    for (file_pair, coupling) in coupling {
        table = table.row(&[&format!("{} and {}", file_pair.0, file_pair.1), &coupling.to_string()])
    }

    table.print();
}

fn print_owners(commits: &[CommitInfo]) {
    let owners = get_primary_owners(&get_owners(commits));

    let mut table = Table::new().headers(&["File", "Owner"]);

    for (file, owner) in owners {
        table = table.row(&[&file, &owner]);
    }
    
    table.print();
}
