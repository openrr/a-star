use crate::node::Node;
use crate::node_status::NodeStatus;
use crate::score::Score;

const DELTA: f64 = 1e-07;

#[derive(Clone, Debug, Default)]
pub struct MapGrid {
    /// x, y
    min_point: (f64, f64),
    /// x, y
    max_point: (f64, f64),
    /// node_map[y][x]
    node_map: Vec<Vec<Node>>,
    resolution: f64,
}

impl MapGrid {
    pub fn new(min_point: (f64, f64), max_point: (f64, f64), resolution: f64) -> Self {
        let x_len = ((max_point.0 - min_point.0) / resolution + DELTA) as usize;
        let y_len = ((max_point.1 - min_point.1) / resolution + DELTA) as usize;
        let node_map = vec![vec![Node::default(); x_len]; y_len];
        Self {
            min_point,
            max_point,
            node_map,
            resolution,
        }
    }

    pub fn set_start_and_goal(
        &mut self,
        (s_x, s_y): (f64, f64),
        (g_x, g_y): (f64, f64),
    ) -> Result<(), std::string::String> {
        self.set_start((s_x, s_y))?;
        self.set_goal((g_x, g_y))?;
        self.init_start_and_goal()?;
        Ok(())
    }

    fn set_start(&mut self, (x, y): (f64, f64)) -> Result<(), std::string::String> {
        // TODO: Error handling
        if self.is_set_start() {
            Err("Start has already set.".to_string())
        } else {
            // TODO: Check index is valid or not.
            let idx = ((x - self.min_point.0) / self.resolution + DELTA) as usize;
            let idy = ((y - self.min_point.1) / self.resolution + DELTA) as usize;
            self.node_map[idy][idx] = Node {
                status: NodeStatus::Start,
                score: Some(Score {
                    real: 0,
                    estimate: 0,
                }),
            };
            Ok(())
        }
    }

    fn set_goal(&mut self, (x, y): (f64, f64)) -> Result<(), std::string::String> {
        // TODO: Error handling
        if self.is_set_goal() {
            Err("Goal has already set.".to_string())
        } else {
            // TODO: Check index is valid or not.
            let idx = ((x - self.min_point.0) / self.resolution + DELTA) as usize;
            let idy = ((y - self.min_point.1) / self.resolution + DELTA) as usize;
            self.node_map[idy][idx].status = NodeStatus::Goal;
            Ok(())
        }
    }

    pub fn set_obstacle(&mut self, obstacles: &[(f64, f64)]) {
        for &o in obstacles {
            let idx = ((o.0 - self.min_point.0 + DELTA) / self.resolution) as usize;
            let idy = ((o.1 - self.min_point.1 + DELTA) / self.resolution) as usize;
            println!("{},{} --> {},{}", o.0, o.1, idx, idy);
            self.node_map[idy][idx] = Node {
                status: NodeStatus::Disable,
                score: None,
            };
        }
    }

    pub fn get_start(&self) -> Option<(usize, usize)> {
        self.node_map
            .iter()
            .flatten()
            .position(|node| node.status == NodeStatus::Start)
            .map(|id| (id / self.node_map[0].len(), id % self.node_map[0].len()))
    }

    pub fn get_goal(&self) -> Option<(usize, usize)> {
        self.node_map
            .iter()
            .flatten()
            .position(|node| node.status == NodeStatus::Goal)
            .map(|id| (id / self.node_map[0].len(), id % self.node_map[0].len()))
    }

    pub fn is_set_start(&self) -> bool {
        self.node_map
            .iter()
            .flatten()
            .any(|node| node.status == NodeStatus::Start)
    }

    pub fn is_set_goal(&self) -> bool {
        self.node_map
            .iter()
            .flatten()
            .any(|node| node.status == NodeStatus::Goal)
    }

    pub fn search(&mut self) {
        // TODO: Check start and goal are set.
        let mut is_reached_goal = false;
        let height = self.node_map.len();
        let width = self.node_map[0].len();

        while !is_reached_goal {
            let parent_node_index;
            if self.is_only_start_node_be_searched() {
                parent_node_index = self.get_start().unwrap();
            } else {
                parent_node_index = self.searched_target_node_index();
                self.node_map[parent_node_index.0][parent_node_index.1].status = NodeStatus::Closed;
            }
            let parent_node_real_cost = self.node_map[parent_node_index.0][parent_node_index.1]
                .score
                .as_ref()
                .unwrap()
                .real;

            let mut new_target_indexes = vec![];

            for i in 0..3 {
                for j in 0..3 {
                    if i == 1 && j == 1 {
                        // Ignore myself
                        continue;
                    }
                    new_target_indexes.push((parent_node_index.0 + i, parent_node_index.1 + j));
                }
            }

            for &id in new_target_indexes
                .iter()
                .filter(|(idy, idx)| *idx > 0 && *idx <= width && *idy > 0 && *idy <= height)
            {
                is_reached_goal |=
                    self.update_to_searched_node(id.0 - 1, id.1 - 1, parent_node_real_cost);
            }

            // For debug
            // TODO: Remove
            #[cfg(feature = "develop")]
            {
                std::thread::sleep(std::time::Duration::from_millis(100));
                println!("{}", self.visualize_as_string());
            }
        }
    }

