/*
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co,.Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: todo.rs
 * Author: mingcheng (mingcheng@apache.org)
 * File Created: 2025-02-19 17:43:27
 *
 * Modified By: mingcheng (mingcheng@apache.org)
 * Last Modified: 2025-02-24 10:12:37
 */

#[cfg(test)]
mod tests {
    use todo::task::Task;
    use todo::todo::{Todo, TodoResult};

    const FILE_PATH: &str = "test.json";
    const TEST_DESCRIPTION: &str = "test task";

    fn setup() -> TodoResult<Todo<'static>> {
        match Todo::new(FILE_PATH) {
            Ok(mut t) => {
                let _ = t.clear()?;
                Ok(t)
            }
            Err(e) => {
                println!("Failed to create Todo: {}", e);
                Err(e)
            }
        }
    }

    #[test]
    fn test_create_and_add_task() -> TodoResult<()> {
        let mut todo = setup()?;

        // Test adding a task
        todo.add(Task {
            description: TEST_DESCRIPTION.to_string(),
            completed: false,
        })?;

        // Verify task was added
        let tasks = todo.list()?;
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, TEST_DESCRIPTION);
        assert!(!tasks[0].completed);

        // Cleanup
        let _ = todo.clear()?;
        Ok(())
    }

    #[test]
    fn test_complete_task() -> TodoResult<()> {
        let mut todo = setup()?;

        // Add and complete a task
        todo.add(Task {
            description: TEST_DESCRIPTION.to_string(),
            completed: false,
        })?;

        todo.complete(0)?;

        // Verify task was completed
        let tasks = todo.list()?;
        assert!(tasks[0].completed);

        // Cleanup
        let _ = todo.clear()?;
        Ok(())
    }

    #[test]
    fn test_delete_task() -> TodoResult<()> {
        let mut todo = setup()?;

        // Add and then delete a task
        todo.add(Task {
            description: TEST_DESCRIPTION.to_string(),
            completed: false,
        })?;

        todo.delete(0)?;

        // Verify task was deleted
        let tasks = todo.list()?;
        assert!(tasks.is_empty());

        // Cleanup
        let _ = todo.clear()?;
        Ok(())
    }
}
