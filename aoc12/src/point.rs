use crate::dir::Dir;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn move_to(&self, dir: Dir, bounds: &Self) -> Option<Self> {
        return match dir {
            Dir::UP => self.move_up(),
            Dir::DOWN => self.move_down(bounds),
            Dir::LEFT => self.move_left(),
            Dir::RIGHT => self.move_right(bounds),
        };
    }

    pub fn from_idx(idx: usize, bounds: &Self) -> Self {
        return Self {
            row: idx / bounds.row,
            col: idx % bounds.row,
        };
    }

    fn move_up(&self) -> Option<Self> {
        return if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                col: self.col,
            })
        } else {
            None
        };
    }

    fn move_down(&self, bounds: &Self) -> Option<Self> {
        return if self.row < bounds.row {
            Some(Self {
                row: self.row + 1,
                col: self.col,
            })
        } else {
            None
        };
    }

    fn move_left(&self) -> Option<Self> {
        return if self.col > 0 {
            Some(Self {
                row: self.row,
                col: self.col - 1,
            })
        } else {
            None
        };
    }

    fn move_right(&self, bounds: &Self) -> Option<Self> {
        return if self.col < bounds.col {
            Some(Self {
                row: self.row,
                col: self.col + 1,
            })
        } else {
            None
        };
    }
}
