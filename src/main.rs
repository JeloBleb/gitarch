mod cli;

use std::collections::{HashMap, HashSet};

use git2::{DiffStats, Repository};
use itertools::Itertools;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let command = Cli::parse();

    let repo_path = command.repo;

    let repo = Repository::discover(repo_path).unwrap();

    let mut revwalk = repo.revwalk().unwrap();

    revwalk.push_head().unwrap();

    let mut file_owners: HashMap<String, HashSet<String>> = HashMap::new();
    let mut coupled_files: HashMap<(String, String), usize> = HashMap::new();
    let mut diff_stats_collection: Vec<(DiffStats, Vec<(String, String)>)> = Vec::new();

    for latest_commit_hash in revwalk {
        let mut changed_files: Vec<String> = Vec::new();

        let commit_hash = latest_commit_hash.unwrap();

        let commit = repo.find_commit(commit_hash).unwrap();
        let latest_tree = commit.tree().unwrap();
        let parent_tree = commit.parent(0).ok().map(|p| p.tree().unwrap());

        let authour = commit.author().name().unwrap().to_string();

        let diff = repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&latest_tree), None)
            .unwrap();

        let mut deltas_collection: Vec<(String, String)> = Vec::new();

        let deltas = diff.deltas();

        for delta in deltas {
            let file_name = delta
                .new_file()
                .path()
                .unwrap()
                .to_string_lossy()
                .to_string();

            changed_files.push(file_name.clone());

            file_owners
                .entry(file_name.clone())
                .or_default()
                .insert(authour.clone());

            deltas_collection.push((file_name, format!("{:?}", delta.status())));
        }

        changed_files.sort();

        for pair in changed_files.iter().combinations(2) {
            *coupled_files
                .entry((pair[0].clone(), pair[1].clone()))
                .or_default() += 1;
        }

        diff_stats_collection.push((diff.stats().unwrap(), deltas_collection));
    }

    match command.command {
        Commands::Knowledge => println!("file_owners: {file_owners:?}"),
        Commands::Coupling => println!("coupled files: {coupled_files:?}"),
        Commands::Diffs => {
            for (diff_stats, deltas_collection) in diff_stats_collection {
                println!(
                    "in the diff, {} files changed, {} lines were inserted, and {} lines were deleted",
                    diff_stats.files_changed(),
                    diff_stats.insertions(),
                    diff_stats.deletions()
                );

                for (file_name, delta) in deltas_collection {
                    println!("file {file_name} was {delta}");
                }
            }
        }
    }
}
