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
 * Last Modified: 2025-02-20 10:16:25
 */

#[cfg(test)]
mod tests {
    use todo::task::Task;
    use todo::todo::Todo;

    const FILE_PATH: &str = "test.json";

    #[test]
    fn test_create_todo() {
        match Todo::new(FILE_PATH) {
            Ok(mut t) => {
                t.clear();
                t.add(Task {
                    description: String::from(""),
                    completed: false,
                });

                match t.list() {
                    Ok(tasks) => {
                        assert_eq!(tasks.len(), 1)
                    }
                    Err(e) => {
                        panic!("Failed to list todo: {}", e);
                    }
                }
                t.clear();
            }
            Err(e) => {
                panic!("Failed to create todo: {}", e);
            }
        }
    }
}
