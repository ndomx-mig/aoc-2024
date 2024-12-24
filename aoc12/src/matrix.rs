use crate::point::Point;

pub trait Matrix {
    type Elem;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn read_at(&self, point: &Point) -> Option<Self::Elem>;
    fn compare_at(&self, point: &Point, target: Self::Elem) -> Option<bool>;

    fn size(&self) -> Point {
        return Point {
            row: self.rows(),
            col: self.cols(),
        };
    }

    fn bounds(&self) -> Point {
        return Point {
            row: self.rows() - 1,
            col: self.cols() - 1,
        };
    }

    fn abs_idx(&self, point: &Point) -> usize {
        return (self.rows() * point.row) + point.col;
    }
}

impl Matrix for &[String] {
    type Elem = char;

    fn rows(&self) -> usize {
        return self.len();
    }

    fn cols(&self) -> usize {
        return match self.first() {
            Some(row) => row.len(),
            None => 0,
        };
    }

    fn read_at(&self, point: &Point) -> Option<Self::Elem> {
        return self
            .get(point.row)
            .and_then(|row| row.chars().nth(point.col));
    }

    fn compare_at(&self, point: &Point, target: char) -> Option<bool> {
        return match self.read_at(point) {
            Some(e) => Some(e == target),
            None => None,
        };
    }
}
