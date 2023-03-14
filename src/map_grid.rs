use crate::node::Node;
use crate::node_status::NodeStatus;
use crate::score::Score;

pub(crate) type MapGridInner = Vec<Vec<Node>>;

#[derive(Clone, Debug, Default)]
pub struct MapGrid(MapGridInner);

impl MapGrid {
    pub fn new_from_size(height: usize, width: usize) -> Self {
        let inner: MapGridInner = vec![vec![Node::default(); width]; height];
        Self(inner)
    }

    pub fn set_start_and_goal(
        &mut self,
        s_idy: usize,
        s_idx: usize,
        g_idy: usize,
        g_idx: usize,
    ) -> Result<(), std::string::String> {
        self.set_start(s_idy, s_idx)?;
        self.set_goal(g_idy, g_idx)?;
        self.init_start_and_goal()?;
        Ok(())
    }

    fn set_start(&mut self, idy: usize, idx: usize) -> Result<(), std::string::String> {
        // TODO: Error handling
        if self.is_set_start() {
            Err("Start has already set.".to_string())
        } else {
            // TODO: Check index is valid or not.
            self.0[idy][idx] = Node {
                status: NodeStatus::Start,
                score: Some(Score {
                    real: 0,
                    estimate: 0,
                }),
            };
            Ok(())
        }
    }

    fn set_goal(&mut self, idy: usize, idx: usize) -> Result<(), std::string::String> {
        // TODO: Error handling
        if self.is_set_goal() {
            Err("Goal has already set.".to_string())
        } else {
            // TODO: Check index is valid or not.
            self.0[idy][idx].status = NodeStatus::Goal;
            Ok(())
        }
    }

    pub fn set_wall(&mut self, walls: &[(usize, usize)]) {
        for &wall in walls {
            self.0[wall.0][wall.1] = Node {
                status: NodeStatus::Disable,
                score: None,
            };
        }
    }

    pub fn get_start(&self) -> Option<(usize, usize)> {
        self.0
            .iter()
            .flatten()
            .position(|node| node.status == NodeStatus::Start)
            .map(|id| (id / self.0[0].len(), id % self.0[0].len()))
    }

    pub fn get_goal(&self) -> Option<(usize, usize)> {
        self.0
            .iter()
            .flatten()
            .position(|node| node.status == NodeStatus::Goal)
            .map(|id| (id / self.0[0].len(), id % self.0[0].len()))
    }

    pub fn is_set_start(&self) -> bool {
        self.0
            .iter()
            .flatten()
            .any(|node| node.status == NodeStatus::Start)
    }

    pub fn is_set_goal(&self) -> bool {
        self.0
            .iter()
            .flatten()
            .any(|node| node.status == NodeStatus::Goal)
    }

    pub fn get_trajectory(&self) -> Vec<(usize, usize)> {
        todo!()
    }

    pub fn search(&mut self) {
        // TODO: Check start and goal are set.
        let mut is_reached_goal = false;
        let height = self.0.len();
        let width = self.0[0].len();

        while !is_reached_goal {
            let parent_node_index;
            if self.is_only_start_node_be_searched() {
                parent_node_index = self.get_start().unwrap();
            } else {
                parent_node_index = self.searched_target_node_index();
                self.0[parent_node_index.0][parent_node_index.1].status = NodeStatus::Closed;
            }
            let parent_node_real_cost = self.0[parent_node_index.0][parent_node_index.1]
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
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("{}", self.visualize_as_string());
        }
    }

    fn init_start_and_goal(&mut self) -> Result<(), std::string::String> {
        if let Some(start) = self.get_start() {
            self.0[start.0][start.1].score = Some(Score {
                real: 0,
                estimate: self.get_estimate_cost(start.0, start.1).unwrap(),
            });
            Ok(())
        } else {
            Err("Start is not set".to_string())
        }
    }

    fn is_only_start_node_be_searched(&self) -> bool {
        self.0
            .iter()
            .flatten()
            .all(|node| node.status != NodeStatus::Open)
            && self.is_set_start()
    }

    pub fn highest_scored_node_index(&self) -> (usize, usize) {
        let max_node = self.0.iter().flatten().filter(|&n| n.score.is_some()).max();
        let max_index = self
            .0
            .iter()
            .flatten()
            .position(|node| *node == *max_node.unwrap());

        (
            max_index.unwrap() / self.0[0].len(),
            max_index.unwrap() % self.0[0].len(),
        )
    }

    pub fn lowest_scored_node_index(&self) -> (usize, usize) {
        let min_node = self.0.iter().flatten().filter(|&n| n.score.is_some()).min();
        let min_index = self
            .0
            .iter()
            .flatten()
            .position(|node| *node == *min_node.unwrap());

        (
            min_index.unwrap() / self.0[0].len(),
            min_index.unwrap() % self.0[0].len(),
        )
    }

