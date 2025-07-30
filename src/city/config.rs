use crate::city::cell::{CellType, Position};
use crate::city::grid::CityGrid;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const ROAD: &str = "Road";
const HOUSE: &str = "House";
const OFFICE: &str = "Office";
const PARK: &str = "Park";
const ENERGY_STATION: &str = "EnergyStation";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellConfig {
    pub x: usize,
    pub y: usize,
    pub cell_type: String,
}

/// city map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityConfig {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<CellConfig>,
}

impl CityConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("failed to open config file: {}", e))?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| format!("failed to parse config: {}", e))
    }

    pub fn default() -> Self {
        let mut cells = Vec::new();

        for x in 0..10 {
            cells.push(CellConfig {
                x,
                y: 2,
                cell_type: ROAD.to_string(),
            });
            cells.push(CellConfig {
                x,
                y: 5,
                cell_type: ROAD.to_string(),
            });
            cells.push(CellConfig {
                x,
                y: 8,
                cell_type: ROAD.to_string(),
            });
        }

        for y in 0..10 {
            cells.push(CellConfig {
                x: 1,
                y,
                cell_type: ROAD.to_string(),
            });
            cells.push(CellConfig {
                x: 4,
                y,
                cell_type: ROAD.to_string(),
            });
            cells.push(CellConfig {
                x: 7,
                y,
                cell_type: ROAD.to_string(),
            });
        }

        for x in 2..4 {
            for y in 0..2 {
                cells.push(CellConfig {
                    x,
                    y,
                    cell_type: HOUSE.to_string(),
                });
            }
        }

        for x in 8..10 {
            for y in 6..8 {
                cells.push(CellConfig {
                    x,
                    y,
                    cell_type: OFFICE.to_string(),
                });
            }
        }

        cells.push(CellConfig {
            x: 9,
            y: 0,
            cell_type: ENERGY_STATION.to_string(),
        });
        cells.push(CellConfig {
            x: 0,
            y: 9,
            cell_type: ENERGY_STATION.to_string(),
        });

        cells.push(CellConfig {
            x: 5,
            y: 3,
            cell_type: PARK.to_string(),
        });
        cells.push(CellConfig {
            x: 5,
            y: 4,
            cell_type: PARK.to_string(),
        });
        cells.push(CellConfig {
            x: 6,
            y: 3,
            cell_type: PARK.to_string(),
        });
        cells.push(CellConfig {
            x: 6,
            y: 4,
            cell_type: PARK.to_string(),
        });

        Self {
            width: 10,
            height: 10,
            cells,
        }
    }

    pub fn to_city_grid(&self) -> CityGrid {
        let mut grid = CityGrid::new(self.width, self.height);
        for cell_config in &self.cells {
            let position = Position::new(cell_config.x, cell_config.y);
            let cell_type = CellType::from(&cell_config.cell_type);
            let _ = grid.set_cell_type(&position, cell_type);
        }
        grid
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let file =
            File::create(path).map_err(|e| format!("failed to create config file: {}", e))?;
        serde_json::to_writer_pretty(file, self)
            .map_err(|e| format!("failed to write config: {}", e))
    }
}

impl<T: AsRef<str>> From<T> for CellType {
    fn from(value: T) -> Self {
        match value.as_ref() {
            ROAD => CellType::Road,
            HOUSE => CellType::House,
            OFFICE => CellType::Office,
            PARK => CellType::Park,
            ENERGY_STATION => CellType::EnergyStation,
            _ => CellType::Empty,
        }
    }
}
