# Taskr

A simple and fast task management CLI built with Rust.

## Features

- Add tasks with optional tags
- List all tasks with colored output
- Filter tasks by tag or completion status
- Mark tasks as complete
- Delete tasks
- Clear all completed tasks
- Persistent storage in JSON format

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/taskr`.

Optionally, install it globally:

```bash
cargo install --path .
```

## Usage

### Add a task

```bash
taskr add "Learn Rust"
taskr add "Build CLI app" --tags rust,programming
```

### List tasks

```bash
# List all tasks
taskr list

# Filter by tag
taskr list --tag programming

# Show only pending tasks
taskr list --pending

# Show only completed tasks
taskr list --completed
```

### Complete a task

```bash
taskr complete 1
```

### Delete a task

```bash
taskr delete 2
```

### Clear completed tasks

```bash
taskr clear
```

## Data Storage

Tasks are stored in `~/.taskr.json` as a JSON file. You can manually edit this file if needed.

## Example Session

```bash
$ taskr add "Learn Rust" --tags programming,study
✓ Task added: Learn Rust

$ taskr add "Build CLI app" --tags rust,programming
✓ Task added: Build CLI app

$ taskr list

Your Tasks:

○ [1] Learn Rust #programming #study
○ [2] Build CLI app #rust #programming

$ taskr complete 1
✓ Task 1 marked as complete

$ taskr list

Your Tasks:

✓ [1] Learn Rust #programming #study
○ [2] Build CLI app #rust #programming
```

## Dependencies

- `clap`: Command-line argument parsing
- `serde`: Serialization/deserialization
- `serde_json`: JSON support
- `colored`: Colored terminal output
- `chrono`: Date and time handling
- `dirs`: Home directory detection
