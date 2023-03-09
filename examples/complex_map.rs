use a_star::AStar;

fn main() {
    let mut map = AStar::new_from_size(30, 30);

    let mut walls = vec![];
    for i in 0..15 {
        walls.push((25 - i, 5 + i));
        walls.push((26 - i, 5 + i));
    }
    for i in 0..12 {
        walls.push((i, 10));
    }
    for i in 0..10 {
        walls.push((20 + i, 23));
    }

    map.set_start_and_goal(1, 0, 28, 25).unwrap();
    map.set_wall(&walls);

    println!("{}", map.visualize_as_string());

    map.search();

    println!("{}", map.visualize_as_string());
}
