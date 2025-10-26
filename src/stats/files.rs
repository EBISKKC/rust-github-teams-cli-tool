use super::is_within_days;
use git2::{DiffOptions, Repository};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct FileStats {
    pub path: String,
    pub changes: usize,
    pub contributors: Vec<String>,
}

pub fn analyze_file_changes(repo: &Repository, days: i64) -> Result<Vec<FileStats>, git2::Error> {
    let mut file_map: HashMap<String, FileStats> = HashMap::new();
    let mut revwalk = repo.revwalk()?;
    // Analyze all branches, not just HEAD
    revwalk.push_glob("refs/*")?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        if !is_within_days(commit.time(), days) {
            continue;
        }

        let author_name = commit.author().name().unwrap_or("unknown").to_string();

        let a = if commit.parent_count() > 0 {
            let parent = commit.parent(0)?;
            Some(parent.tree()?)
        } else {
            None
        };
        let b = commit.tree()?;

        let mut diff_opts = DiffOptions::new();
        let diff = repo.diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diff_opts))?;

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    let path_str = path.to_string_lossy().to_string();
                    let entry = file_map.entry(path_str.clone()).or_insert_with(|| FileStats {
                        path: path_str,
                        changes: 0,
                        contributors: Vec::new(),
                    });
                    entry.changes += 1;
                    if !entry.contributors.contains(&author_name) {
                        entry.contributors.push(author_name.clone());
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;
    }

    let mut files: Vec<FileStats> = file_map.into_values().collect();
    files.sort_by(|a, b| b.changes.cmp(&a.changes));
    Ok(files)
}
