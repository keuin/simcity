use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellType {
    Road,
    House,
    Office,
    Park,
    EnergyStation,
    Empty,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellType::Road => write!(f, "R"),
            CellType::House => write!(f, "H"),
            CellType::Office => write!(f, "O"),
            CellType::Park => write!(f, "P"),
            CellType::EnergyStation => write!(f, "E"),
            CellType::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Manhattan distance
    pub fn distance(&self, other: &Position) -> usize {
        let x_diff = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let y_diff = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        x_diff + y_diff
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: CellType,
    pub position: Position,
    pub occupants: Vec<String>, // agents occupying this cell
}

impl Cell {
    pub fn new(cell_type: CellType, position: Position) -> Self {
        Self {
            cell_type,
            position,
            occupants: Vec::new(),
        }
    }

    pub fn occupant_count(&self) -> usize {
        self.occupants.len()
    }

    pub fn add_occupant(&mut self, agent_id: &str) {
        self.occupants.push(agent_id.to_owned());
    }

    pub fn remove_occupant(&mut self, agent_id: &str) {
        self.occupants.retain(|id| id != agent_id);
    }
}
