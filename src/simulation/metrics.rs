use crate::agent::metrics::AgentMetrics;
use crate::city::cell::{CellType, Position};
use crate::city::grid::CityGrid;
use crate::simulation::simulation::WorldTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationMetrics {
    pub timestamp: WorldTime,
    pub average_commute_time: f64,
    pub average_distance: f64,
    pub congestion_map: HashMap<String, usize>,
    pub most_congested_position: Option<Position>,
    pub max_congestion: usize,
    pub energy_usage: f64,
}

pub fn calc_metrics(
    city: &CityGrid,
    agent_metrics: &[AgentMetrics],
    current_time: WorldTime,
) -> SimulationMetrics {
    let total_commute_time = agent_metrics
        .iter()
        .map(|m| m.total_commute_time)
        .sum::<i64>() as u64;
    let average_commute_time = if !agent_metrics.is_empty() {
        total_commute_time as f64 / agent_metrics.len() as f64
    } else {
        0.0
    };

    let total_distance: usize = agent_metrics.iter().map(|m| m.total_distance).sum();
    let average_distance = if !agent_metrics.is_empty() {
        total_distance as f64 / agent_metrics.len() as f64
    } else {
        0.0
    };

    let mut congestion_map = HashMap::new();
    let mut max_congestion = 0;
    let mut most_congested_position = None;
    for y in 0..city.height {
        for x in 0..city.width {
            let position = Position { x, y };
            if let Some(cell) = city.get_cell(&position) {
                if cell.cell_type == CellType::Road {
                    let occupants = cell.occupant_count();
                    if occupants > 0 {
                        let key = format!("{},{}", x, y);
                        congestion_map.insert(key, occupants);

                        if occupants > max_congestion {
                            max_congestion = occupants;
                            most_congested_position = Some(position);
                        }
                    }
                }
            }
        }
    }

    // simple energy cost model:
    // energy = base_energy + movement_energy + time_energy
    // base_energy: energy consumed without doing anything
    // movement_energy: extra energy consumed when moving
    // time_energy: energy consumed when time is spent on commune
    let base_energy = agent_metrics.len() as f64 * 0.125;
    let movement_energy = agent_metrics
        .iter()
        .map(|agent| agent.total_distance)
        .sum::<usize>() as f64
        * 0.25;
    let time_energy = agent_metrics
        .iter()
        .map(|agent| agent.total_commute_time)
        .sum::<WorldTime>() as f64
        * 0.125;
    let energy_usage = base_energy + movement_energy + time_energy;

    SimulationMetrics {
        timestamp: current_time,
        average_commute_time,
        average_distance,
        congestion_map,
        most_congested_position,
        max_congestion,
        energy_usage,
    }
}
