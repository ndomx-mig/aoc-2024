#[derive(Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn is_bounded(&self, bounds: &Self) -> bool {
        return self.row <= bounds.row && self.col <= bounds.col;
    }

    pub fn from_ref(other: &Self) -> Self {
        return Point {
            row: other.row,
            col: other.col,
        };
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.col == other.col && self.row == other.row;
    }
}
