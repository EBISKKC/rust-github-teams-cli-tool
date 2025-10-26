use super::{is_within_days, time_to_datetime};
use chrono::{Datelike, Local, Timelike};
use git2::Repository;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TimeStats {
    pub hour_distribution: HashMap<u32, usize>,
    pub day_distribution: HashMap<u32, usize>,
}

pub fn analyze_time_distribution(repo: &Repository, days: i64) -> Result<TimeStats, git2::Error> {
    let mut time_stats = TimeStats::default();
    let mut revwalk = repo.revwalk()?;
    // Analyze all branches, not just HEAD
    revwalk.push_glob("refs/*")?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        if !is_within_days(commit.time(), days) {
            continue;
        }

        let dt = time_to_datetime(commit.time()).with_timezone(&Local);
        let hour = dt.hour();
        let day = dt.weekday().num_days_from_monday();

        *time_stats.hour_distribution.entry(hour).or_insert(0) += 1;
        *time_stats.day_distribution.entry(day).or_insert(0) += 1;
    }

    Ok(time_stats)
}
