use aoc09::{calculate_checksum, defragment, read_disk_map};

const SOURCE: &str = "input";

fn main() {
    let filename = format!("data/{}/disk_map.txt", SOURCE);

    let blocks = read_disk_map(&filename).unwrap();
    let sorted = defragment(&blocks);

    let checksum = calculate_checksum(&sorted);
    println!("checksum: {}", checksum);
}
