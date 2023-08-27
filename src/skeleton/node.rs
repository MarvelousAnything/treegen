use std::f64::consts::PI;

use rayon::prelude::*;

use image::RgbImage;
use rand::Rng;

use crate::utils::Point;

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
        let x = self.point.x + (self.length * self.angle.cos());
        let y = self.point.y + (self.length * self.angle.sin());
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

#[derive(Debug)]
pub struct NodeGraph {
    nodes: Vec<Node>,
}

impl NodeGraph {
    pub fn new() -> Self {
        NodeGraph { nodes: Vec::new() }
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
            Point::new(800f64, 600f64)
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
        self.nodes.push(new_node);

        if let Some(parent_index) = parent_index {
            self.nodes[parent_index].children_indices.push(node_index);
        }

        node_index
    }

    fn is_black(&self, point: Point) -> bool {
        self.iter(0)
            .any(|node| node.is_valid(point, node.thickness))
    }

    pub fn generate_random_tree(&mut self, depth: usize, max_children: usize) {
        self.nodes.clear();

        if depth > 0 {
            let root_index = self.add_node(None, 1.5, 0.0, 1.0);
            self.generate_random_subtree(root_index, depth - 1, max_children);
        }
    }

    fn generate_random_subtree(
        &mut self,
        parent_index: usize,
        remaining_depth: usize,
        max_children: usize,
    ) {
        let rng = &mut rand::thread_rng();

        let num_children = rng.gen_range(1..=max_children);
        for _ in 0..num_children {
            let length = rng.gen_range(0.5..=1.0);
            let angle = -rng.gen_range(0.0..=1.0 * PI);
            // let thickness = rng.gen_range(0.5..=1.0);

            let child_index = self.add_node(Some(parent_index), length * 100.0, angle, 3.0);

            if remaining_depth > 0 {
                self.generate_random_subtree(child_index, remaining_depth - 1, max_children);
            }
        }
    }

    pub fn render_image(&self, filename: &str) {
        let width = 1600;
        let height = 1200;
        let mut image = RgbImage::new(width, height);

        // Calculate the number of bytes per row
        let bytes_per_row = 3 * width; // 3 bytes per pixel (for RGB)

        // Convert the image buffer into a mutable slice
        let img_buf = image.as_mut();

        // Parallelize row-wise
        img_buf
            .par_chunks_mut(bytes_per_row as usize)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let offset = x * 3;
                    let point = Point::new(x as f64, y as f64);
                    let pixel = if self.is_black(point) {
                        [1, 1, 1]
                    } else {
                        [(0.3 * x as f32) as u8, 0, (0.3 * y as f32) as u8]
                    };
                    row[offset as usize..offset as usize + 3].copy_from_slice(&pixel);
                }
            });

        image.save(filename).expect("could not save image");
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
