use std::{thread, time::Duration};

pub fn run(agent_id: String) {
    println!("[Agent Process {}] started", agent_id);
    thread::sleep(Duration::from_secs(2));
    println!("[Agent Process {}] finished", agent_id);
}
