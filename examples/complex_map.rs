use a_star::AStar;

fn main() {
    let min = (-1.0, 1.0);
    let max = (1.0, 2.0);
    let resolution = 0.05;
    let mut map = AStar::new(min, max, resolution);

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

    map.set_start_and_goal((-0.8, 0.1), (0.05, 1.9)).unwrap();
    map.set_obstacle(&obstacle);

    println!("{}", map.visualize_as_string());

    map.search();

    println!("{}", map.visualize_as_string());
}
