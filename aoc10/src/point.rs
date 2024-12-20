use std::fmt::Debug;

use crate::dir::Dir;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn move_to(&self, dir: Dir) -> Point {
        return match dir {
            Dir::UP => Point {
                x: self.x,
                y: self.y - 1,
            },
            Dir::DOWN => Point {
                x: self.x,
                y: self.y + 1,
            },
            Dir::LEFT => Point {
                x: self.x - 1,
                y: self.y,
            },
            Dir::RIGHT => Point {
                x: self.x + 1,
                y: self.y,
            },
        };
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}
