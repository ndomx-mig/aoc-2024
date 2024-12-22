use std::env;

use aoc11::iterator::apply_rules_once;

fn main() {
    let n: u64 = env::args()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let result = iterate(n);
    println!("result: {:?}", result);
}

fn iterate(n: u64) -> [u64; 25] {
    let mut result = [1u64; 25];
    let mut curr: Vec<u64> = vec![n];
    for i in 0usize..25 {
        curr = curr.into_iter().map(|x| apply_rules_once(x)).flatten().collect();
        result[i] = curr.len() as u64;
    }

    return result;
}
