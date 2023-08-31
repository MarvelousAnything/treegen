use druid::{Data, Lens};
use palette::Srgb;
use rayon::prelude::*;
use std::ops::Add;

use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::skeleton::tree::Tree;

use super::layer::Layer;

#[derive(Debug)]
pub struct TrunkLayer;

#[derive(Debug, Copy, Clone, Data, Lens)]
pub struct TrunkParams {
    pub spread: f64,
    pub split: f64,
    pub branch: f64,
    pub variability: f64,
    pub default_branch_length: f64,
    pub default_branch_size: f64,
    pub branch_size_falloff: f64,
    pub default_height_mean: f64,
    pub split_falloff_peak: f64,
    pub lean_bias: f64,
    pub variability_modifier: f64,
    pub angle_spread_positive: f64,
    pub angle_spread_negative: f64,
    pub max_children: f64,
}

impl TrunkParams {
    pub fn new(spread: f64, split: f64, branch: f64, variability: f64) -> Self {
        Self {
            spread,
            split,
            branch,
            variability,
            default_branch_length: 30.0,
            default_branch_size: 50.0,
            branch_size_falloff: 2.0,
            default_height_mean: 10.0,
            split_falloff_peak: 5.0,
            lean_bias: 0.0,
            variability_modifier: 0.4,
            angle_spread_positive: 10.0,
            angle_spread_negative: -10.0,
            max_children: 5.0,
        }
    }
}

impl Layer<TrunkParams> for TrunkLayer {
    fn generate(mut tree: Tree, params: &TrunkParams) -> Tree {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(params.default_height_mean, 2.0 * params.variability).unwrap();

        let height = normal.sample(&mut rng);

        let max_size = params.default_branch_size / params.branch_size_falloff;
        let min_size =
            params.default_branch_size / (params.default_height_mean + params.branch_size_falloff);

        let root = tree.add_node(None, 2.0 * height, 0f64, max_size);
        let mut tip_nodes: Vec<usize> = vec![root];

        let split_rate =
            |n: usize| params.split * (-(n as f64 - params.split_falloff_peak).powi(2)).exp();
        let branch_rate =
            |n: usize| 3.0 * params.branch * (-(n as f64 - 0.0).powi(2)).div_euclid(100.0).exp();
        let branch_size =
            |n: usize| params.default_branch_size / ((n as f64) + params.branch_size_falloff);

        struct NodeToAdd {
            parent_index: usize,
            length: f64,
            angle: f64,
            size: f64,
            color: Srgb<u8>,
        }

        for i in 0..height as usize {
            let is_split = rng.gen::<f64>() < split_rate(i);
            let split_index = rng.gen_range(0..tip_nodes.len());

            let new_tips: Vec<NodeToAdd> = tip_nodes
                .par_iter()
                .flat_map(|&j| {
                    let mut local_rng = rand::thread_rng();
                    let should_branch = local_rng.gen::<f64>() < branch_rate(i);

                    let current_node = tree.nodes.nodes[j].clone();
                    let size = branch_size(i);
                    let color = Srgb::new(((225.0 * (size - min_size) / (max_size - min_size)) + 30.0) as u8, 0, 0);
                    if current_node.children_indices.len() >= params.max_children as usize {
                        return vec![];
                    }
                    if is_split && j == split_index {
                        let normal = Normal::new(
                            params.spread * params.angle_spread_positive.to_radians()
                                + params.lean_bias.to_radians(),
                            params.variability * params.variability_modifier,
                        )
                        .unwrap();
                        let angle_a = normal
                            .sample(&mut local_rng)
                            .to_radians()
                            .add(current_node.angle);
                        let normal = Normal::new(
                            params.spread * params.angle_spread_negative.to_radians()
                                + params.lean_bias.to_radians(),
                            params.variability * params.variability_modifier,
                        )
                        .unwrap();
                        let angle_b = normal
                            .sample(&mut local_rng)
                            .to_radians()
                            .add(current_node.angle);

                        vec![
                            NodeToAdd {
                                parent_index: j,
                                length: params.default_branch_length,
                                angle: angle_a,
                                size,
                                color,
                            },
                            NodeToAdd {
                                parent_index: j,
                                length: params.default_branch_length,
                                angle: angle_b,
                                size,
                                color,
                            },
                        ]
                    } else if should_branch {
                        let normal = Normal::new(
                            params.lean_bias.to_radians(),
                            params.spread * params.variability * params.variability_modifier,
                        )
                        .unwrap();
                        let angle = normal.sample(&mut local_rng);
                        vec![NodeToAdd {
                            parent_index: j,
                            length: params.default_branch_length,
                            angle,
                            size,
                            color,
                        }]
                    } else {
                        vec![]
                    }
                })
                .collect();

            for new_tip in new_tips {
                let node_index = tree.add_node(
                    Some(new_tip.parent_index),
                    new_tip.length,
                    new_tip.angle,
                    new_tip.size,
                );
                tree.nodes.nodes[node_index].color = Some(new_tip.color);
                tip_nodes.push(node_index);
            }
        }

        tree
    }
}
