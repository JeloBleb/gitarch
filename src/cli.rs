use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, default_value = ".")]
    pub repo: PathBuf,
    #[command(subcommand)]
    pub command_type: Commands,
    #[arg(long)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Summary,
    Coupling {
        #[arg(long, default_value_t = 20)]
        max_changeset_size: usize,
    },
    Communication,
    Ownership,
    Decay,
    Churn,
}
