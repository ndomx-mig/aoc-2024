use aoc02::{read_inputs, test_report};

fn main() {
    let file_path = "data/input.txt";
    // let file_path = "test-input.txt";

    let inputs = read_inputs(file_path).unwrap();
    let safe_reports: usize = inputs
        .iter()
        .map(|report| match test_report(report) {
            true => 1,
            false => 0,
        })
        .sum();

    println!("safe reports: {}", safe_reports);
}
