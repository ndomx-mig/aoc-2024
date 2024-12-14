use equation::Equation;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod equation;
pub mod operator;

pub fn read_equations(filename: &str) -> io::Result<Vec<Equation>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut equations = Vec::new();

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);

        if let Some((key, values)) = line.split_once(':') {
            // Parse the key
            let result: u64 = key
                .trim()
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid key format"))?;

            // Parse the values
            let values: Vec<u64> = values
                .trim()
                .split_whitespace()
                .map(|v| {
                    v.parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid value format")
                    })
                })
                .collect::<Result<_, _>>()?;

            equations.push(Equation { result, values });
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Line does not contain a colon",
            ));
        }
    }

    return Ok(equations);
}
