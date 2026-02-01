use std::path::Path;

use git2::*;

fn main() {
    let repo_path = Path::new(".");

    let repo = Repository::discover(repo_path).unwrap();

    let mut revwalk = repo.revwalk().unwrap();

    revwalk.push_head().unwrap();

    let latest_commit_hash = revwalk.next().unwrap().unwrap();
    let latest_commit = repo.find_commit(latest_commit_hash).unwrap();
    let latest_tree = latest_commit.tree().unwrap();
    let parent_commit = latest_commit.parent(0);

    let parent_tree = match parent_commit {
        Ok(parent_commit) => parent_commit.tree().unwrap(),
        Err(_) => empty_tree(&repo),
    };

    let diff = repo
        .diff_tree_to_tree(Some(&parent_tree), Some(&latest_tree), None)
        .unwrap();

    let diff_stats = diff.stats().unwrap();

    for delta in diff.deltas() {}

    println!(
        "in the diff, {} files changed, {} lines were inserted, and {} lines were deleted",
        diff_stats.files_changed(),
        diff_stats.insertions(),
        diff_stats.deletions()
    );
}

fn empty_tree(repo: &Repository) -> Tree<'_> {
    let oid = Oid::from_str("4b825dc642cb6eb9a060e54bf899d15363d7b169").unwrap();
    repo.find_tree(oid).unwrap()
}
