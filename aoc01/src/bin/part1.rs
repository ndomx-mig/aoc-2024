use std::error::Error;

use aoc01::read_inputs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/inputs.txt";
    let values = read_inputs(file_path)?;

    // println!("{:?}", values);

    let distances = calculate_distances(values);
    // println!("{:?}", distances);

    let distance: i32 = distances.iter().sum();
    println!("distance: {}", distance);

    return Ok(());
}

pub fn calculate_distances(values: Vec<(i32, i32)>) -> Vec<i32> {
    let mut pairs: (Vec<i32>, Vec<i32>) = values.into_iter().unzip();
    pairs.0.sort();
    pairs.1.sort();

    return pairs
        .0
        .iter()
        .zip(pairs.1.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();
}

