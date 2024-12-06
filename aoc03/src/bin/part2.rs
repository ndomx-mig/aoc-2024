use std::fs;

use regex::Regex;

const MAIN_REGEX: &str = r"mul\((?<first>\d+),(?<second>\d+)\)";

fn main() {
    // let file_path = "input-example.txt";
    let file_path = "data/input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let do_regex = r"do\(\)";
    let dont_regex = r"don't\(\)";
    let combined_pattern = format!("({})|({})|({})", MAIN_REGEX, do_regex, dont_regex);

    let regex = Regex::new(&combined_pattern).unwrap();
    let matches: Vec<&str> = regex.find_iter(&contents).map(|map| map.as_str()).collect();

    let mut keep: Vec<&str> = vec![];

    let mut allow = true;
    for mat in matches.clone().iter() {
        if mat.starts_with("do()") {
            allow = true;
        } else if mat.starts_with("don") {
            allow = false;
        } else if mat.starts_with("mul") && allow {
            keep.push(mat);
        }
    }

    let mut value = 0;
    for ins in keep {
        value += parse_single(ins);
    }

    println!("value: {}", value);
}

fn parse_single(instruction: &str) -> i32 {
    let regex = Regex::new(MAIN_REGEX).unwrap();
    let cap = regex.captures(instruction).unwrap();

    let first_str = cap.name("first").unwrap().as_str();
    let second_str = cap.name("second").unwrap().as_str();

    let first: i32 = first_str.parse().unwrap();
    let second: i32 = second_str.parse().unwrap();

    return first * second;
}
