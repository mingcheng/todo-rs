/*
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co,.Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: main.rs
 * Author: mingcheng (mingcheng@apache.org)
 * File Created: 2025-02-19 14:51:13
 *
 * Modified By: mingcheng (mingcheng@apache.org)
 * Last Modified: 2025-03-04 19:33:29
 */

use anyhow::Result;
use clap::{Parser, Subcommand};
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};
use dialoguer::Confirm;
use log::{debug, error};
use std::fmt::Debug;
use todo::task::Task;
use todo::todo::Todo;
use tracing::Level;

/// A fictional simple command line tool for generating todo list
#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo list manager", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Specify the file path to store the todo list
    #[arg(long, short, env = "TODO_FILE_PATH", required = false)]
    file_path: String,

    /// Verbose mode
    #[arg(long, short, default_value_t = false, required = false)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new task
    #[command(alias = "create")]
    #[command(arg_required_else_help = true)]
    Add { description: String },
    /// Delete a task
    #[command(alias = "remove")]
    #[command(arg_required_else_help = true)]
    Delete { index: usize },
    /// Mark a task as completed or not
    #[command(alias = "finish")]
    #[command(arg_required_else_help = true)]
    Complete { index: usize },
    /// List all tasks
    #[command(alias = "all")]
    List,
    /// Export tasks to a file
    #[command(alias = "dump")]
    Export,
    /// Remove all tasks
    #[command(alias = "truncate")]
    Clear,
}

// Group constants together
const BUILT_ON: &str = build_time::build_time_local!("%Y-%m-%d %H:%M:%S %:z");
const COMPLETED: &str = "✓ Completed";
const PENDING: &str = "⧖ Pending";
const TABLE_HEADERS: [&str; 3] = ["INDEX", "TOPIC", "STATUS"];

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(if cli.verbose {
            Level::TRACE
        } else {
            Level::WARN
        })
        .without_time()
        .with_target(false)
        .init();

    debug!("This binary was compiled on {}", BUILT_ON);
    let mut todo = match Todo::new(&cli.file_path) {
        Ok(todo) => todo,
        Err(e) => {
            error!("Failed to initialize todo list: {}", e);
            return Err(anyhow::anyhow!("Failed to initialize todo list: {}", e));
        }
    };

    match cli.command {
        Commands::Add { description } => {
            todo.add(Task {
                description: description.trim().to_string(),
                completed: false,
            })
            .map_err(|e| anyhow::anyhow!("Failed to add task: {}", e))?;
            println!("Task added successfully!");
        }
        Commands::Delete { index } => {
            if Confirm::new()
                .with_prompt(format!("Are you sure you want to delete task {}?", index))
                .default(false)
                .interact()?
            {
                todo.delete(index)
                    .map_err(|e| anyhow::anyhow!("Failed to delete task: {}", e))?;
                println!("Task deleted successfully!");
            }
        }
        Commands::Complete { index } => {
            todo.complete(index)
                .map_err(|e| anyhow::anyhow!("Failed to complete task: {}", e))?;
            println!("Task marked as completed!");
        }
        Commands::List => {
            let tasks = todo
                .list()
                .map_err(|e| anyhow::anyhow!("Failed to list tasks: {}", e))?;

            if tasks.is_empty() {
                println!("No tasks found. Add one with: todo add <description>");
                return Ok(());
            }

            let mut table = Table::new();
            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(TABLE_HEADERS);

            for (i, task) in tasks.iter().enumerate() {
                let status_cell = if task.completed {
                    Cell::new(COMPLETED).fg(Color::Green)
                } else {
                    Cell::new(PENDING).fg(Color::Yellow)
                };

                table.add_row(vec![
                    Cell::new(i.to_string()).set_alignment(CellAlignment::Center),
                    Cell::new(&task.description),
                    status_cell.set_alignment(CellAlignment::Center),
                ]);
            }

            println!("{table}");
        }
        Commands::Export => {
            let tasks = todo
                .list()
                .map_err(|e| anyhow::anyhow!("Failed to export tasks: {}", e))?;
            println!("{}", serde_json::to_string_pretty(&tasks)?);
        }
        Commands::Clear => {
            if Confirm::new()
                .with_prompt("Are you sure you want to clear all tasks?")
                .default(false)
                .interact()?
            {
                todo.clear()
                    .map_err(|e| anyhow::anyhow!("Failed to clear tasks: {}", e))?;
                println!("All tasks cleared successfully!");
            }
        }
    }

    Ok(())
}
