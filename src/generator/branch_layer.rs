use druid::{Data, Lens};
use palette::Srgb;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::collections::HashSet;

use crate::skeleton::tree::Tree;

use super::layer::Layer;

#[derive(Debug)]
pub struct BranchLayer;

#[derive(Debug, Copy, Clone, Data, Lens)]
pub struct BranchParams {
    pub spread: f64,
    pub branch: f64,
    pub variability: f64,
    pub base_size_reduction: f64,
    pub minimum_size: f64,
    pub initial_branch_size: f64,
    pub initial_length: f64,
    pub base_angle_mean_deg: f64,
    pub base_angle_std_dev_deg: f64,
    pub color: [u8; 3],
}

impl BranchParams {
    pub fn new(
        spread: f64,
        branch: f64,
        variability: f64,
    ) -> Self {
        Self {
            spread,
            branch,
            variability,
            base_size_reduction: 0.1,
            minimum_size: 0.6,
            initial_branch_size: 0.9,
            initial_length: 10.0,
            base_angle_mean_deg: 20.0,
            base_angle_std_dev_deg: 5.0,
            color: [0, 255, 0],
        }
    }
}

impl Layer<BranchParams> for BranchLayer {
    fn generate(mut tree: Tree, params: &BranchParams) -> Tree {
        let tip_nodes = tree.get_tip_nodes();

        let mut processed_nodes = HashSet::new();

        fn generate_branch(
            node: usize,
            tree: &mut Tree,
            size: f64,
            branch: f64,
            processed_nodes: &mut HashSet<usize>,
            params: &BranchParams,
        ) {
            if size <= params.minimum_size || processed_nodes.contains(&node) {
                return;
            }
            let mut local_rng = rand::thread_rng();

            processed_nodes.insert(node);
            let should_branch = local_rng.gen::<f64>() < branch;

            if !should_branch {
                return;
            }
            let base_angle_normal =
                Normal::new(params.base_angle_mean_deg, params.base_angle_std_dev_deg).unwrap();
            let base_angle = base_angle_normal.sample(&mut local_rng).to_radians();

            // Logic for creating sub-branches
            let new_node = tree.add_node(Some(node), params.initial_length, base_angle, size);
            tree.nodes.nodes[new_node].set_color(Srgb::from(params.color));

            // Recursive call for child node
            generate_branch(
                new_node,
                tree,
                size - params.base_size_reduction,
                branch,
                processed_nodes,
                params
            );
        }

        tip_nodes.iter().for_each(|&node| {
            generate_branch(
                node,
                &mut tree,
                params.initial_branch_size,
                params.branch,
                &mut processed_nodes,
                &params
            );
        });

        tree
    }
}
