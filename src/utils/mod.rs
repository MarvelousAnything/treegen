use std::ops::Sub;

pub mod params;
pub mod quadtree;

pub type Line = (Point, Point);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Point {
        let mag = self.magnitude();
        Point::new(self.x / mag, self.y / mag)
    }

    pub fn point_line_distance(&self, line: Line) -> f64 {
        if line.0.x == line.1.x {
            (self.x - line.0.x).abs()
        } else if line.0.y == line.1.y {
            (self.y - line.0.y).abs()
        } else {
            let diff = line.1 - line.0;
            let num = (diff.y * self.x - diff.x * self.y + line.1.x * line.0.y - line.1.y * line.0.x).abs();
            let den = (diff.y.powi(2) + diff.x.powi(2)).sqrt();
            num / den
        }
    }

    pub fn tuple(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
