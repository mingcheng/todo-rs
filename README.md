# Todo CLI Application

[![Rust](https://github.com/mingcheng/todo-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mingcheng/todo-rs/actions/workflows/rust.yml)

![screenshot](./asserts/screenshot.png)

A simple command-line todo list manager written in Rust. This is my first project in Rust, so it may not be idiomatic. And I'm not a professional Rust developer, so there are many things to improve.

## Features

- Add new tasks
- Delete tasks by index
- Mark tasks as complete/incomplete
- List all tasks in a formatted table
- Export tasks to JSON
- Clear all tasks
- Persistent storage using JSON file

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Add a new task
todo --file-path todo.json add "Buy groceries"

# List all tasks
todo --file-path todo.json list

# Complete a task (toggle)
todo --file-path todo.json complete 0

# Delete a task
todo --file-path todo.json delete 0

# Export tasks to JSON
todo --file-path todo.json export

# Clear all tasks
todo --file-path todo.json clear

# Enable verbose logging
todo --file-path todo.json --verbose list
```

## Environment Variables

- `TODO_FILE_PATH`: Set the default path for the todo list file


## Build with Docker

Our Dockerfile is based on the `rust:alpine` image. And for more details, you can refer to the [Dockerfile](./Dockerfile). Build the image is very simple, just run the following command:

```bash
docker compose build
```

Run the container is also very simple, just run the following command:

```bash
docker compose run todo
```

The container will run the `todo` command with the default arguments.

## License

MIT License
