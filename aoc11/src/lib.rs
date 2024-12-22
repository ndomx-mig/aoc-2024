use std::{fs, io};

pub mod iterator;

pub fn read_inputs(env: &str) -> io::Result<Vec<u64>> {
    let filename = format!("data/{}/inputs.txt", env);
    let contents = fs::read_to_string(filename)?;

    let inputs: Vec<u64> = contents
        .split(' ')
        .map(|str_val| str_val.parse().unwrap())
        .collect();

    return Ok(inputs);
}
