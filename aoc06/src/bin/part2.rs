use aoc06::{find_loops, read_input, start_point};

const SOURCE: &str = "input";

fn main() {
    let filename = format!("data/{}/map.txt", SOURCE);
    let map = read_input(filename.as_str()).unwrap();

    let start = match start_point(&map) {
        Some(point) => point,
        None => {
            println!("Unable to find starting point");
            return;
        }
    };

    // let mut steps: Vec<(usize, Dir)> = vec![(start_idx, Dir::UP)];
    // let result = walk(&map, &mut start, Dir::UP, &mut steps);
    // println!("steps = {:?}", steps);

    let loops = find_loops(&map, &start);
    println!("{}", loops);
}
