use std::{collections::HashSet, fs, io};

use dir::Dir;
use matrix::Matrix;
use point::Point;
use region::Region;

mod dir;
mod matrix;
mod point;
mod region;

pub fn read_map(filename: &str) -> io::Result<Vec<String>> {
    let filepath = format!("data/{}.txt", filename);
    let contents = fs::read_to_string(filepath)?;

    let map: Vec<String> = contents
        .split('\n')
        .map(|line| String::from(line))
        .collect();

    return Ok(map);
}

pub fn read_regions(map: &[String]) -> Vec<Region> {
    let rows = map.rows();
    let cols = map.cols();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let point = Point { row, col };
            if visited[row][col] {
                continue;
            }

            let region = read_region(map, &mut visited, &point);
            regions.push(region);
        }
    }

    return regions;
}

fn read_region(map: &[String], visited: &mut [Vec<bool>], start: &Point) -> Region {
    let mut points: HashSet<usize> = HashSet::new();
    let target = map.read_at(start).unwrap();
    walk_region(map, visited, *start, &mut points, target);

    return Region {
        points,
        start: *start,
        id: target,
    };
}

fn walk_region(
    map: &[String],
    visited: &mut [Vec<bool>],
    start: Point,
    points: &mut HashSet<usize>,
    target: char,
) {
    visited[start.row][start.col] = true;
    let idx = map.abs_idx(&start);
    points.insert(idx);

    if let Some(up) = start.move_to(Dir::UP, &map.bounds()) {
        if !visited[up.row][up.col] {
            if visit_point(map, &up, target) {
                walk_region(map, visited, up, points, target);
            }
        }
    }

    if let Some(down) = start.move_to(Dir::DOWN, &map.bounds()) {
        if !visited[down.row][down.col] {
            if visit_point(map, &down, target) {
                walk_region(map, visited, down, points, target);
            }
        }
    }

    if let Some(left) = start.move_to(Dir::LEFT, &map.bounds()) {
        if !visited[left.row][left.col] {
            if visit_point(map, &left, target) {
                walk_region(map, visited, left, points, target);
            }
        }
    }

    if let Some(right) = start.move_to(Dir::RIGHT, &map.bounds()) {
        if !visited[right.row][right.col] {
            if visit_point(map, &right, target) {
                walk_region(map, visited, right, points, target);
            }
        }
    }
}

fn visit_point(map: &[String], point: &Point, target: char) -> bool {
    return match map.read_at(point) {
        Some(c) => c == target,
        None => false,
    };
}
