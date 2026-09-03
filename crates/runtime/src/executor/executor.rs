use std::process::{Child, Command};

use crate::agent::AgentInstance;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute(&self, agent: &mut AgentInstance) -> Child {
        println!("[Executor] start agent {}", agent.id);
        let child = Command::new(std::env::current_exe().unwrap())
            .arg("worker")
            .arg(&agent.id)
            .spawn()
            .expect("failed spawn agent");
        let pid = child.id();
        agent.set_running(pid);
        println!("[Executor] Agent {} PID={}", agent.id, pid);
        child
    }
}
