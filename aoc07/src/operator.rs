#[derive(Debug, Clone, Copy)]
pub enum Operator {
    PLUS,
    MULTIPLICATION,
}

impl Operator {
    pub fn execute(&self, u1: u64, u2: u64) -> u64 {
        return match self {
            Operator::PLUS => u1 + u2,
            Operator::MULTIPLICATION => u1 * u2,
        };
    }
}
