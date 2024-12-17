use std::{collections::HashMap, fs, io};

use point::Point;
use vector::Vector;

pub mod matrix;
pub mod point;
pub mod vector;

pub fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(filename)?;
    let lines: Vec<String> = contents.split('\n').map(|line| line.to_string()).collect();

    return Ok(lines);
}

pub fn make_hashmap(map: &[String]) -> HashMap<char, Vec<Point>> {
    let mut hashmap = HashMap::new();
    for (row, row_str) in map.iter().enumerate() {
        for (col, c) in row_str.chars().enumerate() {
            if !c.is_ascii_alphanumeric() {
                continue;
            }

            hashmap
                .entry(c)
                .or_insert_with(Vec::new)
                .push(Point { row, col });
        }
    }

    return hashmap;
}

pub fn calculate_antinode(p1: &Point, p2: &Point, bounds: &Point) -> Option<Point> {
    let u = Vector::from_point(p1);
    let v = Vector::from_point(p2);

    let antinode_loc = u.scalar_mul(2) - v;
    return antinode_loc.to_point(bounds);
}

pub fn break_list(list: &[Point], idx: usize) -> (&Point, Vec<&Point>) {
    let filtered = list
        .iter()
        .enumerate()
        .filter_map(|(i, val)| match i == idx {
            false => Some(val),
            true => None,
        })
        .collect();
    
    return (&list[idx], filtered);
}
