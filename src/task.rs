/*
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co,.Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: task.rs
 * Author: mingcheng (mingcheng@apache.org)
 * File Created: 2025-02-19 14:52:22
 *
 * Modified By: mingcheng (mingcheng@apache.org)
 * Last Modified: 2025-02-19 23:34:51
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}
