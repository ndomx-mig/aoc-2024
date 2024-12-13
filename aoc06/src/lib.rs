use std::{collections::HashSet, fs, io};

use dir::Dir;
use matrix::Matrix;
use point::Point;
use walk_result::WalkResult;

pub mod dir;
pub mod matrix;
mod point;
mod walk_result;

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

pub fn find_loops(map: &Vec<String>, start: &Point) -> usize {
    let mut loop_count = 0;

    for idx in 0..map.rows() {
        for jdx in 0..map.cols() {
            println!("row: {}, col: {}", idx, jdx);
            let point = Point { x: jdx, y: idx };
            if map.compare_element(&point, '#') {
                continue;
            }

            if point.x == start.x && point.y == start.y {
                continue;
            }

            let modified = add_obstacle(map, point);
            loop_count += match walk_map(&modified, start) {
                Some(res) => res,
                None => panic!(),
            }
        }
    }

    return loop_count;
}

fn add_obstacle(map: &Vec<String>, point: Point) -> Vec<String> {
    let mut map_copy = map.clone();
    let mut row_copy = map[point.y].clone();

    row_copy.replace_range(point.x..point.x + 1, "#");
    map_copy[point.y] = row_copy;

    return map_copy;
}

fn walk_map(map: &Vec<String>, start: &Point) -> Option<usize> {
    let mut result: WalkResult;

    let start_idx = map.abs_idx(start);
    let mut steps = vec![(start_idx, Dir::UP)];
    let mut point = start.clone();
    loop {
        result = walk_line(map, &mut point, Dir::UP, &mut steps);
        if result != WalkResult::OBSTACLE {
            break;
        }

        result = walk_line(map, &mut point, Dir::RIGHT, &mut steps);
        if result != WalkResult::OBSTACLE {
            break;
        }

        result = walk_line(map, &mut point, Dir::DOWN, &mut steps);
        if result != WalkResult::OBSTACLE {
            break;
        }

        result = walk_line(map, &mut point, Dir::LEFT, &mut steps);
        if result != WalkResult::OBSTACLE {
            break;
        }
    }

    return match result {
        WalkResult::EXIT => Some(0),
        WalkResult::LOOP => Some(1),
        _ => None,
    };
}

fn walk_line(
    map: &Vec<String>,
    start: &mut Point,
    dir: Dir,
    steps: &mut Vec<(usize, Dir)>,
) -> WalkResult {
    let result: WalkResult;
    let mut movement: (Box<dyn FnMut(&mut Point)>, Box<dyn FnMut(&mut Point)>) = match dir {
        Dir::UP => (
            Box::new(|point: &mut Point| point.move_up()),
            Box::new(|point: &mut Point| point.move_down()),
        ),
        Dir::RIGHT => (
            Box::new(|point: &mut Point| point.move_right()),
            Box::new(|point: &mut Point| point.move_left()),
        ),
        Dir::DOWN => (
            Box::new(|point: &mut Point| point.move_down()),
            Box::new(|point: &mut Point| point.move_up()),
        ),
        Dir::LEFT => (
            Box::new(|point: &mut Point| point.move_left()),
            Box::new(|point: &mut Point| point.move_right()),
        ),
    };

    // println!("start walking {:?} from point {:?}", dir, start);

    loop {
        movement.0(start);
        if !map.valid_point(start) {
            result = WalkResult::EXIT;
            break;
        }

        if map.compare_element(start, '#') {
            result = WalkResult::OBSTACLE;
            break;
        }

        let abs_idx = map.abs_idx(start);
        let target = (abs_idx, dir);
        if steps.contains(&target) {
            result = WalkResult::LOOP;
            break;
        } else {
            steps.push(target);
        }
    }

    movement.1(start);
    return result;
}

fn move_up(map: &Vec<String>, start: &mut Point, steps: &mut HashSet<usize>) -> bool {
    // println!("start moving up from point {:?}", start);

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
