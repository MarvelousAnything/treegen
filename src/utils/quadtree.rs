use crate::skeleton::node::Node;

use super::Point;

pub trait HasPoint {
    fn point(&self) -> Point;
    fn is_valid(&self, point: Point) -> bool;
}

impl HasPoint for Node {
    fn point(&self) -> Point {
        self.point
    }

    fn is_valid(&self, point: Point) -> bool {
        self.is_valid(point, self.thickness)
    }
}

// Quadtree implementation
#[derive(Debug, Clone)]
pub enum Quadtree<T: HasPoint> {
    Node {
        bounds: [f64; 4],
        objects: Vec<T>,
        children: [Box<Quadtree<T>>; 4],
    },
    Empty,
}

impl<T: HasPoint + Clone> Quadtree<T> {
    pub fn new(bounds: [f64; 4]) -> Self {
        Self::Node {
            bounds,
            objects: Vec::new(),
            children: [
                Box::new(Quadtree::Empty),
                Box::new(Quadtree::Empty),
                Box::new(Quadtree::Empty),
                Box::new(Quadtree::Empty),
            ],
        }
    }

    pub fn insert(&mut self, obj: T) {
        let (should_insert, should_subdivide, quadrant) =
            if let Quadtree::Node {
                bounds, objects, ..
            } = self
            {
                let should_insert = point_within_bounds(obj.point(), bounds);
                let should_subdivide = objects.len() >= 4;
                let quadrant = self.get_quadrant(obj.point());
                (should_insert, should_subdivide, quadrant)
            } else {
                (false, false, 0)
            };

        if !should_insert {
            return;
        }

        if let Quadtree::Node {
            bounds,
            objects,
            children,
        } = self
        {
            if !should_subdivide {
                objects.push(obj);
                return;
            }

            let mid_x = (bounds[0] + bounds[2]) / 2.0;
            let mid_y = (bounds[1] + bounds[3]) / 2.0;

            children[0] = Box::new(Quadtree::new([bounds[0], bounds[1], mid_x, mid_y]));
            children[1] = Box::new(Quadtree::new([bounds[0], mid_y, mid_x, bounds[3]]));
            children[2] = Box::new(Quadtree::new([mid_x, bounds[1], bounds[2], mid_y]));
            children[3] = Box::new(Quadtree::new([mid_x, mid_y, bounds[2], bounds[3]]));

            children[quadrant].insert(obj);
        }
    }

    fn get_quadrant(&self, point: Point) -> usize {
        match self {
            Quadtree::Node { bounds, .. } => {
                let mid_x = (bounds[0] + bounds[2]) / 2.0;
                let mid_y = (bounds[1] + bounds[3]) / 2.0;

                if point.x <= mid_x {
                    if point.y <= mid_y {
                        0
                    } else {
                        1
                    }
                } else {
                    match point.y <= mid_y {
                        true => 2,
                        false => 3,
                    }
                }
            }
            Quadtree::Empty => panic!("get_quadrant called on an Empty Quadtree"),
        }
    }

    pub fn query_point(&self, point: Point, results: &mut Vec<T>) {
        match self {
            Quadtree::Node { bounds, objects, children } => {
                if !point_within_bounds(point, bounds) {
                    return;
                }

                results.extend(objects.iter().cloned().filter(|node| node.is_valid(point)));

                for child in children.iter() {
                    child.query_point(point, results);
                }
            },
            Quadtree::Empty => {},
        }
    }
}
fn point_within_bounds(point: Point, bounds: &[f64; 4]) -> bool {
    point.x >= bounds[0] && point.x <= bounds[2] && point.y >= bounds[1] && point.y <= bounds[3]
}

