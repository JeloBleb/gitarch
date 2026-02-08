use std::{collections::HashMap, path::Path, result::Result};

use git2::{Delta, Repository};
use std::cell::RefCell;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
}

#[derive(Debug)]
pub struct CommitInfo {
    pub hash: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: i64,
    pub message: String,
    pub file_changes: Vec<FileChange>,
}

#[derive(Debug)]
pub struct FileChange {
    pub path: String,
    pub status: FileStatus,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Other,
}

impl From<Delta> for FileStatus {
    fn from(delta: Delta) -> FileStatus {
        match delta {
            Delta::Added => FileStatus::Added,
            Delta::Modified => FileStatus::Modified,
            Delta::Deleted => FileStatus::Deleted,
            Delta::Renamed => FileStatus::Renamed,
            Delta::Copied => FileStatus::Copied,
            _ => FileStatus::Other,
        }
    }
}

pub fn parse_commit_info(path: &Path) -> Result<Vec<CommitInfo>, RepoError> {
    let repo = Repository::discover(path)?;
    let mut commits: Vec<CommitInfo> = Vec::new();

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for latest_commit_hash in revwalk {
        let hash = latest_commit_hash?;
        let commit = repo.find_commit(hash)?;

        let file_changes = RefCell::new(HashMap::new());

        let tree = commit.tree()?;
        let parent_tree = commit.parent(0).ok().and_then(|p| p.tree().ok());

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;

        let current_file = RefCell::new("".to_string());

        diff.foreach(
            &mut |diff_delta, _progress| {
                let Some(path) = diff_delta.new_file().path() else {
                    return true;
                };

                let path = path.to_string_lossy().to_string();
                *current_file.borrow_mut() = path.clone();

                let status: FileStatus = diff_delta.status().into();

                file_changes.borrow_mut().insert(path, (status, 0, 0));

                true
            },
            None,
            None,
            Some(&mut |_diff_delta, _hunk, line| -> bool {
                if let Some((_, insertions, deletions)) =
                    file_changes.borrow_mut().get_mut(&*current_file.borrow())
                {
                    match line.origin() {
                        '+' => *insertions += 1,
                        '-' => *deletions += 1,
                        _ => {}
                    }
                }

                true
            }),
        )?;

        let file_changes: Vec<_> = file_changes
            .into_inner()
            .into_iter()
            .map(|(path, (status, insertions, deletions))| FileChange {
                path,
                status,
                insertions,
                deletions,
            })
            .collect();

        let hash = hash.to_string();
        let commit_info = commit.author();
        let (author_name, author_email, timestamp) = (
            commit_info.name().unwrap_or("").to_string(),
            commit_info.email().unwrap_or("").to_string(),
            commit_info.when().seconds(),
        );

        commits.push(CommitInfo {
            hash,
            author_name,
            author_email,
            timestamp,
            message: commit.message().unwrap_or("").to_string(),
            file_changes,
        });
    }

    Ok(commits)
}
