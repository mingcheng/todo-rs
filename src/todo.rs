/*
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co,.Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: todo.rs
 * Author: mingcheng (mingcheng@apache.org)
 * File Created: 2025-02-19 14:51:37
 *
 * Modified By: mingcheng (mingcheng@apache.org)
 * Last Modified: 2025-03-04 19:36:38
 */

use crate::task::Task;
use log::{debug, error, trace};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};

/// A struct representing a Todo list.
/// It contains a vector of tasks and a file path for saving/loading tasks.
pub struct Todo<'a> {
    tasks: Vec<Task>,
    path: &'a str,
}

/// A type alias for the result of a Todo operation.
pub type TodoResult<T> = Result<T, Box<TodoError>>;

// Add custom error type
#[derive(Debug)]
pub enum TodoError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    IndexError(usize),
    ValidationError(String),
}

impl std::error::Error for TodoError {}
impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::IoError(e) => write!(f, "IO error: {}", e),
            TodoError::JsonError(e) => write!(f, "JSON error: {}", e),
            TodoError::IndexError(i) => write!(f, "Invalid index: {}", i),
            TodoError::ValidationError(s) => write!(f, "Validation error: {}", s),
        }
    }
}

// Update Drop implementation
impl Drop for Todo<'_> {
    fn drop(&mut self) {
        debug!("Saving tasks to file: {} from Drop trait", self.path);
        if let Err(e) = self.save() {
            error!("Failed to save tasks on drop: {}", e);
        }
    }
}

/// A implementation of the Todo struct.
/// This struct represents a todo list and provides methods to manage tasks.
impl<'a> Todo<'a> {
    /// Creates a new Todo instance with the given file path.
    pub fn new(path: &'a str) -> TodoResult<Self> {
        trace!("Creating a new todo instance with path: {}", path);
        let mut t = Todo {
            tasks: vec![],
            path,
        };

        trace!("Loading tasks from file: {}", t.path);
        t.load()?;
        Ok(t)
    }

    /// Returns the list of tasks.
    pub fn list(&self) -> TodoResult<Vec<Task>> {
        debug!(
            "Listing all tasks, this len of tasks size is {:?}",
            self.tasks.len()
        );
        Ok(self.tasks.clone())
    }

    /// Add a new task to the todo list.
    pub fn add(&mut self, task: Task) -> TodoResult<()> {
        // Add validation
        if task.description.trim().is_empty() {
            return Err(Box::new(TodoError::ValidationError(
                "Task description cannot be empty".into(),
            )));
        }
        debug!("Adding a new task: {:?}", task);
        self.tasks.push(task);
        Ok(())
    }

    /// Delete a task by index
    pub fn delete(&mut self, index: usize) -> TodoResult<()> {
        debug!("Deleting task at index: {}", index);
        if index >= self.tasks.len() {
            error!("No task found at index: {}", index);
            return Err(Box::new(TodoError::IndexError(index)));
        }

        self.tasks.remove(index);
        Ok(())
    }

    /// Complete a task by index
    /// If the task is already completed, it will be marked as incomplete.
    pub fn complete(&mut self, index: usize) -> TodoResult<()> {
        debug!("Completing task at index: {}", index);
        match self.tasks.get_mut(index) {
            None => {
                error!("No task found at index: {}", index);
                Err(Box::new(TodoError::IndexError(index)))
            }
            Some(task) => {
                debug!("Completing task: {:?}", task);
                task.completed = !task.completed;
                Ok(())
            }
        }
    }

    /// Load the tasks from the file in JSON format.
    pub fn load(&mut self) -> TodoResult<usize> {
        debug!("Loading tasks from file: {}", self.path);

        match File::open(self.path) {
            Ok(mut file) => {
                // open the file and get its metadata
                let metadata = match file.metadata() {
                    Ok(m) => m,
                    Err(e) => return Err(Box::new(TodoError::IoError(e))),
                };

                // read the file contents into a string
                let mut contents = String::with_capacity(metadata.len() as usize);
                file.read_to_string(&mut contents).unwrap();

                // parse the JSON string into a vector of tasks
                self.tasks = serde_json::from_str(&contents)
                    .map_err(|e| Box::new(TodoError::JsonError(e)))?;

                // return the number of tasks
                Ok(self.tasks.len())
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                error!(
                    "File not found: {}, initializing empty task list",
                    self.path
                );
                self.tasks = Vec::with_capacity(10);
                Ok(0)
            }
            Err(e) => Err(Box::new(TodoError::IoError(e))),
        }
    }

    /// Save the tasks to the file in JSON format.
    pub fn save(&mut self) -> TodoResult<()> {
        debug!("Saving tasks to file: {}", self.path);
        let json = serde_json::to_string_pretty(&self.tasks)
            .map_err(|e| Box::new(TodoError::JsonError(e)))?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path)
            .map_err(|e| Box::new(TodoError::IoError(e)))?;

        file.write_all(json.as_bytes())
            .map_err(|e| Box::new(TodoError::IoError(e)))?;

        Ok(())
    }

    /// Clear all tasks from the todo list.
    pub fn clear(&mut self) -> TodoResult<()> {
        debug!("Clearing all tasks");
        self.tasks.clear();
        Ok(())
    }
}
