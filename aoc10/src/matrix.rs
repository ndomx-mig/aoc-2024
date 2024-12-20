use crate::point::Point;

pub trait Matrix {
    type Element;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn read_at(&self, point: &Point) -> Option<&Self::Element>;

    fn abs_idx(&self, point: &Point) -> u32 {
        let rows = self.rows() as u32;
        let row = point.y as u32;
        let col = point.x as u32;
        return (rows * row) + col;
    }
}

impl Matrix for &[Vec<u8>] {
    type Element = u8;

    fn rows(&self) -> usize {
        return self.len();
    }

    fn cols(&self) -> usize {
        return match self.first() {
            Some(row) => row.len(),
            None => 0,
        };
    }

    fn read_at(&self, point: &Point) -> Option<&Self::Element> {
        if point.x < 0 || point.y < 0 {
            return None;
        }

        let row = point.y as usize;
        let col = point.x as usize;
        return self.get(row).and_then(|cols| cols.get(col));
    }
}
