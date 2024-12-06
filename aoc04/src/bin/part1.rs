use aoc04::{count_from_point, matrix::Matrix, read_input};

const TARGET_WORD: &str = "XMAS";

fn main() {
    let puzzle = read_input("data/input.txt").unwrap();

    let mut count = 0;
    for row in 0..puzzle.rows() {
        for col in 0..puzzle.cols() {
            let point_count = count_from_point(TARGET_WORD, &puzzle, row, col);

            // println!("count({}, {}) = {}", row, col, point_count);
            count += point_count;
        }
    }

    println!("total count = {}", count);
}
