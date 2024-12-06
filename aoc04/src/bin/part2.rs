use aoc04::{matrix::Matrix, read_input};

const VALID_KEYS: [&str; 4] = ["MMSS", "MSMS", "SSMM", "SMSM"];

fn main() {
    // let puzzle = read_input("data/input.example.txt").unwrap();
    let puzzle = read_input("data/input.txt").unwrap();

    let mut count = 0;
    for row in 1..puzzle.max_row_index() {
        for col in 1..puzzle.max_col_index() {
            if !puzzle.compare_element(row, col, 'A') {
                continue;
            }

            if is_valid_x(&puzzle, row, col) {
                count += 1;
            }
        }
    }

    println!("total count = {}", count);
}

fn is_valid_x(puzzle: &Vec<String>, row: usize, col: usize) -> bool {
    let diag: String = match puzzle.window_diagonals(row, col) {
        Some(val) => val.iter().collect(),
        None => return false,
    };

    return VALID_KEYS.contains(&diag.as_str());
}
