use std::{
    fs,
    io::{self},
};

use matrix::Matrix;

pub mod matrix;

pub fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(filename)?;
    return Ok(contents.split('\n').map(|line| line.to_string()).collect());
}

pub fn count_from_point(word: &str, puzzle: &Vec<String>, row: usize, col: usize) -> usize {
    let mut count: usize = 0;

    let mut string = build_north(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    string = build_northeast(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    string = build_east(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    string = build_southeast(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    string = build_south(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    string = build_southwest(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    string = build_west(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    string = build_northwest(puzzle, row, col);
    if string.starts_with(word) {
        count += 1;
    }

    return count;
}

fn build_east(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    return puzzle[row][col..].to_string();
}

fn build_west(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    return puzzle[row][..(col + 1)].chars().rev().collect();
}

fn build_south(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    return puzzle[row..]
        .iter()
        .map(|string| string.chars().nth(col).unwrap())
        .collect();
}

fn build_north(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    return puzzle[..(row + 1)]
        .iter()
        .map(|string| string.chars().nth(col).unwrap())
        .rev()
        .collect();
}

fn build_southeast(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    let mut result = String::new();

    let mut x = col;
    let mut y = row;
    while y < puzzle.rows() && x < puzzle.cols() {
        let ch = puzzle.extract(y, x).unwrap();
        result.push(ch);

        x += 1;
        y += 1;
    }

    return result;
}

fn build_northeast(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    let mut result = String::new();

    let mut x = col;
    let mut j = row as i32;

    while j >= 0 && x < puzzle.cols() {
        let y = j as usize;
        let ch = puzzle.extract(y, x).unwrap();
        result.push(ch);

        x += 1;
        j -= 1;
    }

    return result;
}

fn build_southwest(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    let mut result = String::new();

    let mut i = col as i32;
    let mut y = row;
    while y < puzzle.rows() && i >= 0 {
        let x = i as usize;
        let ch = puzzle.extract(y, x).unwrap();
        result.push(ch);

        i -= 1;
        y += 1;
    }

    return result;
}

fn build_northwest(puzzle: &Vec<String>, row: usize, col: usize) -> String {
    let mut result = String::new();

    let mut i = col as i32;
    let mut j = row as i32;
    while j >= 0 && i >= 0 {
        let x = i as usize;
        let y = j as usize;

        let ch = puzzle.extract(y, x).unwrap();
        result.push(ch);

        j -= 1;
        i -= 1;
    }

    return result;
}
