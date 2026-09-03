mod agent;
mod executor;
mod graph;
mod scheduler;
mod task;
mod worker;

use agent::*;
use executor::*;
use graph::*;
use scheduler::*;
use std::env;
use task::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "worker" {
        worker::run(args[2].clone());
        return;
    }
    println!("=== Agent Runtime Day2 ===");
    let mut scheduler = Scheduler::new();
    let task = Task::new("task-A", "research-agent", 10);
    scheduler.submit(task);
    let task = scheduler.fetch_next().unwrap();
    println!("[Runtime] dispatch {}", task.id);
    let mut agent = AgentInstance::new("agent-001", &task.agent_type);
    println!("[Runtime] create {:?}", agent.state);
    let mut child = Executor::new().execute(&mut agent);
    let status = child.wait().unwrap();
    if status.success() {
        agent.complete();
        println!("[Runtime] Agent {} COMPLETED", agent.id);
        scheduler.finish(&task);
    } else {
        println!("[Runtime] Agent FAILED");
    }
}
