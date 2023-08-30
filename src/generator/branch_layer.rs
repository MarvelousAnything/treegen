use palette::{Srgb, cast::ArraysFrom};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::collections::HashSet;

use crate::skeleton::tree::Tree;

use super::layer::Layer;

pub struct BranchLayer<'a> {
    pub tree: &'a mut Tree,
}

// Moved to constants for easier modification
const BASE_SIZE_REDUCTION: f64 = 0.1;
const MINIMUM_SIZE: f64 = 0.6;
const INITIAL_BRANCH_SIZE: f64 = 0.9;
const INITIAL_LENGTH: f64 = 10.0;
const BASE_ANGLE_MEAN_DEG: f64 = 20.0;
const BASE_ANGLE_STD_DEV_DEG: f64 = 5.0;
const GREEN_COLOR: [u8; 3] = [0, 255, 0];

impl<'a> Layer<'a> for BranchLayer<'a> {
    fn generate(
        &'a mut self,
        spread: f64,
        _split: f64,
        branch: f64,
        variability: f64,
        _branch_height: f64,
    ) -> &'a Tree {
        let tip_nodes = self.tree.get_tip_nodes();

        let mut processed_nodes = HashSet::new();

        // Nested function for recursive branch generation
        fn generate_branch(
            node: usize,
            tree: &mut Tree,
            size: f64,
            _spread: f64,
            branch: f64,
            _variability: f64,
            processed_nodes: &mut HashSet<usize>,
        ) {
            if size <= MINIMUM_SIZE || processed_nodes.contains(&node) {
                return;
            }
            let mut local_rng = rand::thread_rng();

            processed_nodes.insert(node);
            let should_branch = local_rng.gen::<f64>() < branch;

            if !should_branch {
                return;
            }
            let base_angle_normal = Normal::new(BASE_ANGLE_MEAN_DEG, BASE_ANGLE_STD_DEV_DEG).unwrap();
            let base_angle = base_angle_normal.sample(&mut local_rng).to_radians();

            // Logic for creating sub-branches
            let new_node = tree.add_node(
                Some(node),
                INITIAL_LENGTH,
                base_angle,
                size,
            );
            tree.nodes.nodes[new_node].set_color(Srgb::from(GREEN_COLOR));

            // Recursive call for child node
            generate_branch(
                new_node,
                tree,
                size - BASE_SIZE_REDUCTION,
                _spread,
                branch,
                _variability,
                processed_nodes,
            );
        }

        tip_nodes.iter().for_each(|&node| {
            generate_branch(
                node,
                self.tree,
                INITIAL_BRANCH_SIZE,
                spread,
                branch,
                variability,
                &mut processed_nodes,
            );
        });

        self.tree
    }
}

