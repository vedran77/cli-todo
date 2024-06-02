# CLI To-Do List

A simple Command-Line Interface (CLI) To-Do list application written in Rust. This application allows you to manage your tasks directly from the terminal.

## Features

- Add a new task
- List all tasks
- Mark a task as complete
- Remove a task
- Edit a task description

## Installation

1. **Install Rust**:
   Make sure you have Rust installed. If not, follow the instructions [here](https://www.rust-lang.org/tools/install).

2. **Clone the repository**:

   ```sh
   git clone https://github.com/vedran77/cli-todo.git
   cd cli_todo
   ```

3. **Build the project**:
   ```sh
   cargo build --release
   ```

## Usage

Navigate to the project directory and use the following commands:

### Add a Task

Add a new task to the list.

```sh
cargo run -- add "Your task description"
```

### List All Tasks

List all tasks with their completion status.

```sh
cargo run -- list
```

### Complete a Task

Mark a task as complete by specifying its index.

```sh
cargo run -- complete [index]
```

Example:

```sh
cargo run -- complete 0
```

### Remove a Task

Remove a task from the list by specifying its index.

```sh
cargo run -- remove [index]
```

Example:

```sh
cargo run -- remove 0
```

### Edit a Task

Edit an existing task's description by specifying its index and the new description.

```sh
cargo run -- edit [index] "New task description"
```

Example:

```sh
cargo run -- edit 0 "Learn Rust"
```
