use aoc06::{count_steps, read_input, start_point};

const SOURCE: &str = "example";

fn main() {
    let filename = format!("data/{}/map.txt", SOURCE);
    let map = read_input(filename.as_str()).unwrap();

    let mut start = match start_point(&map) {
        Some(point) => point,
        None => {
            println!("Unable to find starting point");
            return;
        }
    };

    let steps = count_steps(&map, &mut start);
    println!("steps = {}", steps);
}
