use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use git2::*;
use itertools::Itertools;

fn main() {
    let repo_path = Path::new(".");

    let repo = Repository::discover(repo_path).unwrap();

    let mut revwalk = repo.revwalk().unwrap();

    revwalk.push_head().unwrap();

    let mut file_owners: HashMap<String, HashSet<String>> = HashMap::new();
    let mut coupled_files: HashMap<(String, String), usize> = HashMap::new();

    for latest_commit_hash in revwalk {
        let mut changed_files: Vec<String> = Vec::new();

        let latest_commit_hash = latest_commit_hash.unwrap();

        let latest_commit = repo.find_commit(latest_commit_hash).unwrap();
        let latest_tree = latest_commit.tree().unwrap();
        let parent_tree = latest_commit.parent(0).ok().map(|p| p.tree().unwrap());

        let authour = latest_commit.author().name().unwrap().to_string();

        let diff = repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&latest_tree), None)
            .unwrap();

        let diff_stats = diff.stats().unwrap();

        println!(
            "in the diff, {} files changed, {} lines were inserted, and {} lines were deleted",
            diff_stats.files_changed(),
            diff_stats.insertions(),
            diff_stats.deletions()
        );

        for delta in diff.deltas() {
            let file_name = delta
                .new_file()
                .path()
                .unwrap()
                .to_string_lossy()
                .to_string();
            println!("file {} was {:?}", file_name, delta.status());

            changed_files.push(file_name.clone());

            file_owners
                .entry(file_name)
                .or_default()
                .insert(authour.clone());
        }

        changed_files.sort();

        for pair in changed_files.iter().combinations(2) {
            *coupled_files
                .entry((pair[0].clone(), pair[1].clone()))
                .or_default() += 1;
        }

        println!();
    }

    println!("file_owners: {file_owners:?}");

    println!("coupled files: {coupled_files:?}");
}
