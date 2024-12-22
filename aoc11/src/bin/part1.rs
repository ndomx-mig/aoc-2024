use aoc11::{iterator::iterate, read_inputs};

fn main() {
    // let inputs = read_inputs("example").unwrap();
    let inputs = read_inputs("input").unwrap();

    // let sum: u64 = inputs.into_iter().map(|n| iterate(n, 0)).sum();
    // println!("{}", sum);

    let mut sum = 0u64;
    for n in inputs.into_iter() {
        let curr = iterate(n, 0);
        println!("sum({}): {}", n, curr);
        sum += curr;
    }

    println!("sum: {}", sum);
}
