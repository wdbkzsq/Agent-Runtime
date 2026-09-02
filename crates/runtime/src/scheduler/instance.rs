use crate::task::{Task, TaskState};
use std::collections::{HashSet, VecDeque};

#[derive(Default)]
pub struct Scheduler {
    ready_queue: VecDeque<Task>,
    finished: HashSet<String>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            finished: HashSet::new(),
        }
    }
    pub fn submit(&mut self, mut task: Task) {
        if task.can_run(&self.finished) {
            task.state = TaskState::Ready;
            println!("[Scheduler] Task {} READY", task.id);
            self.ready_queue.push_back(task);
        } else {
            task.state = TaskState::Blocked;
            println!("[Scheduler] Task {} BLOCKED", task.id);
        }
    }
    pub fn fetch_next(&mut self) -> Option<Task> {
        let mut task: Task = self.ready_queue.pop_front()?;
        task.state = TaskState::Running;
        println!("[Scheduler] Running {}", task.id);
        Some(task)
    }
    pub fn finish(&mut self, task: &Task) {
        println!("[Scheduler] {} SUCCESS", task.id);
        self.finished.insert(task.id.clone());
    }
    pub fn queue_size(&self) -> usize {
        self.ready_queue.len()
    }
}
