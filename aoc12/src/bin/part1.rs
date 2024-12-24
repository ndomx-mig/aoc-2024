use aoc12::{read_map, read_regions};

fn main() {
    let filename = "giant";
    let map = read_map(&filename).unwrap();

    let regions = read_regions(&map);
    let price: usize = regions.into_iter().map(|region| {
        let area = region.area();
        let perimeter = region.perimeter(&map);
        return area * perimeter;
    }).sum();

    println!("price: {}", price);
}
