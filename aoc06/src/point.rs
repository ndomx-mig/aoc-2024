#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn move_up(&mut self) {
        // self.y -= 1;
        self.y = self.y.overflowing_sub(1).0;
    }

    pub fn move_right(&mut self) {
        // self.x += 1;
        self.x = self.x.overflowing_add(1).0;
    }

    pub fn move_down(&mut self) {
        // self.y += 1;
        self.y = self.y.overflowing_add(1).0;
    }

    pub fn move_left(&mut self) {
        // self.x -= 1;
        self.x = self.x.overflowing_sub(1).0;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
