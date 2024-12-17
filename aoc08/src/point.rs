use std::cmp::Ordering;

#[derive(Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.col == other.col && self.row == other.row;
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.row > other.row || self.col > other.col {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        };
    }

    fn lt(&self, other: &Self) -> bool {
        return self.row < other.row && self.col < other.col
    }
}
