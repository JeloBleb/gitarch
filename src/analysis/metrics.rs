use std::collections::HashMap;

use crate::repo::CommitInfo;

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
