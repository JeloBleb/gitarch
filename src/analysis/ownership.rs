use std::collections::HashMap;

use crate::repo::CommitInfo;

pub fn get_ownership(commits: &[CommitInfo]) -> HashMap<String, HashMap<String, usize>> {
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
