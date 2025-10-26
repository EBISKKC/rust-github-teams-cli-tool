use crate::stats::{ContributorStats, FileStats, TimeStats};
use colored::*;
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Color, Table};
use git2::Repository;

pub fn display_contributors(stats: &[ContributorStats]) {
    println!("\n{}\n", "📊 Contributor Statistics".bold().cyan());

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        Cell::new("Contributor").fg(Color::Cyan),
        Cell::new("Commits").fg(Color::Green),
        Cell::new("Additions").fg(Color::Green),
        Cell::new("Deletions").fg(Color::Red),
        Cell::new("Files").fg(Color::Yellow),
        Cell::new("Net").fg(Color::Magenta),
    ]);

    for stat in stats {
        let net = stat.additions as i64 - stat.deletions as i64;
        let net_str = if net >= 0 {
            format!("+{}", net).green().to_string()
        } else {
            format!("{}", net).red().to_string()
        };

        table.add_row(vec![
            Cell::new(&format!("{} <{}>", stat.name, stat.email)),
            Cell::new(stat.commits.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(stat.additions.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(stat.deletions.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(stat.files_changed.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(net_str).set_alignment(CellAlignment::Right),
        ]);
    }

    println!("{}", table);
}

pub fn display_time_analysis(time_stats: &TimeStats) {
    println!("\n{}\n", "⏰ Time-based Commit Analysis".bold().cyan());

    // Hour distribution
    println!("{}", "Commits by Hour:".bold());
    let max_hour_commits = time_stats.hour_distribution.values().max().unwrap_or(&1);
    for hour in 0..24 {
        let count = time_stats.hour_distribution.get(&hour).unwrap_or(&0);
        let bar_length = ((*count as f64 / *max_hour_commits as f64) * 50.0) as usize;
        let bar = "█".repeat(bar_length);
        println!("{:02}:00 │ {} {}", hour, bar.green(), format!("({})", count).dimmed());
    }

    println!();

    // Day distribution
    println!("{}", "Commits by Day of Week:".bold());
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let max_day_commits = time_stats.day_distribution.values().max().unwrap_or(&1);
    for (idx, day) in days.iter().enumerate() {
        let count = time_stats.day_distribution.get(&(idx as u32)).unwrap_or(&0);
        let bar_length = ((*count as f64 / *max_day_commits as f64) * 50.0) as usize;
        let bar = "█".repeat(bar_length);
        println!("{} │ {} {}", day, bar.cyan(), format!("({})", count).dimmed());
    }
    println!();
}

pub fn display_file_changes(files: &[FileStats], top: usize) {
    println!("\n{}\n", "📁 Most Changed Files".bold().cyan());

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        Cell::new("Rank").fg(Color::Cyan),
        Cell::new("File Path").fg(Color::Cyan),
        Cell::new("Changes").fg(Color::Green),
        Cell::new("Contributors").fg(Color::Yellow),
    ]);

    for (idx, file) in files.iter().take(top).enumerate() {
        table.add_row(vec![
            Cell::new((idx + 1).to_string()).set_alignment(CellAlignment::Right),
            Cell::new(&file.path),
            Cell::new(file.changes.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(file.contributors.len().to_string()).set_alignment(CellAlignment::Right),
        ]);
    }

    println!("{}", table);
}

pub fn display_summary(
    repo: &Repository,
    stats: &[ContributorStats],
    days: i64,
) -> Result<(), git2::Error> {
    println!("\n{}\n", "📈 Team Summary".bold().cyan());

    let total_commits: usize = stats.iter().map(|s| s.commits).sum();
    let total_additions: usize = stats.iter().map(|s| s.additions).sum();
    let total_deletions: usize = stats.iter().map(|s| s.deletions).sum();
    let total_contributors = stats.len();

    let period = if days == 0 {
        "All Time".to_string()
    } else {
        format!("Last {} days", days)
    };

    println!("{}: {}", "Period".bold(), period.yellow());
    println!("{}: {}", "Total Contributors".bold(), total_contributors.to_string().green());
    println!("{}: {}", "Total Commits".bold(), total_commits.to_string().cyan());
    println!("{}: {}", "Lines Added".bold(), format!("+{}", total_additions).green());
    println!("{}: {}", "Lines Deleted".bold(), format!("-{}", total_deletions).red());
    println!(
        "{}: {}",
        "Net Change".bold(),
        if total_additions >= total_deletions {
            format!("+{}", total_additions - total_deletions).green()
        } else {
            format!("-{}", total_deletions - total_additions).red()
        }
    );

    // Repository info - show path instead of branch
    if let Some(path) = repo.path().parent() {
        println!(
            "{}: {}",
            "Repository".bold(),
            path.display().to_string().yellow()
        );
    }

    println!();
    Ok(())
}
