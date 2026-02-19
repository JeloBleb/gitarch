use std::collections::HashMap;

use crate::repo::CommitInfo;
use itertools::Itertools;

#[derive(Debug)]
pub struct SummaryStats {
    commits: usize,
    files: usize,
    file_changes: usize,
    authors: usize,
}

pub fn get_summary(commits: &[CommitInfo]) -> SummaryStats {
    let files = get_owners(commits).len();

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

pub fn get_coupling(commits: &[CommitInfo]) -> HashMap<(String, String), usize> {
    let mut couplings: HashMap<(String, String), usize> = HashMap::new();

    for commit in commits {
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
pub fn get_primary_owners(
    file_owners: &HashMap<String, HashMap<String, usize>>,
) -> HashMap<String, String> {
    let file_primary_owners: HashMap<String, String> = file_owners
        .iter()
        .map(|p| {
            (
                p.0.clone(),
                p.1.iter().max_by_key(|p| p.1).unwrap().0.clone(),
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
