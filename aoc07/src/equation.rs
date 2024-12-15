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
            let r = op.execute(result[idx - 1], result[idx]);
            result[idx] = r;
        }

        return match result.last() {
            Some(&val) => val == self.result,
            None => false,
        };
    }

    pub fn has_valid_ops(&self) -> Option<u64> {
        let array_size = self.values.len() - 1;
        let mut ops = vec![Operator::PLUS; array_size];

        let len = (3 as u64).pow(array_size as u32);
        for val in 0..=len {
            self.ops_array(val, &mut ops);
            if self.validate(&ops) {
                return Some(self.result);
            }
        }

        return None;
    }

    fn ops_array(&self, value: u64, ops: &mut [Operator]) {
        let base = Operator::count() as u64;
        let array_size = ops.len();

        (0..array_size).for_each(|idx| {
            let operator_index = (value / base.pow(idx as u32)) % base;
            ops[idx] = Operator::from_index(operator_index as usize);
        });
    }
}
