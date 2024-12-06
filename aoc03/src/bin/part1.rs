use std::fs;

use regex::Regex;

fn main() {
    // let file_path = "input-example.txt";
    let file_path = "data/input.txt";

    let contents = fs::read_to_string(file_path).unwrap();
    let regex = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

    let mut value = 0;
    for cap in regex.captures_iter(&contents) {
        let first_str = cap.name("first").unwrap().as_str();
        let second_str = cap.name("second").unwrap().as_str();

        let first: i32 = first_str.parse().unwrap();
        let second: i32 = second_str.parse().unwrap();

        value += first * second;
    }

    println!("value: {}", value);
}
