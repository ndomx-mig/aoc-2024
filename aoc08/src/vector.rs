use std::ops;

use crate::point::Point;

#[derive(Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn to_point(&self, bounds: &Point) -> Option<Point> {
        if self.x < 0 || self.y < 0 {
            return None;
        }

        let point = Point {
            row: self.y as usize,
            col: self.x as usize,
        };

        return match &point > bounds {
            true => None,
            false => Some(point),
        };
    }

    pub fn from_point(point: &Point) -> Self {
        return Self {
            x: point.col as i32,
            y: point.row as i32,
        };
    }

    pub fn scalar_mul(self, rhs: i32) -> Self {
        return Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}