    pub fn get_path(&self) -> Vec<(f64, f64)> {
        let mut path = vec![];

        let start = self.get_start().unwrap();
        let goal = self.get_goal().unwrap();
        let mut current = goal;

        while current != start {
            let mut min_id = (current.0 + 1, current.1 + 1);
            let aaa = current.0.wrapping_sub(1);
        }

        path
    }

    fn init_start_and_goal(&mut self) -> Result<(), std::string::String> {
        if let Some(start) = self.get_start() {
            self.node_map[start.0][start.1].score = Some(Score {
                real: 0,
                estimate: self.get_estimate_cost(start.0, start.1).unwrap(),
            });
            Ok(())
        } else {
            Err("Start is not set".to_string())
        }
    }

    fn is_only_start_node_be_searched(&self) -> bool {
        self.node_map
            .iter()
            .flatten()
            .all(|node| node.status != NodeStatus::Open)
            && self.is_set_start()
    }

    pub fn highest_scored_node_index(&self) -> (usize, usize) {
        let max_node = self
            .node_map
            .iter()
            .flatten()
            .filter(|&n| n.score.is_some())
            .max();
        let max_index = self
            .node_map
            .iter()
            .flatten()
            .position(|node| *node == *max_node.unwrap());

        (
            max_index.unwrap() / self.node_map[0].len(),
            max_index.unwrap() % self.node_map[0].len(),
        )
    }

    pub fn lowest_scored_node_index(&self) -> (usize, usize) {
        let min_node = self
            .node_map
            .iter()
            .flatten()
            .filter(|&n| n.score.is_some())
            .min();
        let min_index = self
            .node_map
            .iter()
            .flatten()
            .position(|node| *node == *min_node.unwrap());

        (
            min_index.unwrap() / self.node_map[0].len(),
            min_index.unwrap() % self.node_map[0].len(),
        )
    }

    /// Return (h, w)
    fn searched_target_node_index(&self) -> (usize, usize) {
        let min_node = self
            .node_map
            .iter()
            .flatten()
            .filter(|&n| n.score.is_some())
            .filter(|&n| n.status == NodeStatus::Open)
            .min();
        let min_index = self.node_map.iter().flatten().position(|node| {
            *node == *min_node.unwrap() && node.status == min_node.unwrap().status
        });

        (
            min_index.unwrap() / self.node_map[0].len(),
            min_index.unwrap() % self.node_map[0].len(),
        )
    }

    fn update_to_searched_node(
        &mut self,
        idy: usize,
        idx: usize,
        parent_node_real_cost: u64,
    ) -> bool {
        match self.node_map[idy][idx].status {
            NodeStatus::Start => {}
            NodeStatus::Goal => return true,
            NodeStatus::Open => {}
            NodeStatus::Closed => {}
            NodeStatus::Disable => {}
            NodeStatus::None => {
                self.node_map[idy][idx].status = NodeStatus::Open;
                self.node_map[idy][idx].score = Some(Score {
                    real: parent_node_real_cost + 1,
                    estimate: self.get_estimate_cost(idy, idx).unwrap(),
                })
            }
            NodeStatus::Path => {}
        }
        false
    }

    fn get_estimate_cost(&self, idy: usize, idx: usize) -> Result<u64, std::string::String> {
        match self.get_goal() {
            Some(goal) => {
                if goal.0.abs_diff(idy) > goal.1.abs_diff(idx) {
                    Ok(goal.0.abs_diff(idy) as u64)
                } else {
                    Ok(goal.1.abs_diff(idx) as u64)
                }
            }
            None => Err("Goal is not set".to_string()),
        }
    }

    /// For debug or something
    pub fn visualize_as_string(&self) -> std::string::String {
        let mut displayed = String::new();
        for node_raw in &self.node_map {
            for node in node_raw {
                let status = node.status.to_string();
                let color = node_status_as_color(&node.status);
                if let Some(_) = node.score.as_ref() {
                    displayed = format!("{}|{} {}\x1b[m", displayed, color, status);
                } else {
                    displayed = format!("{}|{} {}\x1b[m", displayed, color, status,);
                }
            }
            displayed = format!("{}|\n", displayed);
        }
        displayed
    }
}

impl std::fmt::Display for MapGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut displayed = String::new();
        for node_raw in &self.node_map {
            for node in node_raw {
                let status = node.status.to_string();
                let color = node_status_as_color(&node.status);
                if let Some(score) = node.score.as_ref() {
                    displayed = format!(
                        "{} |{} {} R:{:2},E:{:2} T:{:2}\x1b[m",
                        displayed,
                        color,
                        status,
                        score.real,
                        score.estimate,
                        score.get_score()
                    );
                } else {
                    displayed =
                        format!("{} |{} {} R:--,E:-- T:--\x1b[m", displayed, color, status,);
                }
            }
            displayed = format!("{} |\n", displayed);
        }
        write!(f, "{}", displayed)
    }
}

fn node_status_as_color(status: &NodeStatus) -> String {
    match status {
        NodeStatus::Start => "\x1b[38;5;1m".to_string(),
        NodeStatus::Goal => "\x1b[38;5;1m".to_string(),
        NodeStatus::Open => "\x1b[38;5;2m".to_string(),
        NodeStatus::Closed => "\x1b[38;5;3m".to_string(),
        NodeStatus::Disable => "\x1b[38;5;0m\x1b[47m".to_string(),
        NodeStatus::None => "\x1b[38;5;7m".to_string(),
        NodeStatus::Path => "\x1b[38;5;4m".to_string(),
    }
}
