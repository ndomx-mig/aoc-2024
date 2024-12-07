use aoc05::{find_error, group_rules, is_update_valid, read_rules, read_updates, Rules};

const SOURCE: &str = "input";

macro_rules! middle_element {
    ($vec:ident) => {
        $vec[$vec.len() >> 1]
    };
}

fn main() {
    let rules_filename = format!("data/{}/rules.txt", SOURCE);
    let updates_filename = format!("data/{}/updates.txt", SOURCE);

    let rules = read_rules(&rules_filename).unwrap();
    let grouped = group_rules(rules);
    grouped
        .iter()
        .for_each(|entry| println!("first: {}, second: {:?}", entry.0, entry.1));

    let lines = read_updates(&updates_filename).unwrap();
    let result: usize = lines
        .iter()
        .filter(|line| !is_update_valid(line, &grouped))
        .map(|line| fix_error(line, &grouped))
        .map(|line| middle_element!(line))
        .sum();

    println!("result: {:?}", result);
}

fn fix_error(line: &Vec<usize>, rules: &Rules) -> Vec<usize> {
    let mut test_line = line.clone();
    print!("test: {:?} -> ", test_line);

    let mut valid = false;
    while !valid {
        match find_error(&test_line, rules) {
            Some(indices) => swap(test_line.as_mut(), indices),
            None => valid = true,
        }
    }

    println!("test: {:?}", test_line);

    return test_line;
}

fn swap(line: &mut Vec<usize>, indices: (usize, usize)) {
    let first = line[indices.0];
    let second = line[indices.1];

    line[indices.0] = second;
    line[indices.1] = first;
}
