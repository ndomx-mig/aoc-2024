use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use dir::Dir;
use matrix::Matrix;
use point::Point;

mod dir;
mod matrix;
mod point;

pub fn read_map(filename: &str) -> io::Result<Vec<Vec<u8>>> {
    let contents = fs::read_to_string(filename)?;

    let mut rows: Vec<Vec<u8>> = Vec::new();
    for line in contents.split('\n') {
        let parsed: Vec<u8> = line.bytes().map(|b| b - 48).collect();
        rows.push(parsed);
    }

    return Ok(rows);
}

pub fn find_trail_heads(map: &[Vec<u8>]) -> Vec<Point> {
    let mut points = Vec::new();
    for (row_idx, row_val) in map.iter().enumerate() {
        for (col_idx, col_val) in row_val.iter().enumerate() {
            if *col_val == 0 {
                points.push(Point {
                    x: col_idx as i32,
                    y: row_idx as i32,
                });
            }
        }
    }

    return points;
}

pub fn find_trails(
    map: &[Vec<u8>],
    start: &Point,
    curr: Point,
    target: u8,
    success_map: &mut HashMap<u32, HashSet<u32>>,
) {
    let curr_value = match map.read_at(&curr) {
        Some(val) => *val,
        None => return,
    };

    if target != curr_value {
        return;
    }

    if target == 9 {
        let key = map.abs_idx(start);
        let value = map.abs_idx(&curr);
        success_map
            .entry(key)
            .or_insert(HashSet::new())
            .insert(value);

        return;
    }

    let next_target = target + 1;
    find_trails(
        map,
        start,
        curr.move_to(Dir::RIGHT),
        next_target,
        success_map,
    );

    find_trails(
        map,
        start,
        curr.move_to(Dir::DOWN),
        next_target,
        success_map,
    );

    find_trails(
        map,
        start,
        curr.move_to(Dir::LEFT),
        next_target,
        success_map,
    );

    find_trails(map, start, curr.move_to(Dir::UP), next_target, success_map);
}

pub fn find_trails2(map: &[Vec<u8>], curr: Point, target: u8) -> usize {
    let curr_value = match map.read_at(&curr) {
        Some(val) => *val,
        None => return 0,
    };

    if target != curr_value {
        return 0;
    }

    if target == 9 {
        return 1;
    }

    let mut rating: usize = 0;

    rating += find_trails2(map, curr.move_to(Dir::RIGHT), target + 1);
    rating += find_trails2(map, curr.move_to(Dir::DOWN), target + 1);
    rating += find_trails2(map, curr.move_to(Dir::LEFT), target + 1);
    rating += find_trails2(map, curr.move_to(Dir::UP), target + 1);

    return rating;
}
