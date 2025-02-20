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
 * Last Modified: 2025-02-20 10:33:37
 */

use clap::{Parser, Subcommand};
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};
use log::{debug, error, info, trace};
use std::fmt::Debug;
use todo::task::Task;
use todo::todo::{Todo, TodoResult};
use tracing::Level;

/// A fictional simple command line tool for generating todo list
#[derive(Debug, Parser)]
#[command(name = "todo")]
// #[command(about = "A fictional versioning CLI", long_about = None)]
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

fn main() -> TodoResult<()> {
    let cli = Cli::parse();

    trace!(
        "Initializing the logger, with verbose mode: {}",
        if cli.verbose { "on" } else { "off" }
    );
    tracing_subscriber::fmt()
        .with_max_level(if cli.verbose {
            Level::TRACE
        } else {
            Level::WARN
        })
        .without_time()
        .with_target(false)
        .init();

    debug!(
        "specified the file path {:?} to  get and store the todo list",
        cli.file_path
    );
    trace!("Creating a new todo instance with path: {}", cli.file_path);
    let mut todo = Todo::new(cli.file_path.as_str()).unwrap();

    match cli.command {
        Commands::Add { description } => {
            info!("Adding a new task: {:?}", description);
            todo.add(Task {
                description,
                completed: false,
            });
        }
        Commands::Delete { index } => {
            info!("Deleting task at index: {}", index);
            let _ = todo.delete(index);
        }
        Commands::Complete { index } => {
            info!("Completing task at index: {}", index);
            match todo.complete(index) {
                Ok(_) => {
                    info!("Task completed successfully");
                }
                Err(e) => {
                    error!("Failed to complete task: {}", e);
                }
            }
        }
        Commands::List => {
            info!("Listing all tasks");
            let tasks = todo.list()?;

            if tasks.is_empty() {
                println!(
                    "The tasks list is empty, you can add a new task by `todo add <description>`"
                );
                return Ok(());
            }

            const COMPLETED: &str = "Completed";
            const PENDING: &str = "Pending";

            let mut table = Table::new();
            table
                // .load_preset(UTF8_FULL)
                // .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["INDEX", "TOPIC", "STATUS"]);

            for (i, task) in tasks.iter().enumerate() {
                let status_cell = if task.completed {
                    Cell::new(COMPLETED).fg(Color::Green)
                } else {
                    Cell::new(PENDING).fg(Color::Red)
                };

                table.add_row(vec![
                    Cell::new(i.to_string()).set_alignment(CellAlignment::Center),
                    Cell::new(task.description.clone()),
                    status_cell,
                ]);
            }

            println!("{table}");
        }
        Commands::Export => {
            trace!("Exporting tasks to a stdout");
            let tasks = todo.list()?;
            let json = serde_json::to_string_pretty(&tasks)?;
            println!("{}", json);
        }
        Commands::Clear => {
            trace!("Clearing all tasks");
            todo.clear();
        }
    };

    Ok(())
}
