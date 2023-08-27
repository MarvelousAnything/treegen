use std::f64::consts::PI;

use image::RgbImage;
use image::Rgb;
use imageproc::drawing::draw_line_segment_mut;
use rand::Rng;
use rand_distr::Normal;

#[derive(Debug, Clone)]
pub struct Node {
    pub parent_index: Option<usize>,
    pub children_indices: Vec<usize>,

    pub length: f64,
    pub angle: f64,
    pub thickness: f64,
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
        let new_node = Node {
            parent_index,
            children_indices: Vec::new(),
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

    pub fn generate_trunk(&mut self, depth: usize, max_children: usize) {
        let rng = &mut rand::thread_rng();
        let normal = Normal::new(0.0, 3.0).unwrap();
        let parent_index = self.add_node(None, 2.0, PI/2.0, 1.0);

        let num_children = rng.gen_range(1..=max_children);
        for _ in 0..num_children {
            let length = rng.gen_range(0.5..=1.0);
            let angle = -rng.gen_range(0.0..=1.0 * PI);
            let thickness = rng.gen_range(0.5..=1.0);

            let child_index = self.add_node(Some(parent_index), length, angle, thickness);

            if depth > 0 {
                self.generate_random_subtree(child_index, depth - 1, max_children);
            }
        }
    }

    pub fn generate_random_tree(&mut self, depth: usize, max_children: usize) {
        self.nodes.clear();

        if depth > 0 {
            let root_index = self.add_node(None, 1.5, 0.0, 1.0);
            self.generate_random_subtree(root_index, depth - 1, max_children);
        }
    }

    fn generate_random_subtree(&mut self, parent_index: usize, remaining_depth: usize, max_children: usize) {
        let rng = &mut rand::thread_rng();
        
        let num_children = rng.gen_range(1..=max_children);
        for _ in 0..num_children {
            let length = rng.gen_range(0.5..=1.0);
            let angle = -rng.gen_range(0.0..=1.0 * PI);
            let thickness = rng.gen_range(0.5..=1.0);

            let child_index = self.add_node(Some(parent_index), length, angle, thickness);

            if remaining_depth > 0 {
                self.generate_random_subtree(child_index, remaining_depth - 1, max_children);
            }
        }
    }

    pub fn render_image(&self, filename: &str) {
        let mut image = RgbImage::new(1600, 1200);

        self.render_node(&mut image, &self.nodes[0], (800, 1200), 200.0, 0.0);

        image.save(filename).expect("Failed to save image");
    }

    fn render_node(&self, image: &mut RgbImage, node: &Node, position: (i32, i32), length: f64, angle_offset: f64) {
        let rng = &mut rand::thread_rng();

        let end_x = position.0 as f64 + length * node.angle.cos();
        let end_y = position.1 as f64 + length * node.angle.sin();

        let start = (position.0 as f32, position.1 as f32);
        let end = (end_x as f32, end_y as f32);

        draw_line_segment_mut(image, start, end, Rgb([rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)]));

        let child_angle = node.angle + angle_offset;
        let child_length = length * node.length;
        for &child_index in &node.children_indices {
            self.render_node(image, &self.nodes[child_index], (end_x as i32, end_y as i32), child_length, child_angle);
        }
    }
}
