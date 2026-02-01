use std::path::Path;

use git2::*;

fn main() {
    let repo_path = Path::new(".");

    let repo = Repository::discover(repo_path).unwrap();

    let mut revwalk = repo.revwalk().unwrap();

    revwalk.push_head().unwrap();

    for commit_hash in revwalk {
        let commit_hash = commit_hash.unwrap();
        let commit = repo.find_commit(commit_hash).unwrap();
        println!(
            "author: {}, message: {}, time: {}, id: {}",
            commit.author().name().unwrap(),
            commit.message().unwrap(),
            commit.time().seconds(),
            commit.id()
        );
    }
}
