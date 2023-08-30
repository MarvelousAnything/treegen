use nalgebra::Point2;

use palette::Srgb;

use crate::utils::quadtree::{BoundingBox, Quadtree};

pub const WIDTH: usize = 400;
pub const HEIGHT: usize = 800;

pub type Point = Point2<f64>;

#[derive(Debug, Clone)]
pub struct Node {
    pub parent_index: Option<usize>,
    pub children_indices: Vec<usize>,
    pub point: Point,
    pub length: f64,
    pub angle: f64,
    pub thickness: f64,
    pub color: Option<Srgb<u8>>,
}

impl Node {
    pub fn next_point(&self) -> Point {
        let x = self.point.x + (self.length * self.angle.sin());
        let y = self.point.y - (self.length * self.angle.cos());
        Point::new(x, y)
    }

    pub fn set_color(&mut self, color: Srgb<u8>) {
        self.color = Some(color);
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
            color: None,
        };

        let node_index = self.nodes.len();
        self.quadtree.insert(new_node.point, node_index);
        self.nodes.push(new_node);

        if let Some(parent_index) = parent_index {
            self.nodes[parent_index].children_indices.push(node_index);
        }

        node_index
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
