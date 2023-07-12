use a_star::*;

fn main() {
    let result = a_star(start, goal, min_point, max_point, resolution, obstacle);
}

fn new_obstacle_map(min: (f64, f64), resolution: f64) -> Vec<(f64, f64)> {
    let mut obstacle = vec![];
    for i in 0..15 {
        obstacle.push((
            (26 - i) as f64 * resolution + min.0,
            (5 + i) as f64 * resolution + min.1,
        ));
        obstacle.push((
            (25 - i) as f64 * resolution + min.0,
            (5 + i) as f64 * resolution + min.1,
        ));
    }
    for i in 0..12 {
        obstacle.push((i as f64 * resolution + min.0, 10. * resolution + min.1));
    }

    obstacle
}
