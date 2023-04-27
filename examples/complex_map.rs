use a_star::AStar;

fn main() {
    let mut map = AStar::new_from_size(30, 30);

    let mut obstacle = vec![];
    for i in 0..15 {
        obstacle.push((25 - i, 5 + i));
        obstacle.push((26 - i, 5 + i));
    }
    for i in 0..12 {
        obstacle.push((i, 10));
    }
    for i in 0..10 {
        obstacle.push((20 + i, 23));
    }

    map.set_start_and_goal(1, 0, 28, 25).unwrap();
    map.set_obstacle(&obstacle);

    println!("{}", map.visualize_as_string());

    map.search();

    println!("{}", map.visualize_as_string());
}
