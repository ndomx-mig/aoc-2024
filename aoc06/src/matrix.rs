use crate::point::Point;

pub trait Matrix {
    type Elem;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn compare_element(&self, point: &Point, comp: Self::Elem) -> bool;

    fn valid_point(&self, point: &Point) -> bool {
        return (0..self.cols()).contains(&point.x) && (0..self.rows()).contains(&point.y);
    }

    fn abs_idx(&self, point: &Point) -> usize {
        return point.y * self.rows() + point.x;
    }

    fn size(&self) -> usize {
        return self.rows() * self.cols();
    }
}

impl Matrix for Vec<String> {
    type Elem = char;

    fn rows(&self) -> usize {
        return self.len();
    }

    fn cols(&self) -> usize {
        return self[0].len();
    }

    fn compare_element(&self, point: &Point, comp: Self::Elem) -> bool {
        return match self[point.y].chars().nth(point.x) {
            Some(c) => comp == c,
            None => false,
        };
    }
}
