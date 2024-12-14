use aoc07::read_equations;

const SOURCE: &str = "input";

fn main() {
    let filename = format!("data/{}/equations.txt", SOURCE);

    let mut result: u64 = 0;

    let equations = read_equations(&filename).unwrap();
    for eq in equations {
        result += match eq.has_valid_ops() {
            Some(val) => val,
            None => 0,
        }
    }

    println!("result = {}", result);
}
