mod analysis;
mod cli;
mod repo;

use crate::{cli::Cli, repo::parse_commit_info};
use clap::Parser;

fn main() {
    let command = Cli::parse();

    let _commit_info = parse_commit_info(&command.repo).unwrap();

    println!("Your command was {command:?}")
}
