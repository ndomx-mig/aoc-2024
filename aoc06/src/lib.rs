use std::{collections::HashSet, fs, io};

use matrix::Matrix;
use point::Point;

mod matrix;
mod point;

pub fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(filename)?;
    return Ok(contents.split('\n').map(|line| line.to_string()).collect());
}

pub fn count_steps(map: &Vec<String>, start: &mut Point) -> usize {
    let mut steps: HashSet<usize> = HashSet::new();
    let mut continue_walking: bool;

    loop {
        continue_walking = move_up(map, start, &mut steps);
        if !continue_walking {
            break;
        }

        continue_walking = move_right(map, start, &mut steps);
        if !continue_walking {
            break;
        }

        continue_walking = move_down(map, start, &mut steps);
        if !continue_walking {
            break;
        }

        continue_walking = move_left(map, start, &mut steps);
        if !continue_walking {
            break;
        }
    }

    return steps.len();
}

pub fn start_point(map: &Vec<String>) -> Option<Point> {
    for (jdx, row) in map.iter().enumerate() {
        match row.find('^') {
            Some(idx) => return Some(Point { x: idx, y: jdx }),
            None => continue,
        }
    }

    return None;
}

fn move_up(map: &Vec<String>, start: &mut Point, steps: &mut HashSet<usize>) -> bool {
    println!("start moving up from point {:?}", start);

    let mut result = true;
    loop {
        start.move_up();
        if !map.valid_point(start) {
            result = false;
            break;
        }

        if !map.compare_element(&start, '#') {
            steps.insert(map.abs_idx(&start));
        } else {
            break;
        }
    }

    start.move_down();
    return result;
}

fn move_right(map: &Vec<String>, start: &mut Point, steps: &mut HashSet<usize>) -> bool {
    println!("start moving right from point {:?}", start);

    let mut result = true;
    loop {
        start.move_right();
        if !map.valid_point(start) {
            result = false;
            break;
        }

        if !map.compare_element(&start, '#') {
            steps.insert(map.abs_idx(&start));
        } else {
            break;
        }
    }

    start.move_left();
    return result;
}

fn move_down(map: &Vec<String>, start: &mut Point, steps: &mut HashSet<usize>) -> bool {
    println!("start moving down from point {:?}", start);

    let mut result = true;
    loop {
        start.move_down();
        if !map.valid_point(start) {
            result = false;
            break;
        }

        if !map.compare_element(&start, '#') {
            steps.insert(map.abs_idx(&start));
        } else {
            break;
        }
    }

    start.move_up();
    return result;
}

fn move_left(map: &Vec<String>, start: &mut Point, steps: &mut HashSet<usize>) -> bool {
    println!("start moving left from point {:?}", start);

    let mut result = true;
    loop {
        start.move_left();
        if !map.valid_point(start) {
            result = false;
            break;
        }

        if !map.compare_element(&start, '#') {
            steps.insert(map.abs_idx(&start));
        } else {
            break;
        }
    }

    start.move_right();
    return result;
}
