use crate::analysis::{derived::*, metrics::*};
use crate::*;
use chrono::{DateTime, NaiveDate};
use cliux::Table;
use itertools::Itertools;
use serde::Serialize;
use serde_json::to_string_pretty;

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

#[derive(Serialize)]
struct ChurnEntry {
    file: String,
    revisions: usize,
    insertions: usize,
    deletions: usize,
    created: NaiveDate,
    last_modified: NaiveDate,
}
pub fn print_summary(commits: &[CommitInfo], json_out: bool) {
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

pub fn print_decay(commits: &[CommitInfo], json_out: bool) {
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

pub fn print_coupling(
    commits: &[CommitInfo],
    max_changeset_size: usize,
    coupling_percent: usize,
    json_out: bool,
) {
    let coupling = get_coupling(commits, max_changeset_size);
    let revisions = get_revision_counts(commits);

    let file_statuses = get_file_statuses(commits);

    let coupling = coupling
        .into_iter()
        .filter(|p| {
            p.1 > ((revisions.get(&p.0.0).unwrap() + revisions.get(&p.0.1).unwrap()) / 2
                * coupling_percent
                / 100)
        })
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

pub fn print_owners(commits: &[CommitInfo], json_out: bool) {
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

pub fn print_communication(commits: &[CommitInfo], json_out: bool) {
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

pub fn print_churn(
    commits: &[CommitInfo],
    since: Option<NaiveDate>,
    until: Option<NaiveDate>,
    json_out: bool,
) {
    let last_modified = get_files_last_modified(commits);
    let created = get_files_creation(commits);

    let commits: Vec<CommitInfo> = commits
        .iter()
        .filter(|commit| {
            let date = DateTime::from_timestamp(commit.timestamp, 0)
                .unwrap()
                .date_naive();
            since.is_none_or(|p| date >= p) && until.is_none_or(|p| date <= p)
        })
        .cloned()
        .collect();

    let line_changes = get_line_changes(&commits);
    let line_changes = filter_deleted(line_changes, &commits);
    let revisions = get_revision_counts(&commits);

    let mut churn_entries: Vec<ChurnEntry> = Vec::new();

    for (file, (insertions, deletions)) in line_changes {
        let revisions = *revisions.get(&file).unwrap();
        let created = DateTime::from_timestamp(*created.get(&file).unwrap(), 0)
            .unwrap()
            .date_naive();
        let last_modified = DateTime::from_timestamp(*last_modified.get(&file).unwrap(), 0)
            .unwrap()
            .date_naive();
        churn_entries.push(ChurnEntry {
            file,
            revisions,
            insertions,
            deletions,
            created,
            last_modified,
        });
    }

    if json_out {
        let json = to_string_pretty(&churn_entries).unwrap();
        println!("{json}");
    } else {
        let mut table = Table::new().headers(&[
            "File",
            "Revisions",
            "Insertions",
            "Deletions",
            "Created",
            "Last Modified",
        ]);

        for ChurnEntry {
            file,
            revisions,
            insertions,
            deletions,
            created,
            last_modified,
        } in churn_entries
        {
            table = table.row(&[
                &file,
                &revisions.to_string(),
                &insertions.to_string(),
                &deletions.to_string(),
                &created.to_string(),
                &last_modified.to_string(),
            ])
        }

        table.print();
    }
}
