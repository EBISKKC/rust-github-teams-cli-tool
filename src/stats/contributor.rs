use super::{get_commit_stats, is_within_days};
use git2::Repository;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct ContributorStats {
    pub name: String,
    pub email: String,
    pub commits: usize,
    pub additions: usize,
    pub deletions: usize,
    pub files_changed: usize,
}

pub fn analyze_contributors(
    repo: &Repository,
    days: i64,
) -> Result<Vec<ContributorStats>, git2::Error> {
    let mut stats_map: HashMap<String, ContributorStats> = HashMap::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        if !is_within_days(commit.time(), days) {
            continue;
        }

        let author = commit.author();
        let email = author.email().unwrap_or("unknown").to_string();
        let name = author.name().unwrap_or("unknown").to_string();

        let key = format!("{}|{}", name, email);
        let entry = stats_map.entry(key).or_insert_with(|| ContributorStats {
            name: name.clone(),
            email: email.clone(),
            ..Default::default()
        });

        entry.commits += 1;

        // Analyze diff for additions/deletions
        if let Ok((additions, deletions, files)) = get_commit_stats(&commit, repo) {
            entry.additions += additions;
            entry.deletions += deletions;
            entry.files_changed += files;
        }
    }

    let mut stats: Vec<ContributorStats> = stats_map.into_values().collect();
    stats.sort_by(|a, b| b.commits.cmp(&a.commits));
    Ok(stats)
}
