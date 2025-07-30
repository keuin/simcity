mod agent;
mod city;
mod simulation;
mod visualization;

use city::config::CityConfig;
use simulation::config::SimulationConfig;
use simulation::simulation::Simulation;
use std::sync::Arc;
use tokio::sync::Mutex;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let city_config = CityConfig::default();
    log::info!("city config: {}", serde_json::to_string(&city_config)?);
    let city = city_config.to_city_grid();
    log::info!("initial city grid:\n{}", city);
    let sim_config = SimulationConfig::default();
    let mut sim = Simulation::new(city, sim_config);
    sim.initialize();
    let addr = "127.0.0.1:8000".parse().expect("invalid endpoint string");
    log::info!("starting visualization server at http://{}", addr);
    visualization::start_server(Arc::new(Mutex::new(sim)), addr).await
}
