mod cli;
mod display;
mod stats;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use display::{display_contributors, display_file_changes, display_summary, display_time_analysis};
use git2::Repository;
use stats::{analyze_contributors, analyze_file_changes, analyze_time_distribution};

fn main() {
    let cli = Cli::parse();

    let repo = match Repository::open(&cli.repo) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            eprintln!("Make sure you're in a Git repository or specify the path with --repo");
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Commands::Contributors { days } => match analyze_contributors(&repo, days) {
            Ok(stats) => {
                display_contributors(&stats);
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::TimeAnalysis { days } => match analyze_time_distribution(&repo, days) {
            Ok(time_stats) => {
                display_time_analysis(&time_stats);
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::Files { top, days } => match analyze_file_changes(&repo, days) {
            Ok(files) => {
                display_file_changes(&files, top);
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::Summary { days } => match analyze_contributors(&repo, days) {
            Ok(stats) => display_summary(&repo, &stats, days),
            Err(e) => Err(e),
        },
        Commands::Report { period } => {
            let days = if period == "weekly" { 7 } else { 30 };

            println!("\n{}", format!("=== {} Report ===", period.to_uppercase()).bold().cyan());

            match analyze_contributors(&repo, days) {
                Ok(stats) => {
                    if let Err(e) = display_summary(&repo, &stats, days) {
                        eprintln!("{} {}", "Error:".red().bold(), e);
                        std::process::exit(1);
                    }
                    display_contributors(&stats);
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            }

            match analyze_time_distribution(&repo, days) {
                Ok(time_stats) => display_time_analysis(&time_stats),
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            }

            match analyze_file_changes(&repo, days) {
                Ok(files) => {
                    display_file_changes(&files, 10);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
