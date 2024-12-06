use std::{error::Error, fs::File};

use csv::{ReaderBuilder, StringRecord};

pub fn read_inputs(file_path: &str) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut values = vec![];

    for result in reader.records() {
        let record = result?;

        let parsed = parse_line(record);
        values.push(parsed);
    }

    return Ok(values);
}

pub fn parse_line(line: StringRecord) -> (i32, i32) {
    let mut values = line.as_slice().split_whitespace();

    let first: i32 = values.next().unwrap().parse().unwrap();
    let second: i32 = values.next().unwrap().parse().unwrap();

    return (first, second);
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