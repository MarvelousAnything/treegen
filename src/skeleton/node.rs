use imageproc::drawing::draw_polygon_mut;
use indicatif::ProgressBar;
use nalgebra::{Point2, Vector2};

use rayon::prelude::*;

use image::{Rgb, RgbImage};

use crate::utils::{
    quadtree::{BoundingBox, Quadtree},
    Point,
};

pub const WIDTH: usize = 400;
pub const HEIGHT: usize = 800;

#[derive(Debug, Clone)]
pub struct Node {
    pub parent_index: Option<usize>,
    pub children_indices: Vec<usize>,
    pub point: Point,
    pub length: f64,
    pub angle: f64,
    pub thickness: f64,
}

impl Node {
    pub fn next_point(&self) -> Point {
        let x = self.point.x + (self.length * self.angle.sin());
        let y = self.point.y - (self.length * self.angle.cos());
        Point::new(x, y)
    }

    pub fn is_valid(&self, point: Point, edge_size: f64) -> bool {
        if point.distance(self.point) < edge_size || point.distance(self.next_point()) < edge_size {
            return true;
        }

        let distance = point.point_line_distance((self.point, self.next_point()));
        if distance > edge_size {
            return false;
        }

        let start = self.point;
        let end = self.next_point();
        (point.x >= start.x.min(end.x) - edge_size && point.x <= start.x.max(end.x) + edge_size)
            && (point.y >= start.y.min(end.y) - edge_size
                && point.y <= start.y.max(end.y) + edge_size)
    }
}

#[derive(Debug, Clone)]
pub struct NodeGraph {
    pub nodes: Vec<Node>,
    pub quadtree: Quadtree,
}

impl NodeGraph {
    pub fn new() -> Self {
        let boundary = BoundingBox {
            x: 0.0,
            y: 0.0,
            width: WIDTH as f64,
            height: HEIGHT as f64,
        };
        NodeGraph {
            nodes: Vec::new(),
            quadtree: Quadtree::new(boundary, 4),
        }
    }

    pub fn add_node(
        &mut self,
        parent_index: Option<usize>,
        length: f64,
        angle: f64,
        thickness: f64,
    ) -> usize {
        let point: Point = if let Some(parent_index) = parent_index {
            let parent = &self.nodes[parent_index];
            parent.next_point()
        } else {
            Point::new(WIDTH as f64 / 2.0, HEIGHT as f64)
        };
        let new_node = Node {
            parent_index,
            children_indices: Vec::new(),
            point,
            length,
            angle,
            thickness,
        };

        let node_index = self.nodes.len();
        self.quadtree.insert(new_node.point, node_index);
        self.nodes.push(new_node);

        if let Some(parent_index) = parent_index {
            self.nodes[parent_index].children_indices.push(node_index);
        }

        node_index
    }

    fn is_black(&self, point: Point) -> bool {
        let range = BoundingBox {
            x: point.x - 1.0, // The size here is arbitrary and depends on your use case
            y: point.y - 1.0,
            width: 10.0,
            height: 10.0,
        };

        let nearby_nodes = self.quadtree.query(range, Vec::new());

        nearby_nodes.par_iter().any(|&point| {
            let node: Node = self.nodes[point.1].clone();
            node.is_valid(point.0, node.thickness)
        })
    }

    pub fn render_image(&self, filename: &str) {
        let width = WIDTH as u32;
        let height = HEIGHT as u32;
        let mut image = RgbImage::new(width, height);

        // Calculate the number of bytes per row
        let bytes_per_row = 3 * width; // 3 bytes per pixel (for RGB)

        // Convert the image buffer into a mutable slice
        let img_buf = image.as_mut();

        // Parallelize row-wise
        let bar = ProgressBar::new(width as u64 * height as u64);
        img_buf
            .par_chunks_mut(bytes_per_row as usize)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let offset = x * 3;
                    let point = Point::new(x as f64, y as f64);
                    let pixel = if self.is_black(point) {
                        [0, 0, 0]
                    } else {
                        [255, 255, 255]
                    };
                    row[offset as usize..offset as usize + 3].copy_from_slice(&pixel);
                    bar.inc(1);
                }
            });
        bar.finish();
        // image.save(filename).expect("could not save image");
        imageproc::window::display_image(filename, &image, width, height);
    }

    pub fn render_lined_image(&self, filename: &str) {
        let width = WIDTH as u32;
        let height = HEIGHT as u32;
        let mut image = RgbImage::new(width, height);
        for n in self.iter(0) {
            let p1 = Point2::new(n.point.x, n.point.y);
            let p2 = Point2::new(n.next_point().x, n.next_point().y);
            // println!("p1: {p1}, p2: {p2}");

            let direction = p2 - p1;
            let normal = Vector2::new(-direction.y, direction.x).normalize();
            // println!("direction: {direction}, normal: {normal}");

            // Calculate the four corners of the rectangle
            let corner1 = p1 + normal * n.thickness / 2.0;
            let corner2 = p1 - normal * n.thickness / 2.0;
            let corner3 = p2 - normal * n.thickness / 2.0;
            let corner4 = p2 + normal * n.thickness / 2.0;

            let corner1 = imageproc::point::Point::new(corner1.x as i32, corner1.y as i32);
            let corner2 = imageproc::point::Point::new(corner2.x as i32, corner2.y as i32);
            let corner3 = imageproc::point::Point::new(corner3.x as i32, corner3.y as i32);
            let corner4 = imageproc::point::Point::new(corner4.x as i32, corner4.y as i32);

            // println!("corner1: {corner1}, corner2: {corner2}, corner3: {corner3}, corner4: {corner4}");

            // println!("thickness: {}\nangle: {}", n.thickness * 255.0, n.angle.abs() * 2550.0);
            draw_polygon_mut(&mut image, &[corner1, corner2, corner3, corner4], Rgb([(n.thickness * 255.0).clamp(10.0, 255.0) as u8, 0, (n.angle.abs() * 255.0).clamp(10.0, 255.0) as u8]));
        }
        image.save(filename).expect("could not save image");
        // imageproc::window::display_image(filename, &image, width, height);
    }

    pub fn traverse(&self, start_index: usize, visitor: &mut dyn NodeVisitor) {
        let node = &self.nodes[start_index];
        visitor.visit(node);

        for &child_index in &node.children_indices {
            self.traverse(child_index, visitor);
        }
    }

    pub fn iter(&self, start_index: usize) -> NodeGraphIterator {
        NodeGraphIterator::new(self, start_index)
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

pub trait NodeVisitor {
    fn visit(&mut self, node: &Node);
}

pub struct NodeGraphIterator<'a> {
    graph: &'a NodeGraph,
    stack: Vec<usize>,
}

impl<'a> NodeGraphIterator<'a> {
    pub fn new(graph: &'a NodeGraph, start_index: usize) -> Self {
        NodeGraphIterator {
            graph,
            stack: vec![start_index],
        }
    }
}

impl<'a> Iterator for NodeGraphIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|index| {
            let node = &self.graph.nodes[index];
            self.stack.extend(&node.children_indices);
            node
        })
    }
}
