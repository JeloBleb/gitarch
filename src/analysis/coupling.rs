use std::{collections::HashMap, usize};

use itertools::Itertools;

use crate::repo::CommitInfo;

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
