use super::Point;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[derive(Debug, Clone)]
pub struct Quadtree {
    capacity: usize,
    boundary: BoundingBox,
    points: Vec<(Point, usize)>,
    divided: bool,
    north_west: Option<Box<Quadtree>>,
    north_east: Option<Box<Quadtree>>,
    south_west: Option<Box<Quadtree>>,
    south_east: Option<Box<Quadtree>>,
}

impl Quadtree {
    pub fn new(boundary: BoundingBox, capacity: usize) -> Self {
        Self {
            capacity,
            boundary,
            points: Vec::new(),
            divided: false,
            north_west: None,
            north_east: None,
            south_west: None,
            south_east: None,
        }
    }

    pub fn insert(&mut self, point: Point, index: usize) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push((point, index));
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        self.north_west.as_mut().unwrap().insert(point, index) ||
        self.north_east.as_mut().unwrap().insert(point, index) ||
        self.south_west.as_mut().unwrap().insert(point, index) ||
        self.south_east.as_mut().unwrap().insert(point, index)
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;

        let nw = BoundingBox { x, y, width: w, height: h };
        let ne = BoundingBox { x: x + w, y, width: w, height: h };
        let sw = BoundingBox { x, y: y + h, width: w, height: h };
        let se = BoundingBox { x: x + w, y: y + h, width: w, height: h };

        self.north_west = Some(Box::new(Quadtree::new(nw, self.capacity)));
        self.north_east = Some(Box::new(Quadtree::new(ne, self.capacity)));
        self.south_west = Some(Box::new(Quadtree::new(sw, self.capacity)));
        self.south_east = Some(Box::new(Quadtree::new(se, self.capacity)));

        self.divided = true;
    }

    pub fn query(&self, range: BoundingBox, mut found: Vec<(Point, usize)>) -> Vec<(Point, usize)> {
        if !self.boundary.intersects(&range) {
            return found;
        }

        for point in &self.points {
            if range.contains(&point.0) {
                found.push(*point);
            }
        }

        if self.divided {
            found = self.north_west.as_ref().unwrap().query(range, found.clone());
            found = self.north_east.as_ref().unwrap().query(range, found.clone());
            found = self.south_west.as_ref().unwrap().query(range, found.clone());
            found = self.south_east.as_ref().unwrap().query(range, found.clone());
        }

        found
    }
}
