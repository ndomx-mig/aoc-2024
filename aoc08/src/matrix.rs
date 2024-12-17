use crate::point::Point;

pub trait Matrix {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn max_idx(&self) -> Point {
        return Point {
            col: self.cols() - 1,
            row: self.rows() - 1,
        };
    }

    fn abs_idx(&self, point: &Point) -> usize {
        return point.row * self.rows() + point.col;
    }
}

impl Matrix for Vec<String> {
    fn rows(&self) -> usize {
        return self.len();
    }

    fn cols(&self) -> usize {
        return self[0].len();
    }
}
