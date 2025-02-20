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
 * Last Modified: 2025-02-20 10:37:30
 */

use crate::task::Task;
use log::{debug, error, trace};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub struct Todo {
    tasks: Vec<Task>,
    path: String,
}

pub type TodoResult<T> = Result<T, Box<dyn Error>>;

impl Drop for Todo {
    fn drop(&mut self) {
        debug!("Saving tasks to file: {} from Drop trait", self.path);
        self.save().unwrap();
    }
}

#[allow(unused)]
impl Todo {
    pub fn new(path: &str) -> TodoResult<Self> {
        trace!("Creating a new todo instance with path: {}", path);
        let mut t = Todo {
            tasks: vec![],
            path: String::from(path),
        };

        trace!("Loading tasks from file: {}", t.path);
        match t.load() {
            Ok(size) => {
                debug!("Loaded {} tasks from file: {}", size, t.path);
                Ok(t)
            }
            Err(e) => {
                error!("Failed to load tasks from file: {}", e);
                Err(e)
            }
        }
    }

    pub fn list(&self) -> TodoResult<Vec<Task>> {
        debug!(
            "Listing all tasks, this len of tasks size is {:?}",
            self.tasks.len()
        );
        Ok(self.tasks.clone())
    }

    pub fn add(&mut self, task: Task) {
        debug!("Adding a new task: {:?}", task);
        self.tasks.push(task)
    }

    pub fn delete(&mut self, index: usize) -> TodoResult<()> {
        debug!("Deleting task at index: {}", index);
        if index >= self.tasks.len() {
            error!("No task found at index: {}", index);
            return Err("No task found".into());
        }

        self.tasks.remove(index);
        Ok(())
    }

    pub fn complete(&mut self, index: usize) -> TodoResult<()> {
        debug!("Completing task at index: {}", index);
        match self.tasks.get_mut(index) {
            None => {
                error!("No task found at index: {}", index);
                Err("No task found".into())
            }
            Some(task) => {
                debug!("Completing task: {:?}", task);
                task.completed = !task.completed;
                Ok(())
            }
        }
    }

    pub fn load(&mut self) -> TodoResult<usize> {
        debug!("Loading tasks from file: {}", self.path);
        let mut file = match File::open(&self.path) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open file: {}, so mark tasks is empty", self.path);
                self.tasks = vec![];
                return Ok(0);
            }
        };

        let mut contents = String::new();
        trace!("Reading file contents");
        file.read_to_string(&mut contents)?;

        trace!("Parsing JSON contents");
        self.tasks = serde_json::from_str::<Vec<Task>>(&contents).unwrap_or_default();

        trace!("Loaded {} tasks from file: {}", self.tasks.len(), self.path);
        Ok(self.tasks.len())
    }

    pub(crate) fn save(&mut self) -> TodoResult<bool> {
        debug!("Saving tasks to file: {}", self.path);
        let json = serde_json::to_string_pretty(&self.tasks)?;

        trace!("Opening file for writing");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        trace!("Writing to file");
        match file.write_all(json.as_bytes()) {
            Ok(_) => {
                trace!("Successfully wrote to file");
                Ok(true)
            }
            Err(e) => {
                error!("Failed to write to file: {}", e);
                Err(e.into())
            }
        }
    }

    pub fn clear(&mut self) {
        debug!("Clearing all tasks");
        self.tasks.clear();
    }
}
