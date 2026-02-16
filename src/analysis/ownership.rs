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

pub fn get_file_concentrations(
    file_owners: &HashMap<String, HashMap<String, usize>>,
    file_primary_owners: &HashMap<String, String>,
) -> HashMap<String, f64> {
    let mut files: HashMap<String, f64> = HashMap::new();

    for (path, authors) in file_owners {
        let concentration = *authors
            .get(
                file_primary_owners
                    .get(path)
                    .expect("primary owner invalid"),
            )
            .unwrap() as f64
            / authors.values().sum::<usize>() as f64;

        files.insert(path.clone(), concentration);
    }

    files
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
