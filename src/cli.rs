use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-team-stats")]
#[command(about = "Team Git statistics and analysis tool", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Path to the Git repository (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    pub repo: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show contributor statistics
    Contributors {
        /// Number of days to analyze (0 = all time)
        #[arg(short, long, default_value = "0")]
        days: i64,
    },
    /// Show time-based commit analysis
    TimeAnalysis {
        /// Number of days to analyze (0 = all time)
        #[arg(short, long, default_value = "30")]
        days: i64,
    },
    /// Show file change frequency ranking
    Files {
        /// Number of top files to show
        #[arg(short, long, default_value = "20")]
        top: usize,
        /// Number of days to analyze (0 = all time)
        #[arg(short, long, default_value = "0")]
        days: i64,
    },
    /// Generate a comprehensive report
    Report {
        /// Report period: weekly or monthly
        #[arg(short, long, default_value = "weekly")]
        period: String,
    },
    /// Show overall team summary
    Summary {
        /// Number of days to analyze (0 = all time)
        #[arg(short, long, default_value = "30")]
        days: i64,
    },
}

impl Commands {
    pub fn days(&self) -> i64 {
        match self {
            Commands::Contributors { days } => *days,
            Commands::TimeAnalysis { days } => *days,
            Commands::Files { days, .. } => *days,
            Commands::Summary { days } => *days,
            Commands::Report { period } => {
                if period == "weekly" {
                    7
                } else {
                    30
                }
            }
        }
    }
}
