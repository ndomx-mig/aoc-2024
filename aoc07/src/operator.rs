#[derive(Debug, Clone, Copy)]
pub enum Operator {
    PLUS,
    MULTIPLICATION,
    CONCAT,
}

impl Operator {
    pub fn from_index(value: usize) -> Self {
        return match value {
            0 => Operator::PLUS,
            1 => Operator::MULTIPLICATION,
            2 => Operator::CONCAT,
            _ => panic!("Invalid index"),
        };
    }

    pub fn count() -> usize {
        return 3;
    }

    pub fn execute(&self, u1: u64, u2: u64) -> u64 {
        return match self {
            Operator::PLUS => u1 + u2,
            Operator::MULTIPLICATION => u1 * u2,
            Operator::CONCAT => self.concat(u1, u2),
        };
    }

    fn concat(&self, u1: u64, u2: u64) -> u64 {
        let string = format!("{}{}", u1, u2);
        return string.parse().unwrap();
    }
}
