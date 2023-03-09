use a_star::AStar;

fn main() {
    let mut map = AStar::new_from_size(12, 6);
    map.set_start_and_goal(1, 0, 11, 5).unwrap();

    println!("{}", map);

    map.search();

    println!("{}", map);
}
