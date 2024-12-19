use aoc09::{calculate_checksum, read_disk_map, sort_disk_map};

const SOURCE: &str = "input";

fn main() {
    let filename = format!("data/{}/disk_map.txt", SOURCE);

    let blocks = read_disk_map(&filename).unwrap();
    for block in blocks.iter() {
        print!("{}", block);
    }
    println!();

    let sorted = sort_disk_map(&blocks);
    for block in sorted.iter() {
        print!("{}", block);
    }
    println!();

    let checksum = calculate_checksum(&sorted);
    println!("checksum: {}", checksum);
}
