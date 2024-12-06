use std::error::Error;

use aoc01::read_inputs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/inputs.txt";
    let values = read_inputs(file_path)?;

    let (left, right) = split_into_vec(values);

    let similarity_score = calculate_similarity(left, right);
    println!("similarity score: {}", similarity_score);

    return Ok(());
}

fn calculate_similarity(left: Vec<i32>, right: Vec<i32>) -> usize {
    left.iter().map(|x| item_score(x, &right)).sum()
}

fn split_into_vec(values: Vec<(i32, i32)>) -> (Vec<i32>, Vec<i32>) {
    return values.into_iter().unzip();
}

fn item_score(item: &i32, values: &Vec<i32>) -> usize {
    let appearances = values.into_iter().filter(|x| item.eq(x)).count();
    let score = appearances * (*item as usize);

    return score;
}
