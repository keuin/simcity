use crate::agent::agent::Agent;
use crate::agent::metrics::AgentMetrics;
use crate::agent::state::AgentState;
use crate::city::cell::{CellType, Position};
use crate::city::grid::CityGrid;
use crate::simulation::config::SimulationConfig;
use crate::simulation::metrics::{calc_metrics, SimulationMetrics};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{atomic, Arc};
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};
use tokio::time;

#[derive(Debug)]
pub struct Simulation {
    pub city: CityGrid,
    pub agents: HashMap<String, Agent>,
    pub config: SimulationConfig,
    pub current_time: WorldTime,
    pub tick_updates_broadcaster: broadcast::Sender<SimulationUpdate>,
}

impl Simulation {
    pub fn new(city: CityGrid, config: SimulationConfig) -> Self {
        let (tx, _) = broadcast::channel(100);

        Self {
            city,
            agents: HashMap::new(),
            config,
            current_time: 0,
            tick_updates_broadcaster: tx,
        }
    }

    pub fn initialize(&mut self) {
        let mut rng = rand::rng();
        let houses = self.city.find_cells_of_type(CellType::House);
        let offices = self.city.find_cells_of_type(CellType::Office);

        if houses.is_empty() || offices.is_empty() {
            return;
        }

        for i in 0..self.config.num_agents {
            let home = houses[rng.random_range(0..houses.len())];
            let work = offices[rng.random_range(0..offices.len())];

            let parks = self.city.find_cells_of_type(CellType::Park);
            let park = if !parks.is_empty() {
                parks[rng.random_range(0..parks.len())]
            } else {
                work
            };
            let agent = Agent::new(format!("agent-{}", i), home, work, park);
            if let Some(cell) = self.city.get_cell_mut(&home) {
                cell.add_occupant(&agent.id);
            }
            self.agents.insert(agent.id.clone(), agent);
        }
    }

    pub async fn run(sim: Arc<Mutex<Simulation>>, running: Arc<AtomicBool>) {
        let tick_interval = {
            let mut sim = sim.lock().await;
            sim.current_time = 0;
            Duration::from_secs_f64(1.0 / sim.config.tick_rate as f64)
        };
        let mut interval = time::interval(tick_interval);
        // run until running is set to false
        while running.load(atomic::Ordering::Relaxed) {
            interval.tick().await;
            let mut sim = sim.lock().await;
            let tick_updates = sim.do_tick();
            _ = sim.tick_updates_broadcaster.send(tick_updates);
        }
    }

    fn do_tick(&mut self) -> SimulationUpdate {
        self.current_time += 1;
        let now = self.current_time;
        let mut updates = SimulationUpdate {
            timestamp: now,
            agents: vec![],
            metrics: SimulationMetrics {
                timestamp: now,
                average_commute_time: 0.0,
                average_distance: 0.0,
                congestion_map: Default::default(),
                most_congested_position: None,
                max_congestion: 0,
                energy_usage: 0.0,
            },
        };

        // calc stats
        let agent_metrics = self
            .agents
            .values()
            .map(AgentMetrics::from)
            .collect::<Vec<_>>();

        for (_, agent) in self.agents.iter_mut() {
            let original_position = agent.position;
            agent.find_goal_path(&self.city);
            agent.move_along_path(now, &self.city);
            if original_position != agent.position {
                if let Some(cell) = self.city.get_cell_mut(&original_position) {
                    cell.remove_occupant(&agent.id);
                }
                if let Some(cell) = self.city.get_cell_mut(&agent.position) {
                    cell.add_occupant(&agent.id);
                }
            }
            updates.agents.push(AgentUpdate {
                id: agent.id.clone(),
                position: agent.position,
                state: agent.state,
            });
            // update state after position has changed
            agent.update_state(now);
        }

        updates.metrics = calc_metrics(&self.city, &agent_metrics, self.current_time);

        updates
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SimulationUpdate> {
        self.tick_updates_broadcaster.subscribe()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationUpdate {
    pub timestamp: WorldTime,
    pub agents: Vec<AgentUpdate>,
    pub metrics: SimulationMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentUpdate {
    pub id: String,
    pub position: Position,
    pub state: AgentState,
}

pub type WorldTime = i64;
