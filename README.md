# 🦀 Rust Todo App

A simple command-line todo application written in Rust. This project demonstrates Rust fundamentals such as structs, ownership and borrowing, traits, vectors, pattern matching, error handling, and user input.

## Features

* Add a new todo
* List all todos
* Mark a todo as complete
* Delete a todo
* View todo statistics
* Interactive command-line interface

## Technologies

* Rust
* Standard Library (`std`)

## Running the Project

1. Clone the repository:
  ```
git clone https://github.com/christinathucanh/todo-app.git
```
2. Navigate to the project directory.
3. Build and run the application:

```bash
cargo run
```

## Available Commands

| Command       | Description              |
| ------------- | ------------------------ |
| `add <title>` | Add a new todo           |
| `list`        | Display all todos        |
| `done <id>`   | Mark a todo as complete  |
| `delete <id>` | Delete a todo            |
| `stats`       | Show summary statistics  |
| `help`        | Display the command list |
| `quit`        | Exit the application     |

## Example

```text
todo> add Finish Rust project
➕ Added: "Finish Rust project" (id=1)

todo> list
[1] ⬜ — Finish Rust project

todo> done 1
✅ Marked done: "Finish Rust project"

todo> stats
📊 Stats: 1 total | 1 done | 0 pending
```

## Learning Objectives

This project was built to practice:

* Structs and methods
* Ownership and borrowing
* Mutable and immutable references
* Vectors (`Vec<T>`)
* Pattern matching with `match`
* `Option` and `Result`
* Traits (`Display`)
* User input and output
* Basic command-line application design
