pub mod contributor;
pub mod files;
pub mod time;

pub use contributor::{analyze_contributors, ContributorStats};
pub use files::{analyze_file_changes, FileStats};
pub use time::{analyze_time_distribution, TimeStats};

use chrono::{DateTime, Duration, Utc};
use git2::{Commit, DiffOptions, Repository, Time};

pub fn time_to_datetime(time: Time) -> DateTime<Utc> {
    use chrono::TimeZone;
    Utc.timestamp_opt(time.seconds(), 0).unwrap()
}

pub fn is_within_days(commit_time: Time, days: i64) -> bool {
    if days == 0 {
        return true;
    }
    let now = Utc::now();
    let commit_dt = time_to_datetime(commit_time);
    let cutoff = now - Duration::days(days);
    commit_dt >= cutoff
}

pub fn get_commit_stats(
    commit: &Commit,
    repo: &Repository,
) -> Result<(usize, usize, usize), git2::Error> {
    let a = if commit.parent_count() > 0 {
        let parent = commit.parent(0)?;
        Some(parent.tree()?)
    } else {
        None
    };
    let b = commit.tree()?;

    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diff_opts))?;

    let stats = diff.stats()?;
    Ok((stats.insertions(), stats.deletions(), stats.files_changed()))
}
