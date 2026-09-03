use std::collections::HashMap;

use crate::task::*;

pub struct TaskGraph {
    pub tasks: HashMap<String, Task>,
}

impl TaskGraph {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
    }
}
