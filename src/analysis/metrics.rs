use std::collections::HashMap;

use crate::repo::{CommitInfo, FileStatus};
use itertools::Itertools;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SummaryStats {
    pub commits: usize,
    pub files: usize,
    pub file_changes: usize,
    pub authors: usize,
}

pub fn filter_deleted<V>(files: HashMap<String, V>, commits: &[CommitInfo]) -> HashMap<String, V> {
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

pub fn get_summary(commits: &[CommitInfo]) -> SummaryStats {
    let files = get_file_statuses(commits)
        .iter()
        .filter(|p| *p.1 != FileStatus::Deleted)
        .count();

    let file_changes = commits.iter().map(|p| p.file_changes.len()).sum();

    let authors = get_user_last_active(commits).len();

    let commits = commits.len();

    SummaryStats {
        commits,
        files,
        file_changes,
        authors,
    }
}

pub fn get_file_statuses(commits: &[CommitInfo]) -> HashMap<String, FileStatus> {
    let mut file_statuses: HashMap<String, FileStatus> = HashMap::new();

    for commit in commits {
        for file in &commit.file_changes {
            file_statuses
                .entry(file.path.clone())
                .or_insert(file.status);
        }
    }

    file_statuses
}

pub fn get_owners(commits: &[CommitInfo]) -> HashMap<String, HashMap<String, usize>> {
    let mut files: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for commit in commits {
        let author = commit.author_name.clone();
        for file in &commit.file_changes {
            *files
                .entry(file.path.clone())
                .or_default()
                .entry(author.clone())
                .or_default() += 1;
        }
    }

    files
}

pub fn get_coupling(
    commits: &[CommitInfo],
    max_changeset_size: usize,
) -> HashMap<(String, String), usize> {
    let mut couplings: HashMap<(String, String), usize> = HashMap::new();

    for commit in commits
        .iter()
        .filter(|p| p.file_changes.len() <= max_changeset_size)
    {
        let mut changed_files: Vec<String> =
            commit.file_changes.iter().map(|p| p.path.clone()).collect();

        changed_files.sort();

        for entry in changed_files.iter().combinations(2) {
            *couplings
                .entry((entry[0].clone(), entry[1].clone()))
                .or_default() += 1;
        }
    }

    couplings
}

pub fn get_owner_coupling(commits: &[CommitInfo]) -> HashMap<(String, String), usize> {
    let mut owner_coupling: HashMap<(String, String), usize> = HashMap::new();
    let file_owners = get_owners(commits);

    for (_file, owners) in file_owners {
        for owner_pair in owners.keys().sorted().combinations(2) {
            *owner_coupling
                .entry((owner_pair[0].clone(), owner_pair[1].clone()))
                .or_default() += 1;
        }
    }

    owner_coupling
}

pub fn get_primary_owners(
    file_owners: &HashMap<String, HashMap<String, usize>>,
) -> HashMap<String, String> {
    let file_primary_owners: HashMap<String, String> = file_owners
        .iter()
        .map(|p| {
            (
                p.0.clone(),
                p.1.iter()
                    .max_by_key(|p| p.1)
                    .expect("iterator was empty")
                    .0
                    .clone(),
            )
        })
        .collect();
    file_primary_owners
}

pub fn get_user_last_active(commits: &[CommitInfo]) -> HashMap<String, i64> {
    let mut users: HashMap<String, i64> = HashMap::new();

    for commit in commits {
        users
            .entry(commit.author_name.clone())
            .or_insert(commit.timestamp);
    }

    users
}
pub fn get_revision_counts(commits: &[CommitInfo]) -> HashMap<String, usize> {
    let mut revision_counts: HashMap<String, usize> = HashMap::new();

    for commit in commits {
        commit
            .file_changes
            .iter()
            .for_each(|p| *revision_counts.entry(p.path.clone()).or_insert(0) += 1);
    }

    revision_counts
}

pub fn get_line_changes(commits: &[CommitInfo]) -> HashMap<String, (usize, usize)> {
    let mut line_changes: HashMap<String, (usize, usize)> = HashMap::new();

    for commit in commits {
        for file in &commit.file_changes {
            let (insertions, deletions) = line_changes.entry(file.path.clone()).or_insert((0, 0));
            *insertions += file.insertions;
            *deletions += file.deletions;
        }
    }

    line_changes
}

pub fn get_files_last_modified(commits: &[CommitInfo]) -> HashMap<String, i64> {
    let mut timestamps: HashMap<String, i64> = HashMap::new();

    for commit in commits {
        for file in &commit.file_changes {
            timestamps
                .entry(file.path.clone())
                .or_insert(commit.timestamp);
        }
    }

    timestamps
}

pub fn get_files_creation(commits: &[CommitInfo]) -> HashMap<String, i64> {
    let mut timestamps: HashMap<String, i64> = HashMap::new();

    for commit in commits {
        for file in &commit.file_changes {
            timestamps.insert(file.path.clone(), commit.timestamp);
        }
    }

    timestamps
}
