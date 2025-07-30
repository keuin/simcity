use crate::agent::state::AgentState;
use crate::city::cell::{CellType, Position};
use crate::city::grid::CityGrid;
use crate::simulation::simulation::WorldTime;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub position: Position,
    pub home: Position,
    pub work: Position,
    pub park: Position,
    pub state: AgentState,
    pub work_counter: u8,
    pub park_visits_remaining: u8,
    pub path: Vec<Position>, // current_position -> goal
    pub speed: f64,          // cells per second
    pub last_move: Option<i64>,
    pub total_distance: usize,
    pub total_commute_time: WorldTime,
    pub commute_start: Option<i64>,
}

impl Agent {
    pub fn new(id: String, home: Position, work: Position, park: Position) -> Self {
        Self {
            id,
            position: home,
            home,
            work,
            park,
            state: AgentState::AtHome,
            work_counter: 0,
            park_visits_remaining: 0,
            path: Vec::new(),
            speed: 1.0,
            last_move: None,
            total_distance: 0,
            total_commute_time: 0,
            commute_start: None,
        }
    }

    pub fn get_goal(&self) -> Position {
        match self.state {
            AgentState::GoingToWork | AgentState::Wandering => self.work,
            AgentState::GoingHome | AgentState::AtWork => self.home,
            AgentState::GoingToPark => self.park,
            AgentState::AtPark => self.park,
            AgentState::AtHome => self.home,
        }
    }

    pub fn has_reached_goal(&self) -> bool {
        self.position == self.get_goal()
    }

    pub fn update_state(&mut self, now: WorldTime) {
        match self.state {
            AgentState::AtHome => {
                self.work_counter = (self.work_counter + 1) % 5;
                if self.work_counter == 0 {
                    self.park_visits_remaining = 2;
                    self.state = AgentState::GoingToPark;
                } else {
                    self.state = AgentState::GoingToWork;
                }
                self.commute_start = Some(now);
            }
            AgentState::GoingToWork => {
                if self.has_reached_goal() {
                    self.state = AgentState::AtWork;
                    if let Some(start) = self.commute_start {
                        self.total_commute_time += now;
                        self.commute_start = None;
                    }
                }
            }
            AgentState::AtWork => {
                self.state = AgentState::GoingHome;
                self.commute_start = Some(now);
            }
            AgentState::GoingHome => {
                if self.has_reached_goal() {
                    self.state = AgentState::AtHome;
                    if let Some(start) = self.commute_start {
                        self.total_commute_time += now - start;
                        self.commute_start = None;
                    }
                }
            }
            AgentState::Wandering => {
                if self.has_reached_goal() {
                    self.state = AgentState::AtWork;
                }
            }
            AgentState::GoingToPark => {
                if self.has_reached_goal() {
                    self.state = AgentState::AtPark;
                    self.commute_start = Some(now);
                }
            }
            AgentState::AtPark => {
                if self.park_visits_remaining > 0 {
                    self.park_visits_remaining -= 1;
                    self.state = AgentState::GoingToPark;
                } else {
                    self.state = AgentState::GoingToWork;
                }
                self.commute_start = Some(now);
            }
        }
    }

    pub fn move_along_path(&mut self, now: WorldTime, city: &CityGrid) -> bool {
        if self.path.is_empty() {
            return false;
        }

        let next_pos = self.path[0];

        if let Some(cell) = city.get_cell(&next_pos) {
            // do not move into empty blocks
            // the path finding algorithm should already have guaranteed that
            // if not, this is a bug, print an error log
            if cell.cell_type == CellType::Empty {
                log::error!("agent {}: moving to empty block at {:?}", self.id, next_pos);
                self.path.clear();
                return false;
            }
        }

        let should_move = match self.last_move {
            Some(last) => {
                let elapsed = now - last;
                elapsed as f64 >= 1.0 / self.speed
            }
            None => true,
        };

        if should_move {
            let next_pos = self.path[0];
            log::debug!(
                "agent {}: attempting move from {:?} to {:?}",
                self.id,
                self.position,
                next_pos
            );

            self.total_distance += self.position.distance(&next_pos);
            self.position = next_pos;
            self.path.remove(0);
            self.last_move = Some(now);
            log::debug!("agent {}: moved to {:?}", self.id, self.position);
        }
        should_move
    }

    /// find the path to goal using BFS
    pub fn find_goal_path(&mut self, city: &CityGrid) {
        let goal = self.get_goal();
        self.path.clear();
        if self.has_reached_goal() {
            return;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back(self.position);
        visited.insert(self.position, None);

        const MAX_DEPTH: usize = 1024; // BFS max search depth
        let mut depth = 0;
        let mut found = false;
        while !queue.is_empty() && !found && depth < MAX_DEPTH {
            depth += 1;
            let current = queue.pop_front().unwrap();

            found = current == goal;
            if found {
                break;
            }

            const MOVE_DIRECTIONS: [(i32, i32); 4] = [
                (0, 1),  // down
                (1, 0),  // right
                (0, -1), // up
                (-1, 0), // left
            ];
            for (x, y) in MOVE_DIRECTIONS {
                let next_pos = Position {
                    x: if x < 0 && current.x == 0 {
                        continue;
                    } else if x > 0 {
                        current.x + (x as usize)
                    } else if x < 0 {
                        current.x - (-x as usize)
                    } else {
                        current.x
                    },
                    y: if y < 0 && current.y == 0 {
                        continue;
                    } else if y > 0 {
                        current.y + (y as usize)
                    } else if y < 0 {
                        current.y - (-y as usize)
                    } else {
                        current.y
                    },
                };
                if visited.contains_key(&next_pos) {
                    continue;
                }
                match city.get_cell(&next_pos) {
                    Some(cell) => match cell.cell_type {
                        CellType::Empty => continue,
                        _ => {
                            // possible, try visit
                            queue.push_back(next_pos);
                            visited.insert(next_pos, Some(current));
                        }
                    },
                    None => continue,
                }
            }
        }
        if found {
            let mut current = goal;
            let mut path = Vec::new();
            while current != self.position {
                path.push(current);
                current = visited[&current].unwrap();
            }
            self.path = path.into_iter().rev().collect();
        } else {
            let g = self.get_goal();
            log::error!("agent {}: no path to goal ({},{})", self.id, g.x, g.y);
            // fallback to a direct path to goal, regardless of buildings on map
            if let Some(cell) = city.get_cell(&self.position) {
                if cell.cell_type != CellType::Road {
                    if let Some(nearest_road) =
                        city.find_nearest_cell_with_type(&self.position, CellType::Road)
                    {
                        let mut current = self.position;

                        // move along X axis
                        while current.x != nearest_road.x {
                            if current.x < nearest_road.x {
                                current.x += 1;
                            } else {
                                current.x -= 1;
                            }
                            self.path.push(current);
                        }

                        // move along Y axis
                        while current.y != nearest_road.y {
                            if current.y < nearest_road.y {
                                current.y += 1;
                            } else {
                                current.y -= 1;
                            }
                            self.path.push(current);
                        }
                    }
                }
            }
        }
    }
}
