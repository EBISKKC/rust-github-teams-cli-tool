use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "taskr")]
#[command(about = "A simple and fast task management CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: String,
        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// List all tasks
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
        /// Show only completed tasks
        #[arg(short, long)]
        completed: bool,
        /// Show only pending tasks
        #[arg(short, long)]
        pending: bool,
    },
    /// Mark a task as complete
    Complete {
        /// Task ID
        id: usize,
    },
    /// Delete a task
    Delete {
        /// Task ID
        id: usize,
    },
    /// Clear all completed tasks
    Clear,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    fn new() -> Self {
        TaskList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String, tags: Vec<String>) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
            tags,
            created_at: Utc::now(),
            completed_at: None,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn complete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            task.completed_at = Some(Utc::now());
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    fn delete_task(&mut self, id: usize) -> Result<(), String> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() < initial_len {
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    fn clear_completed(&mut self) -> usize {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| !t.completed);
        initial_len - self.tasks.len()
    }

    fn filter_tasks(&self, tag: Option<&str>, completed: bool, pending: bool) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|t| {
                let tag_match = tag.map_or(true, |tag| t.tags.contains(&tag.to_string()));
                let status_match = if completed {
                    t.completed
                } else if pending {
                    !t.completed
                } else {
                    true
                };
                tag_match && status_match
            })
            .cloned()
            .collect()
    }
}

fn get_data_file() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    home_dir.join(".taskr.json")
}

fn load_tasks() -> TaskList {
    let path = get_data_file();
    if path.exists() {
        let content = fs::read_to_string(&path).expect("Failed to read task file");
        serde_json::from_str(&content).unwrap_or_else(|_| TaskList::new())
    } else {
        TaskList::new()
    }
}

fn save_tasks(task_list: &TaskList) {
    let path = get_data_file();
    let content = serde_json::to_string_pretty(task_list).expect("Failed to serialize tasks");
    fs::write(&path, content).expect("Failed to write task file");
}

fn display_task(task: &Task) {
    let status = if task.completed {
        "✓".green()
    } else {
        "○".yellow()
    };

    let id = format!("[{}]", task.id).cyan();
    let description = if task.completed {
        task.description.dimmed().strikethrough()
    } else {
        task.description.normal()
    };

    let tags = if !task.tags.is_empty() {
        format!(" {}", task.tags.iter()
            .map(|t| format!("#{}", t).magenta().to_string())
            .collect::<Vec<_>>()
            .join(" "))
    } else {
        String::new()
    };

    println!("{} {} {}{}", status, id, description, tags);
}

fn main() {
    let cli = Cli::parse();
    let mut task_list = load_tasks();

    match cli.command {
        Commands::Add { description, tags } => {
            let tag_list = tags
                .map(|t| {
                    t.split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                })
                .unwrap_or_default();

            task_list.add_task(description.clone(), tag_list);
            save_tasks(&task_list);
            println!("{} Task added: {}", "✓".green(), description);
        }
        Commands::List { tag, completed, pending } => {
            let tasks = task_list.filter_tasks(
                tag.as_deref(),
                completed,
                pending,
            );

            if tasks.is_empty() {
                println!("{}", "No tasks found".dimmed());
            } else {
                println!("\n{}\n", "Your Tasks:".bold().underline());
                for task in tasks {
                    display_task(&task);
                }
                println!();
            }
        }
        Commands::Complete { id } => {
            match task_list.complete_task(id) {
                Ok(_) => {
                    save_tasks(&task_list);
                    println!("{} Task {} marked as complete", "✓".green(), id);
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Delete { id } => {
            match task_list.delete_task(id) {
                Ok(_) => {
                    save_tasks(&task_list);
                    println!("{} Task {} deleted", "✓".green(), id);
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Clear => {
            let count = task_list.clear_completed();
            save_tasks(&task_list);
            println!("{} Cleared {} completed task(s)", "✓".green(), count);
        }
    }
}
