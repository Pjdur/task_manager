# Task Manager

A small, fast CLI task manager written in Rust.  
Tasks are stored in `~/.task_manager/tasks.json` so you can run the tool from anywhere.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Pjdur/task_manager
   cd task_manager
   ```

2. Install the binary to your PATH:
   ```bash
   cargo install --path .
   ```

## Usage

### `add <text>`
Add a new task.

Example:
```bash
task_manager add "buy milk"
```

### `list`
Show all tasks.

Example:
```bash
task_manager list
```

### `done <id>`
Mark a task as completed.

Example:
```bash
task_manager done 3
```

### `undone <id>`
Mark a task as not completed.

Example:
```bash
task_manager undone 3
```

### `delete <id>`
Remove a task permanently.

Example:
```bash
task_manager delete 2
```

## Storage

Tasks are saved in:
```
~/.task_manager/tasks.json
```

This keeps your home directory clean and lets you use the tool from any folder.
