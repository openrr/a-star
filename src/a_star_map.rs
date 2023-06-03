use crate::score::*;
use grid_map::{Cell, GridMap, Position};

pub struct AStarMap {
    map: GridMap<Score>,
    start: Position,
    goal: Position,
}

impl AStarMap {
    pub fn new(grid_map: GridMap<Score>, start: Position, goal: Position) -> Self {
        if grid_map.min_point().x > start.x
            || grid_map.min_point().y > start.y
            || grid_map.max_point().x < start.x
            || grid_map.max_point().y < start.y
        {
            panic!("Start position is out of range");
        }
        Self {
            map: grid_map,
            start,
            goal,
        }
    }

    pub fn set_start(&mut self, start: Position) {
        if self.is_out_of_range(start) {
            panic!("Start position is out of range");
        }
        self.start = start;
    }
    pub fn set_goal(&mut self, goal: Position) {
        if self.is_out_of_range(goal) {
            panic!("Goal position is out of range");
        }
        self.goal = goal;
    }

    fn is_out_of_range(&self, position: Position) -> bool {
        self.map.min_point().x > position.x
            || self.map.min_point().y > position.y
            || self.map.max_point().x < position.x
            || self.map.max_point().y < position.y
    }

    pub fn search(&mut self) {
        let mut is_reached_to_goal = false;

        while is_reached_to_goal {
            let parent_cell = if self.is_only_start_node_be_serched() {
                self.map.to_grid(self.start.x, self.start.y).unwrap()
            } else {
            };
        }
    }

    fn is_only_start_node_be_serched(&self) -> bool {
        self.map
            .cells()
            .iter()
            .all(|cell| cell.is_obstacle() || cell.is_uninitialized())
    }

    fn searched_target_node_index(&self) -> Option<usize> {
        self.map
            .cells()
            .iter()
            .position(|cell| ce)
    }
}
