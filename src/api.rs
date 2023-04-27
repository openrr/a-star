use crate::map_grid::MapGrid;

pub fn a_star(
    start: (f64, f64),
    goal: (f64, f64),
    min_point: (f64, f64),
    max_point: (f64, f64),
    resolution: f64,
    obstacle: &[(f64, f64)],
) -> Result<Vec<(f64, f64)>, String> {
    let mut map = MapGrid::new(min_point, max_point, resolution);

    map.set_start_and_goal(start, goal).unwrap();
    map.set_obstacle(obstacle);

    map.search();

    let path = map.get_path();

    Ok(path)
}
