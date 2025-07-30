use crate::agent::agent::Agent;
use crate::agent::state::AgentState;
use crate::city::cell::Position;
use serde::{Deserialize, Serialize};
use crate::simulation::simulation::WorldTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub id: String,
    pub state: AgentState,
    pub total_distance: usize,
    pub total_commute_time: WorldTime,
    pub current_position: Position,
}

impl From<&Agent> for AgentMetrics {
    fn from(agent: &Agent) -> Self {
        AgentMetrics {
            id: agent.id.clone(),
            state: agent.state,
            total_distance: agent.total_distance,
            total_commute_time: agent.total_commute_time,
            current_position: agent.position,
        }
    }
}
