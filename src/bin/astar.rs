use a_star::{parse_grid_map_u8_to_node, Node};
use grid_map::*;

fn new_sample_map() -> GridMap<u8> {
    let mut map =
        grid_map::GridMap::<u8>::new(Position::new(-2.05, -2.05), Position::new(6.05, 2.05), 0.05);
    for i in 20..100 {
        for j in 10..14 {
            map.set_obstacle(&Grid::new(i + 20, j)).unwrap();
        }
        for j in 40..60 {
            map.set_obstacle(&Grid::new(i, j)).unwrap();
        }
    }
    map
}

fn main() {
    let map = parse_grid_map_u8_to_node(&new_sample_map());
}
