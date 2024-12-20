use std::collections::{HashMap, HashSet};

use aoc10::{find_trail_heads, find_trails, read_map};

const SOURCE: &str = "input";

fn main() {
    let filename = format!("data/{}/map.txt", SOURCE);

    let map = read_map(&filename).unwrap();
    let trail_heads = find_trail_heads(&map);

    let mut success_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let trail_count = trail_heads.len();
    for (idx, &trail_head) in trail_heads.iter().enumerate() {
        println!("Looping over {:?} [{}/{}]", trail_head, idx, trail_count);

        let start_copy = trail_head;
        find_trails(&map, &trail_head, start_copy, 0, &mut success_map);
    }

    let score_sum: usize = success_map.values().map(|set| set.len()).sum();
    println!("sum of scores: {}", score_sum);
}
