mod analysis;
mod cli;
mod repo;

use crate::{
    analysis::{
        derived::get_decay,
        metrics::{get_coupling, get_owners, get_summary},
    },
    cli::{Cli, Commands},
    repo::parse_commit_info,
};

use clap::Parser;

fn main() {
    let command = Cli::parse();

    let commits = parse_commit_info(&command.repo).unwrap();

    match command.command_type {
        Commands::Summary => println!("{:?}", get_summary(&commits)),
        Commands::Decay => println!("{:?}", get_decay(&commits)),
        Commands::Coupling => println!(
            "{:?}",
            get_coupling(&commits)
                .iter()
                .filter(|p| *p.1 > commits.len() / 10)
        ),
        Commands::Ownership => println!("{:?}", get_owners(&commits)),
    };
}
