use std::collections::HashSet;

use aoc08::{break_list, calculate_antinodes, make_hashmap, matrix::Matrix, read_input};

const SOURCE: &str = "input";

fn main() {
    let filename = format!("data/{}/map.txt", SOURCE);
    let matrix_map = read_input(&filename).unwrap();
    let bounds = matrix_map.max_idx();

    let hashed_map = make_hashmap(&matrix_map);
    let mut unique_locs: HashSet<usize> = HashSet::new();

    hashed_map.iter().for_each(|(_, val)| {
        for idx in 0..val.len() {
            let (curr, others) = break_list(&val, idx);
            for point in others {
                let locs = calculate_antinodes(curr, point, &bounds);
                locs.iter().for_each(|loc| {
                    unique_locs.insert(matrix_map.abs_idx(loc));
                });
            }
        }
    });

    println!("{}", unique_locs.len());
}
