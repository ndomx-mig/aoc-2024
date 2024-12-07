use std::{collections::HashMap, fs::File};

use csv::ReaderBuilder;
use rule::Rule;

mod rule;

pub type Rules = HashMap<usize, Vec<usize>>;

pub fn read_rules(filename: &str) -> Result<Vec<Rule>, csv::Error> {
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    return reader.deserialize().collect();
}

pub fn read_updates(filename: &str) -> Result<Vec<Vec<usize>>, csv::Error> {
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(file);

    let mut updates: Vec<Vec<usize>> = Vec::new();
    for record in reader.records() {
        let parsed_row: Vec<usize> = record?
            .iter()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        updates.push(parsed_row);
    }

    return Ok(updates);
}

pub fn group_rules(rules: Vec<Rule>) -> Rules {
    let mut map = HashMap::new();

    for rule in rules {
        map.entry(rule.first)
            .or_insert_with(Vec::new)
            .push(rule.second);
    }

    return map;
}

pub fn is_update_valid(update: &Vec<usize>, rules: &Rules) -> bool {
    let size = update.len() - 1;
    for (idx, &entry) in update.iter().rev().enumerate() {
        let filtered = &update[0..(size - idx)];
        if filtered.len() == 0 {
            break;
        }

        let invalid_values = match rules.get(&entry) {
            Some(val) => val,
            None => continue,
        };

        for val in invalid_values {
            if filtered.contains(val) {
                return false;
            }
        }
    }

    return true;
}
