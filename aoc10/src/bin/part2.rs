use aoc10::{find_trail_heads, find_trails2, read_map};

const SOURCE: &str = "input";

fn main() {
    let filename = format!("data/{}/map.txt", SOURCE);

    let map = read_map(&filename).unwrap();
    let trail_heads = find_trail_heads(&map);

    let trail_count = trail_heads.len();
    let mut ratings: usize = 0;

    for (idx, &trail_head) in trail_heads.iter().enumerate() {
        println!("Looping over {:?} [{}/{}]", trail_head, idx, trail_count);

        let rating = find_trails2(&map, trail_head, 0);
        // println!("{}", rating);

        ratings += rating;
    }

    // for (idx, set) in success_map {
    //     println!("{}: {}", idx, set.len());
    // }

    println!("sum of ratings: {}", ratings);
}
