use aoc05::{group_rules, is_update_valid, read_rules, read_updates};

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

    let updates = read_updates(&updates_filename).unwrap();
    let valid_updates: usize = updates
        .iter()
        .filter(|update| is_update_valid(update, &grouped))
        .map(|update| middle_element!(update))
        .sum();

    println!("result: {}", valid_updates);
}
