use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;
use std::ops::Add;

use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::skeleton::tree::Tree;

use super::layer::Layer;

pub const DEFAULT_BRANCH_LENGTH: f64 = 30.0;
pub const DEFAULT_BRANCH_SIZE: f64 = 50.0;
pub const BRANCH_SIZE_FALLOFF: f64 = 20.0;
pub const DEFAULT_HEIGHT_MEAN: f64 = 10.0;
pub const SPLIT_FALLOFF_PEAK: f64 = 5.0;
pub const LEAN_BIAS: f64 = 0.0;
pub const VARIABILITY_MODIFIER: f64 = 0.4;
pub const ANGLE_SPREAD_POSITIVE: f64 = 10.0;
pub const ANGLE_SPREAD_NEGATIVE: f64 = -10.0;
pub const MAX_CHILDREN: usize = 5;

pub struct TrunkLayer<'a> {
    pub tree: &'a mut Tree,
}

#[derive(Debug, Copy, Clone)]
pub struct TrunkParams {
    pub spread: f64,
    pub split: f64,
    pub branch: f64,
    pub variability: f64,
}

impl TrunkParams {
    pub fn new(spread: f64, split: f64, branch: f64, variability: f64) -> Self {
        Self {
            spread,
            split,
            branch,
            variability,
        }
    }
}

impl<'a> Layer<'a, TrunkParams> for TrunkLayer<'a> {
    fn generate(
        &'a mut self,
        params: TrunkParams
    ) -> &'a Tree {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(DEFAULT_HEIGHT_MEAN, 2.0 * params.variability).unwrap();

        let root = self.tree.add_node(None, 10f64, 0f64, 1f64);
        let mut tip_nodes: Vec<usize> = vec![root];

        let height = normal.sample(&mut rng);
        let split_rate = |n: usize| params.split * (-(n as f64 - SPLIT_FALLOFF_PEAK).powi(2)).exp();
        let branch_rate =
            |n: usize| 3.0 * params.branch * (-(n as f64 - 0.0).powi(2)).div_euclid(100.0).exp();
        let branch_size = |n: usize| DEFAULT_BRANCH_SIZE / ((n as f64) + BRANCH_SIZE_FALLOFF);

        // Define a struct to store the intermediate data
        struct NodeToAdd {
            parent_index: usize,
            length: f64,
            angle: f64,
            size: f64,
        }

        for i in (0..height as usize).progress() {
            let is_split = rng.gen::<f64>() < split_rate(i);
            let split_index = rng.gen_range(0..tip_nodes.len());

            let new_tips: Vec<NodeToAdd> = tip_nodes
                .par_iter()
                .progress()
                .flat_map(|&j| {
                    let mut local_rng = rand::thread_rng();
                    let should_branch = local_rng.gen::<f64>() < branch_rate(i);

                    let current_node = self.tree.nodes.nodes[j].clone();
                    let size = branch_size(i);
                    if current_node.children_indices.len() >= MAX_CHILDREN {
                        return vec![];
                    }
                    if is_split && j == split_index {
                        let normal = Normal::new(
                            params.spread * ANGLE_SPREAD_POSITIVE.to_radians() + LEAN_BIAS.to_radians(),
                            params.variability * VARIABILITY_MODIFIER,
                        )
                        .unwrap();
                        let angle_a = normal
                            .sample(&mut local_rng)
                            .to_radians()
                            .add(current_node.angle);
                        let normal = Normal::new(
                            params.spread * ANGLE_SPREAD_NEGATIVE.to_radians() + LEAN_BIAS.to_radians(),
                            params.variability * VARIABILITY_MODIFIER,
                        )
                        .unwrap();
                        let angle_b = normal
                            .sample(&mut local_rng)
                            .to_radians()
                            .add(current_node.angle);

                        vec![
                            NodeToAdd {
                                parent_index: j,
                                length: DEFAULT_BRANCH_LENGTH,
                                angle: angle_a,
                                size,
                            },
                            NodeToAdd {
                                parent_index: j,
                                length: DEFAULT_BRANCH_LENGTH,
                                angle: angle_b,
                                size,
                            },
                        ]
                    } else if should_branch {
                        let normal = Normal::new(
                            LEAN_BIAS.to_radians(),
                            params.spread * params.variability * VARIABILITY_MODIFIER,
                        )
                        .unwrap();
                        let angle = normal.sample(&mut local_rng);
                        vec![NodeToAdd {
                            parent_index: j,
                            length: DEFAULT_BRANCH_LENGTH,
                            angle,
                            size,
                        }]
                    } else {
                        vec![]
                    }
                })
                .collect();

            for new_tip in new_tips {
                let node_index = self.tree.add_node(
                    Some(new_tip.parent_index),
                    new_tip.length,
                    new_tip.angle,
                    new_tip.size,
                );
                tip_nodes.push(node_index);
            }
        }

        self.tree
    }
}
