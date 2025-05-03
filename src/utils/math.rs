use brickadia::util::octree::Point;
use std::ops::{Add, Mul};

pub trait TupleMath<T> {
    fn add(self, other: (T, T)) -> (T, T);
    fn mul(self, scalar: T) -> (T, T);
}

impl<T> TupleMath<T> for (T, T)
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    fn add(self, other: (T, T)) -> (T, T) {
        (self.0 + other.0, self.1 + other.1)
    }
    fn mul(self, scalar: T) -> (T, T) {
        (self.0 * scalar, self.1 * scalar)
    }
}


/// Finds the bounding box
pub fn bounds(points: &[(i32, i32)]) -> (Point, Point) {
    let (mut min_x, mut min_y) = (i32::MAX, i32::MAX); // extreme
    let (mut max_x, mut max_y) = (i32::MIN, i32::MIN); // values
    for &(x, y) in points {
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }
    (Point::new(min_x, min_y, 1), Point::new(max_x, max_y, 1))
}