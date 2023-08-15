use grid_map::{Cell, Grid, GridMap};

use crate::{Error, Node};

const SCORE_INCREASE_VALUE: f64 = 1.0;
const QUIT_COUNT_LIMIT: usize = 100;

pub fn a_star(
    map: &mut GridMap<Node>,
    start: &[f64; 2],
    goal: &[f64; 2],
) -> Result<Vec<Vec<f64>>, Error> {
    let mut path = vec![];

    println!("INITIAL MAP: {:?}", map);

    let mut open_node_table = vec![];

    let start_position = map.to_grid(start[0], start[1]).unwrap();
    let goal_position = map.to_grid(goal[0], goal[1]).unwrap();

    println!("START: {:?}", start_position);
    println!("GOAL: {:?}", goal_position);

    let mut target_cell = start_position;

    open_node_table.push(start_position);

    let mut count = 0;

    loop {
        let distance = map
            .cell(&target_cell)
            .unwrap()
            .value()
            .unwrap_or(&Node::default())
            .total_score();

        let neighbors = target_cell.neighbors8();

        if neighbors.iter().any(|n| *n == goal_position) {
            break;
        }

        for neighbor in neighbors.iter() {
            open_node_table.push(neighbor.clone());
            match map.cell_mut(neighbor) {
                Some(c) => {
                    *c = Cell::Value(Node {
                        heuristic: get_heuristic_score(goal_position, *neighbor) as f64,
                        real: distance + SCORE_INCREASE_VALUE,
                    });
                }
                None => {}
            };
        }

        count += 1;
        let (idx, min_heuristic_score_node) =
            get_minimum_heuristic_score_node(map, &open_node_table).unwrap();

        target_cell = min_heuristic_score_node;
        open_node_table.remove(idx);

        if count > QUIT_COUNT_LIMIT {
            break;
        }
    }

    println!("RESULT MAP: {:?}", map);

    Ok(path)
}

pub fn parse_grid_map_u8_to_node(map: &GridMap<u8>) -> GridMap<Node> {
    let cells = map
        .copy_without_value()
        .cells()
        .iter()
        .map(|c| match c {
            Cell::Unknown => Cell::Unknown,
            Cell::Obstacle => Cell::Obstacle,
            _ => Cell::Uninitialized,
        })
        .collect::<Vec<Cell<Node>>>();

    let mut cost_map = GridMap::<Node>::new(
        map.min_point().clone(),
        map.max_point().clone(),
        map.resolution(),
    );
    *cost_map.cells_mut() = cells;

    cost_map
}

/// Manhattan distance
fn get_heuristic_score(goal: Grid, current: Grid) -> usize {
    goal.x.abs_diff(current.x).max(goal.y.abs_diff(current.y))
}

fn get_minimum_heuristic_score_node(
    map: &mut GridMap<Node>,
    grids: &[Grid],
) -> Option<(usize, Grid)> {
    if grids.is_empty() {
        return None;
    }

    let mut min_heuristic_score_node = Node {
        heuristic: f64::MAX,
        real: 0.,
    };
    let mut idx = usize::MAX;

    for (i, grid) in grids.iter().enumerate() {
        let score = match map.cell(grid) {
            Some(c) => {
                let node = match c.value() {
                    Some(n) => n,
                    None => &Node {
                        heuristic: f64::MAX,
                        real: 0.,
                    },
                };
                node
            }
            None => continue,
        };

        if score.heuristic < min_heuristic_score_node.heuristic {
            min_heuristic_score_node = *score;
            idx = i;
        }
    }

    Some((idx, grids[idx]))
}

#[cfg(test)]
mod test {
    use grid_map::*;

    use super::*;

    #[test]
    fn test_get_estimate_score() {
        let grid_a = Grid::new(10, 12);
        let grid_b = Grid::new(8, 15);

        let score = get_heuristic_score(grid_a, grid_b);

        assert_eq!(score, 3);
    }

    #[test]
    fn test_a_star() {
        let mut map = parse_grid_map_u8_to_node(&new_sample_map());

        let start = [-0.08, -0.08];
        let goal = [0.1, 0.1];

        let path = a_star(&mut map, &start, &goal).unwrap();

        println!("PATH: {:?}", path);
    }

    fn new_sample_map() -> GridMap<u8> {
        let mut map =
            grid_map::GridMap::<u8>::new(Position::new(-0.1, -0.1), Position::new(0.1, 0.1), 0.05);
        map.set_obstacle(&Grid::new(2, 3));

        map
    }
}
