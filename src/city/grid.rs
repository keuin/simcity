use crate::city::cell::{Cell, CellType, Position};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl CityGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let position = Position::new(x, y);
                        Cell::new(CellType::Empty, position)
                    })
                    .collect()
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get_cell(&self, position: &Position) -> Option<&Cell> {
        if position.y < self.height && position.x < self.width {
            Some(&self.cells[position.y][position.x])
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, position: &Position) -> Option<&mut Cell> {
        if position.y < self.height && position.x < self.width {
            Some(&mut self.cells[position.y][position.x])
        } else {
            None
        }
    }

    pub fn set_cell_type(
        &mut self,
        position: &Position,
        cell_type: CellType,
    ) -> Result<(), String> {
        if let Some(cell) = self.get_cell_mut(position) {
            cell.cell_type = cell_type;
            Ok(())
        } else {
            Err(format!(
                "position ({}, {}) is out of bounds",
                position.x, position.y
            ))
        }
    }

    pub fn find_cells_of_type(&self, cell_type: CellType) -> Vec<Position> {
        let mut positions = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position::new(x, y);
                if let Some(cell) = self.get_cell(&position) {
                    if cell.cell_type == cell_type {
                        positions.push(position);
                    }
                }
            }
        }

        positions
    }

    pub fn find_nearest_cell_with_type(
        &self,
        from: &Position,
        cell_type: CellType,
    ) -> Option<Position> {
        let cells_of_type = self.find_cells_of_type(cell_type);

        cells_of_type
            .into_iter()
            .min_by_key(|pos| from.distance(pos))
    }
}

impl Display for CityGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                let display = if !cell.occupants.is_empty() {
                    format!("{}", cell.occupants.len())
                } else {
                    format!("{}", cell.cell_type)
                };
                f.write_str(&display)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
