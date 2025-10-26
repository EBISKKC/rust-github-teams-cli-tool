use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub repository: Option<PathBuf>,
    pub teams: Vec<String>,
    pub default_days: Option<i64>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            repository: None,
            teams: Vec::new(),
            default_days: None,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        // Try to load .env file if it exists (doesn't fail if not found)
        let _ = dotenvy::dotenv();

        let repository = env::var("GIT_REPO_PATH").ok().map(PathBuf::from);

        let teams = env::var("GIT_TEAMS")
            .ok()
            .map(|t| t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();

        let default_days = env::var("DEFAULT_DAYS").ok().and_then(|d| d.parse().ok());

        Config {
            repository,
            teams,
            default_days,
        }
    }

    /// Get the repository path, falling back to CLI argument or current directory
    pub fn get_repo_path(&self, cli_repo: &PathBuf) -> PathBuf {
        // If CLI repo is the default ".", prefer .env setting
        if cli_repo.as_os_str() == "." {
            self.repository.clone().unwrap_or_else(|| PathBuf::from("."))
        } else {
            cli_repo.clone()
        }
    }

    /// Filter contributors by configured teams
    pub fn filter_by_teams<T>(&self, items: Vec<T>, get_email: impl Fn(&T) -> &str) -> Vec<T> {
        if self.teams.is_empty() {
            return items;
        }

        items
            .into_iter()
            .filter(|item| {
                let email = get_email(item);
                self.teams.iter().any(|team| email.contains(team))
            })
            .collect()
    }

    /// Get default days or use the provided value
    pub fn get_days(&self, cli_days: i64) -> i64 {
        if cli_days != 0 && cli_days != 30 {
            // If user explicitly set days via CLI, use that
            cli_days
        } else {
            // Otherwise, use config default or fall back to CLI value
            self.default_days.unwrap_or(cli_days)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.repository.is_none());
        assert!(config.teams.is_empty());
        assert!(config.default_days.is_none());
    }

    #[test]
    fn test_get_repo_path_default() {
        let config = Config::default();
        let path = config.get_repo_path(None);
        assert_eq!(path, PathBuf::from("."));
    }

    #[test]
    fn test_filter_by_teams_empty() {
        let config = Config::default();
        let items = vec!["alice@example.com", "bob@example.com"];
        let filtered = config.filter_by_teams(items.clone(), |s| s);
        assert_eq!(filtered, items);
    }

    #[test]
    fn test_filter_by_teams() {
        let config = Config {
            teams: vec!["example.com".to_string()],
            ..Default::default()
        };
        let items = vec!["alice@example.com", "bob@other.com"];
        let filtered = config.filter_by_teams(items, |s| s);
        assert_eq!(filtered, vec!["alice@example.com"]);
    }
}
