use a_star::AStar;

fn main() {    let min = (-1.0, 1.0);
    let max = (1.0, 2.0);
    let resolution = 0.05;
    let mut map = AStar::new(min, max, resolution);

    map.set_start_and_goal((-0.8, 0.1), (0.05, 1.9)).unwrap();

    println!("{}", map);

    map.search();

    println!("{}", map);
}
