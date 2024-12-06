use aoc02::{read_inputs, test_report};

fn main() {
    let file_path = "data/input.txt";
    // let file_path = "test-input.txt";

    let inputs = read_inputs(file_path).unwrap();
    let safe_reports: usize = inputs
        .iter()
        .map(|report| {
            let result = extensive_test(report);
            println!("{:?} => {}", report, result);

            match result {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("safe_reports: {}", safe_reports);
}

fn extensive_test(report: &Vec<i32>) -> bool {
    if test_report(report) {
        return true;
    }

    for index in 0..report.len() {
        let mut input_report = report.clone();
        input_report.remove(index);
        if test_report(&input_report) {
            return true;
        }
    }

    return false;
}
