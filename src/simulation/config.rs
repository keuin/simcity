use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub num_agents: usize,
    pub tick_rate: i64, // TPS, tick per second
    pub work_duration: Duration,
    pub home_duration: Duration,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            num_agents: 20,
            tick_rate: 10,
            work_duration: Duration::from_secs(30),
            home_duration: Duration::from_secs(30),
        }
    }
}
