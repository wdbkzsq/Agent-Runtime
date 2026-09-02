use std::process::Child;

#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    CREATED,
    READY,
    RUNNING,
    COMPLETED,
    FAILED,
}

pub struct AgentInstance {
    pub id: String,
    pub agent_type: String,
    pub state: AgentState,
    pub pid: Option<u32>,
}

impl AgentInstance {
    pub fn new(id: &str, agent_type: &str) -> Self {
        Self {
            id: id.to_string(),
            agent_type: agent_type.to_string(),
            state: AgentState.CREATED,
            pid: None,
        }
    }
    pub fn set_running(&mut self, pid: u32) {
        self.pid = Some(pid);
        self.state = AgentState.RUNNING;
    }
    pub fn complete(&mut self) {
        self.state = AgentState.COMPLETED;
    }
}
