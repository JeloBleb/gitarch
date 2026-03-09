mod analysis;
mod cli;
mod repo;

use cliux::Table;
use itertools::Itertools;
use std::collections::HashMap;

use crate::{
    analysis::{
        derived::get_decay,
        metrics::{
            SummaryStats, get_coupling, get_file_statuses, get_owners, get_primary_owners,
            get_summary,
        },
    },
    cli::{Cli, Commands},
    repo::{CommitInfo, FileChange, FileStatus, parse_commit_info},
};

use clap::Parser;

fn main() {
    let command = Cli::parse();

    let commits = parse_commit_info(&command.repo).unwrap();

    match command.command_type {
        Commands::Summary => print_summary(&commits),
        Commands::Decay => print_decay(&commits),
        Commands::Coupling { max_changeset_size } => print_coupling(&commits, max_changeset_size),
        Commands::Ownership => print_owners(&commits),
    };
}

fn filter_deleted<V>(files: HashMap<String, V>, commits: &[CommitInfo]) -> HashMap<String, V> {
    let file_statuses = get_file_statuses(commits);
    files
        .into_iter()
        .filter(|p| {
            *file_statuses
                .get(&p.0)
                .expect("mismatch between file status and other hashmap")
                != FileStatus::Deleted
        })
        .collect::<HashMap<String, V>>()
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
    let decay = filter_deleted(decay, commits);

    let mut table = Table::new().headers(&["File", "Decay Score"]);

    for (file, decay_score) in decay.into_iter().sorted_by(|a, b| b.1.total_cmp(&a.1)) {
        table = table.row(&[&file, &decay_score.to_string()]);
    }

    table.print();
}

fn print_coupling(commits: &[CommitInfo], max_changeset_size: usize) {
    let coupling = get_coupling(commits, max_changeset_size);

    let file_statuses = get_file_statuses(commits);

    let coupling = coupling
        .into_iter()
        .filter(|p| p.1 > (commits.len() / 5))
        .filter(|p| {
            file_statuses.get(&p.0.0) != Some(&FileStatus::Deleted)
                && file_statuses.get(&p.0.1) != Some(&FileStatus::Deleted)
        });

    let mut table = Table::new().headers(&["File Pair", "Coupling"]);

    for (file_pair, coupling) in coupling {
        table = table.row(&[
            &format!("{} and {}", file_pair.0, file_pair.1),
            &coupling.to_string(),
        ])
    }

    table.print();
}

fn print_owners(commits: &[CommitInfo]) {
    let owners = get_primary_owners(&get_owners(commits));
    let owners = filter_deleted(owners, commits);

    let mut table = Table::new().headers(&["File", "Owner"]);

    for (file, owner) in owners {
        table = table.row(&[&file, &owner]);
    }

    table.print();
}
