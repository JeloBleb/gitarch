use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, default_value = ".")]
    pub repo: PathBuf,
    #[command(subcommand)]
    pub command_type: Commands,
    #[command(flatten)]
    pub config: OutputConfig,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Summary,
    Coupling {
        #[arg(long, default_value_t = 20)]
        max_changeset_size: usize,
        #[arg(long, default_value_t = 15)]
        min_coupling_percentage: usize,
        #[arg(long, default_value_t = 5)]
        min_revision_count: usize,
    },
    Communication,
    Ownership,
    Decay {
        #[arg(long, default_value_t = 180)]
        decay_threshold: i64,
    },
    Churn,
    OwnerSummary,
}

#[derive(Debug, Parser, Clone)]
pub struct OutputConfig {
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub top: Option<usize>,
    #[arg(long)]
    pub since: Option<chrono::NaiveDate>,
    #[arg(long)]
    pub until: Option<chrono::NaiveDate>,
    #[arg(long)]
    pub path: Option<String>,
    #[arg(long)]
    pub include: Vec<String>,
    #[arg(long)]
    pub exclude: Vec<String>,
}
