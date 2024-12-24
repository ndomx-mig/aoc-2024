use std::collections::HashSet;

use crate::{dir::Dir, matrix::Matrix, point::Point};

pub struct Region {
    pub start: Point,
    pub points: HashSet<usize>,
    pub id: char,
}

impl Region {
    pub fn area(&self) -> usize {
        return self.points.len();
    }

    pub fn perimeter(&self, map: &[String]) -> usize {
        return self
            .points
            .iter()
            .map(|&idx| {
                let point = Point::from_idx(idx, &map.size());
                let edges = self.count_edges(map, &point);
                return edges;
            })
            .sum();
    }

    fn count_edges(&self, map: &[String], point: &Point) -> usize {
        let mut count = 0;

        count += self.count_dir(map, point, Dir::UP);
        count += self.count_dir(map, point, Dir::DOWN);
        count += self.count_dir(map, point, Dir::LEFT);
        count += self.count_dir(map, point, Dir::RIGHT);

        return count;
    }

    fn count_dir(&self, map: &[String], point: &Point, dir: Dir) -> usize {
        let bounds = map.bounds();

        return if let Some(moved) = point.move_to(dir, &bounds) {
            match map.compare_at(&moved, self.id) {
                Some(res) => {
                    if res {
                        0
                    } else {
                        1
                    }
                }
                None => 0,
            }
        } else {
            1
        };
    }
}
