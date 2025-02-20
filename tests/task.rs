/*
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co,.Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: task.rs
 * Author: mingcheng (mingcheng@apache.org)
 * File Created: 2025-02-19 23:10:08
 *
 * Modified By: mingcheng (mingcheng@apache.org)
 * Last Modified: 2025-02-19 23:18:16
 */

use todo::task::Task;

#[test]
fn test_create_task() {
    let task = Task {
        description: String::from("Test task"),
        completed: false,
    };
    assert_eq!(task.description, "Test task");
    assert_eq!(task.completed, false);
}

#[test]
fn test_task_serialization() {
    let task = Task {
        description: String::from("Test serialization"),
        completed: true,
    };

    let serialized = serde_json::to_string(&task).unwrap();
    let expected = r#"{"description":"Test serialization","completed":true}"#;
    assert_eq!(serialized, expected);
}

#[test]
fn test_task_deserialization() {
    let json = r#"{"description":"Test deserialization","completed":false}"#;
    let task: Task = serde_json::from_str(json).unwrap();

    assert_eq!(task.description, "Test deserialization");
    assert_eq!(task.completed, false);
}

#[test]
fn test_task_clone() {
    let task = Task {
        description: String::from("Original task"),
        completed: false,
    };

    let cloned_task = task.clone();
    assert_eq!(task.description, cloned_task.description);
    assert_eq!(task.completed, cloned_task.completed);
}

#[test]
fn test_empty_description() {
    let task = Task {
        description: String::new(),
        completed: false,
    };
    assert_eq!(task.description, "");
}