    /// Return (h, w)
    fn searched_target_node_index(&self) -> (usize, usize) {
        let min_node = self
            .0
            .iter()
            .flatten()
            .filter(|&n| n.score.is_some())
            .filter(|&n| n.status == NodeStatus::Open)
            .min();
        let min_index = self.0.iter().flatten().position(|node| {
            *node == *min_node.unwrap() && node.status == min_node.unwrap().status
        });

        (
            min_index.unwrap() / self.0[0].len(),
            min_index.unwrap() % self.0[0].len(),
        )
    }

    fn update_to_searched_node(
        &mut self,
        idy: usize,
        idx: usize,
        parent_node_real_cost: u64,
    ) -> bool {
        match self.0[idy][idx].status {
            NodeStatus::Start => {}
            NodeStatus::Goal => return true,
            NodeStatus::Open => {}
            NodeStatus::Closed => {}
            NodeStatus::Disable => {}
            NodeStatus::None => {
                self.0[idy][idx].status = NodeStatus::Open;
                self.0[idy][idx].score = Some(Score {
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
        for node_raw in &self.0 {
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
        for node_raw in &self.0 {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_5x6_grid() {
        let mut map = MapGrid::new_from_size(5, 6);
        map.set_start_and_goal(1, 0, 1, 5).unwrap();

        assert!(map.is_set_start());
        assert!(map.is_set_goal());

        let start = map.get_start().unwrap();
        let goal = map.get_goal().unwrap();

        assert_eq!(start.0, 1);
        assert_eq!(start.1, 0);
        assert_eq!(goal.0, 1);
        assert_eq!(goal.1, 5);

        assert_eq!(map.0[start.0][start.1].score.as_ref().unwrap().estimate, 5);

        println!("{}", map);

        map.search();

        println!("{}", map);
    }

    #[test]
    fn test_get_foo() {
        let mut map = MapGrid::new_from_size(4, 4);
        let is_set_start = map.is_set_start();
        let is_set_goal = map.is_set_goal();
        let start = map.get_start();
        let goal = map.get_goal();
        assert!(!is_set_start);
        assert!(!is_set_goal);
        assert!(start.is_none());
        assert!(goal.is_none());

        map.set_start_and_goal(0, 1, 2, 3).unwrap();
        let is_set_start = map.is_set_start();
        let is_set_goal = map.is_set_goal();
        let start = map.get_start().unwrap();
        let goal = map.get_goal().unwrap();
        assert!(is_set_start);
        assert!(is_set_goal);
        assert_eq!(start, (0, 1));
        assert_eq!(goal, (2, 3));
    }

    #[test]
    fn test_get_target_node_index() {
        let mut map = MapGrid::new_from_size(12, 5);

        map.0[3][2] = Node {
            status: NodeStatus::Open,
            score: Some(Score {
                real: 3,
                estimate: 3,
            }),
        };
        map.0[8][4] = Node {
            status: NodeStatus::Open,
            score: Some(Score {
                real: 4,
                estimate: 2,
            }),
        };
        map.0[9][1] = Node {
            status: NodeStatus::Open,
            score: Some(Score {
                real: 3,
                estimate: 2,
            }),
        };
        map.0[10][3] = Node {
            status: NodeStatus::Closed,
            score: Some(Score {
                real: 1,
                estimate: 1,
            }),
        };
        map.0[11][4] = Node {
            status: NodeStatus::Start,
            score: Some(Score {
                real: 9,
                estimate: 2,
            }),
        };

        let highest_scored_node_index = map.highest_scored_node_index();
        let lowest_scored_node_index = map.lowest_scored_node_index();
        let searched_target_node_index = map.searched_target_node_index();

        assert_eq!(highest_scored_node_index, (11, 4));
        assert_eq!(lowest_scored_node_index, (10, 3));
        assert_eq!(searched_target_node_index, (9, 1));
    }

    #[test]
    fn test_update_to_searched_node() {
        let mut map = MapGrid::new_from_size(3, 3);
        map.set_start_and_goal(0, 0, 2, 2).unwrap();
        map.update_to_searched_node(1, 2, 10);

        let node = &map.0[1][2];
        let status = &node.status;
        let score = &node.score.as_ref().unwrap();
        assert_eq!(*status, NodeStatus::Open);
        assert_eq!(score.real, 11);
        assert_eq!(score.estimate, 1);
    }

    #[test]
    fn test_get_estimate_score() {
        let mut map = MapGrid::new_from_size(12, 5);
        map.set_start_and_goal(1, 2, 11, 3).unwrap();

        let estimate_score = map.get_estimate_cost(2, 1).unwrap();
        assert_eq!(estimate_score, 9);
    }
}
