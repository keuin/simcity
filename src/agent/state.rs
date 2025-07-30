use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    AtHome,
    GoingToWork,
    AtWork,
    GoingHome,
    GoingToPark,
    AtPark,
    Wandering,
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentState::AtHome => write!(f, "At Home"),
            AgentState::GoingToWork => write!(f, "Going to Work"),
            AgentState::AtWork => write!(f, "At Work"),
            AgentState::GoingHome => write!(f, "Going Home"),
            AgentState::GoingToPark => write!(f, "Going to Park"),
            AgentState::AtPark => write!(f, "At Park"),
            AgentState::Wandering => write!(f, "Wandering"),
        }
    }
}
