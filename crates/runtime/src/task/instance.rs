use std::collections::HashSet;
use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Created,
    Blocked,
    Ready,
    Running,
    Success,
    Failed,
}
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub agent_type: String,
    pub state: TaskState,
    pub priority: u32,
    pub dependencies: Vec<String>,
}

impl Task {
    pub fn new(id: &str, agent_type: &str, priority: u32) -> Self {
        Self {
            id: id.to_string(),
            agent_type: agent_type.to_string(),
            state: TaskState::Created,
            priority,
            dependencies: Vec::new(),
        }
    }
    pub fn add_dependency(&mut self, task_id: &str) {
        self.dependencies.push(task_id.to_string());
    }
    pub fn can_run(&self, finished_tasks: &HashSet<String>) -> bool {
        for dep in &self.dependencies {
            if !finished_tasks.contains(dep) {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task(id={}, agent={}, state={:?}, priority={})",
            self.id, self.agent_type, self.state, self.priority
        )
    }
}
