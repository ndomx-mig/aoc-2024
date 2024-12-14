use crate::operator::Operator;

#[derive(Debug)]
pub struct Equation {
    pub result: u64,
    pub values: Vec<u64>,
}

impl Equation {
    pub fn validate(&self, operators: &[Operator]) -> bool {
        let mut result = self.values.clone();
        for idx in 1..result.len() {
            let op = &operators[idx - 1];
            result[idx] = op.execute(result[idx], result[idx - 1]);
        }

        return match result.last() {
            Some(&val) => val == self.result,
            None => false,
        };
    }

    pub fn has_valid_ops(&self) -> Option<u64> {
        let array_size = self.values.len() - 1;
        let mut ops = vec![Operator::PLUS;array_size];

        let len = 1 << array_size;
        for val in 0..=len {
            self.ops_array(val, &mut ops);
            if self.validate(&ops) {
                return Some(self.result);
            }
        }

        return None;
    }

    fn ops_array(&self, value: usize, ops: &mut [Operator]) {
        let array_size = self.values.len() - 1;
        (0..array_size).for_each(|idx| {
            ops[idx] = match (value >> idx) & 0b1 {
                0 => Operator::PLUS,
                _ => Operator::MULTIPLICATION,
            }
        })
    }
}
