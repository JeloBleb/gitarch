mod analysis;
mod cli;
mod repo;

use cliux::Table;
use itertools::Itertools;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::collections::HashMap;

use crate::{
    analysis::{
        derived::get_decay,
        metrics::{
            SummaryStats, get_coupling, get_file_statuses, get_owner_coupling, get_owners,
            get_primary_owners, get_summary,
        },
    },
    cli::{Cli, Commands},
    repo::{CommitInfo, FileStatus, parse_commit_info},
};

use clap::Parser;

#[derive(Serialize)]
struct DecayEntry {
    file: String,
    score: f64,
}

#[derive(Serialize)]
struct CouplingEntry {
    file_pair: (String, String),
    count: usize,
}

#[derive(Serialize)]
struct OwnershipEntry {
    file: String,
    owner: String,
}

#[derive(Serialize)]
struct CommunicationEntry {
    owner_pair: (String, String),
    count: usize,
}

struct ChurnEntry {
    file: String,
    revisions: usize,
    insertions: usize,
    deletions: usize,
    created: i64,
    last_modified: i64,
}

fn main() {
    let command = Cli::parse();

    let commits = parse_commit_info(&command.repo).unwrap();

    match command.command_type {
        Commands::Summary => print_summary(&commits, command.json),
        Commands::Decay => print_decay(&commits, command.json),
        Commands::Coupling { max_changeset_size } => {
            print_coupling(&commits, max_changeset_size, command.json)
        }
        Commands::Ownership => print_owners(&commits, command.json),
        Commands::Communication => print_communication(&commits, command.json),
        Commands::Churn => print_churn(&commits, command.json),
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

fn print_summary(commits: &[CommitInfo], json_out: bool) {
    let summary = get_summary(commits);

    if json_out {
        let json = to_string_pretty(&summary).unwrap();
        println!("{json}");
    } else {
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
}

fn print_decay(commits: &[CommitInfo], json_out: bool) {
    let decay = get_decay(commits);
    let decay = filter_deleted(decay, commits);
    let decay = decay
        .into_iter()
        .map(|(file, score)| DecayEntry { file, score })
        .sorted_by(|a, b| b.score.total_cmp(&a.score));

    if json_out {
        let json = to_string_pretty(&decay.collect::<Vec<DecayEntry>>()).unwrap();
        println!("{json}");
    } else {
        let mut table = Table::new().headers(&["File", "Decay Score"]);

        for DecayEntry { file, score } in decay {
            table = table.row(&[&file, &score.to_string()]);
        }

        table.print();
    }
}

fn print_coupling(commits: &[CommitInfo], max_changeset_size: usize, json_out: bool) {
    let coupling = get_coupling(commits, max_changeset_size);

    let file_statuses = get_file_statuses(commits);

    let coupling = coupling
        .into_iter()
        .filter(|p| p.1 > (commits.len() / 5))
        .filter(|p| {
            file_statuses.get(&p.0.0) != Some(&FileStatus::Deleted)
                && file_statuses.get(&p.0.1) != Some(&FileStatus::Deleted)
        })
        .map(|(file_pair, count)| CouplingEntry { file_pair, count });

    if json_out {
        let json = serde_json::to_string_pretty(&coupling.collect::<Vec<CouplingEntry>>()).unwrap();
        println!("{json}");
    } else {
        let mut table = Table::new().headers(&["File Pair", "Coupling"]);

        for CouplingEntry { file_pair, count } in coupling {
            table = table.row(&[
                &format!("{} and {}", file_pair.0, file_pair.1),
                &count.to_string(),
            ])
        }

        table.print();
    }
}

fn print_owners(commits: &[CommitInfo], json_out: bool) {
    let owners = get_primary_owners(&get_owners(commits));
    let owners = filter_deleted(owners, commits)
        .into_iter()
        .map(|(file, owner)| OwnershipEntry { file, owner });

    if json_out {
        let json = to_string_pretty(&owners.collect::<Vec<OwnershipEntry>>()).unwrap();
        println!("{json}");
    } else {
        let mut table = Table::new().headers(&["File", "Owner"]);

        for OwnershipEntry { file, owner } in owners {
            table = table.row(&[&file, &owner]);
        }

        table.print();
    }
}

fn print_communication(commits: &[CommitInfo], json_out: bool) {
    let owner_coupling = get_owner_coupling(commits);
    let owner_coupling = owner_coupling
        .into_iter()
        .map(|(owner_pair, count)| CommunicationEntry { owner_pair, count });

    if json_out {
        let json = to_string_pretty(&owner_coupling.collect::<Vec<CommunicationEntry>>()).unwrap();
        println!("{json}");
    } else {
        let mut table = Table::new().headers(&["Owner Pair", "File Overlap"]);

        for CommunicationEntry { owner_pair, count } in owner_coupling {
            table = table.row(&[
                &format!("{} and {}", owner_pair.0, owner_pair.1),
                &count.to_string(),
            ]);
        }

        table.print();
    }
}
