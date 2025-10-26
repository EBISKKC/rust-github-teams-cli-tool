mod cli;
mod config;
mod display;
mod stats;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::Config;
use display::{display_contributors, display_file_changes, display_summary, display_time_analysis};
use git2::Repository;
use stats::{analyze_contributors, analyze_file_changes, analyze_time_distribution};

/// Fetch all remotes to ensure we have the latest data
fn fetch_all_remotes(repo: &Repository) {
    print!("{}", "🔄 Fetching latest data from remotes...".cyan());
    std::io::Write::flush(&mut std::io::stdout()).ok();

    let remotes = match repo.remotes() {
        Ok(remotes) => remotes,
        Err(_) => {
            println!(" {}", "No remotes found, skipping fetch".yellow());
            return;
        }
    };

    let mut fetch_count = 0;
    let mut has_remotes = false;

    for remote_name in remotes.iter().flatten() {
        has_remotes = true;
        if let Ok(mut remote) = repo.find_remote(remote_name) {
            // Fetch all refs for this remote
            let refspec = format!("+refs/heads/*:refs/remotes/{}/*", remote_name);
            match remote.fetch(&[refspec.as_str()], None, None) {
                Ok(_) => {
                    fetch_count += 1;
                }
                Err(_) => {
                    // Silently continue on fetch error (might be network issue)
                    continue;
                }
            }
        }
    }

    if fetch_count > 0 {
        println!(" {}", format!("✓ Fetched {} remote(s)", fetch_count).green());
    } else if has_remotes {
        println!(" {}", "Already up to date".dimmed());
    } else {
        println!(" {}", "No remotes configured".dimmed());
    }
}

fn main() {
    let cli = Cli::parse();
    let config = Config::from_env();

    // Use config to determine repo path
    let repo_path = config.get_repo_path(&cli.repo);

    let repo = match Repository::open(&repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            eprintln!("Make sure you're in a Git repository or specify the path with --repo");
            std::process::exit(1);
        }
    };

    // Automatically fetch all remotes before analysis
    fetch_all_remotes(&repo);

    let result = match cli.command {
        Commands::Contributors { days } => {
            let effective_days = config.get_days(days);
            match analyze_contributors(&repo, effective_days) {
                Ok(stats) => {
                    let filtered_stats = config.filter_by_teams(stats, |s| &s.email);
                    display_contributors(&filtered_stats);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::TimeAnalysis { days } => {
            let effective_days = config.get_days(days);
            match analyze_time_distribution(&repo, effective_days) {
                Ok(time_stats) => {
                    display_time_analysis(&time_stats);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::Files { top, days } => {
            let effective_days = config.get_days(days);
            match analyze_file_changes(&repo, effective_days) {
                Ok(files) => {
                    display_file_changes(&files, top);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::Summary { days } => {
            let effective_days = config.get_days(days);
            match analyze_contributors(&repo, effective_days) {
                Ok(stats) => {
                    let filtered_stats = config.filter_by_teams(stats, |s| &s.email);
                    display_summary(&repo, &filtered_stats, effective_days)
                }
                Err(e) => Err(e),
            }
        }
        Commands::Report { period } => {
            let days = if period == "weekly" { 7 } else { 30 };
            let effective_days = config.get_days(days);

            println!("\n{}", format!("=== {} Report ===", period.to_uppercase()).bold().cyan());

            match analyze_contributors(&repo, effective_days) {
                Ok(stats) => {
                    let filtered_stats = config.filter_by_teams(stats, |s| &s.email);
                    if let Err(e) = display_summary(&repo, &filtered_stats, effective_days) {
                        eprintln!("{} {}", "Error:".red().bold(), e);
                        std::process::exit(1);
                    }
                    display_contributors(&filtered_stats);
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            }

            match analyze_time_distribution(&repo, effective_days) {
                Ok(time_stats) => display_time_analysis(&time_stats),
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            }

            match analyze_file_changes(&repo, effective_days) {
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
