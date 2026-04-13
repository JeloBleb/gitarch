use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Format {
    Table,
    Json,
    Csv,
}

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
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Parser, Clone)]
pub struct OutputConfig {
    #[arg(long, default_value = "table", value_enum)]
    pub format: Format,
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
