use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, default_value = ".")]
    pub repo: PathBuf,
    #[command(subcommand)]
    pub command_type: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Summary,
    Coupling,
    Ownership,
    Decay,
}
