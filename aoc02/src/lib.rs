use std::{error::Error, fs::File};

use csv::{ReaderBuilder, StringRecord};

pub fn read_inputs(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut values = vec![];

    for result in reader.records() {
        let record = result?;

        let parsed = parse_line(record);
        values.push(parsed);
    }

    return Ok(values);
}

fn parse_line(line: StringRecord) -> Vec<i32> {
    let values = line.as_slice().split_whitespace();
    return values.into_iter().map(|s| s.parse().unwrap()).collect();
}

pub fn dif_test(a: i32, b: i32) -> bool {
    let dif = (a - b).abs();
    return 1 <= dif && dif <= 3;
}

pub fn test_report(levels: &Vec<i32>) -> bool {
    let dec_test = |a: i32, b: i32| {
        return (a > b) && dif_test(a, b);
    };

    let inc_test = |a: i32, b: i32| {
        return (a < b) && dif_test(a, b);
    };

    let test: fn(i32, i32) -> bool;
    match levels[0] - levels[1] {
        x if x > 0 => test = dec_test,
        x if x < 0 => test = inc_test,
        _ => {
            println!("aborting");
            return false;
        }
    };

    for pair in levels.windows(2) {
        let first = pair.first().unwrap();
        let second = pair.get(1).unwrap();
        if !check_levels(*first, *second, test) {
            return false;
        }
    }

    return true;
}

fn check_levels(l1: i32, l2: i32, test: fn(i32, i32) -> bool) -> bool {
    return test(l1, l2);
}
