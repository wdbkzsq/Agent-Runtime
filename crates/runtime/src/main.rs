mod scheduler;
mod task;

use scheduler::*;
use task::*;

fn main() {
    println!("=== Agent Runtime Day1 ===");
    let mut scheduler = Scheduler::new();

    let task_a = Task::new("task_a", "research-agent", 10);
    scheduler.submit(task_a);
    let mut task_b = Task::new("task_b", "coding-agent", 5);
    task_b.add_dependency("task_a");
    scheduler.submit(task_b);

    println!("\nReady Queue size = {}", scheduler.queue_size());

    let running = scheduler.fetch_next().unwrap();

    println!("Execute {}", running);
    scheduler.finish(&running);
}
