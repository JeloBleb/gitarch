use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value = ".")]
    pub repo: PathBuf,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Knowledge,
    Coupling,
    Diffs,
}
