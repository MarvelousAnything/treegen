use std::{f64::consts::PI, ops::Add};

use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::skeleton::tree::Tree;

pub trait Layer<'a> {
    fn generate(
        &'a mut self,
        spread: f64,
        split: f64,
        branch: f64,
        variability: f64,
        branch_height: f64,
    ) -> &'a Tree;
}

pub struct TrunkLayer<'a> {
    pub tree: &'a mut Tree,
}

impl<'a> Layer<'a> for TrunkLayer<'a> {
    fn generate(
        &'a mut self,
        spread: f64,
        split: f64,
        branch: f64,
        variability: f64,
        branch_height: f64,
    ) -> &'a Tree {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(15.0, 2.0 * variability).unwrap();

        let root = self.tree.add_node(None, 5f64, PI / 2f64, 1f64);
        let mut tip_nodes: Vec<usize> = vec![root];

        let height = normal.sample(&mut rng);
        let split_rate = |n: usize| split * (-(n as f64 - 5.0).powi(2)).exp();
        let branch_size = |n: usize| 20.0 / n as f64 + 5.0;

        for i in 0..height as usize {
            let is_split = rng.gen::<f64>() < split_rate(i);
            let split_index = rng.gen_range(0..tip_nodes.len());

            for j in 0..tip_nodes.len() {
                let current_node = self.tree.nodes.nodes[tip_nodes[j]].clone();
                let size = branch_size(i);
                if is_split && j == split_index {
                    let normal = Normal::new(spread * 10.0, variability * 15.0).unwrap();
                    let angle_a = normal.sample(&mut rng).add(current_node.angle);
                    let normal = Normal::new(spread * -10.0, variability * 15.0).unwrap();
                    let angle_b = normal.sample(&mut rng).add(current_node.angle);

                    tip_nodes.push(self.tree.add_node(Some(j), size, angle_a, 1.0));
                    tip_nodes.push(self.tree.add_node(Some(j), size, angle_b, 1.0));
                    continue;
                }

                let normal = Normal::new(0.0, spread * variability * 15.0).unwrap();
                let angle = normal.sample(&mut rng);
                tip_nodes.push(self.tree.add_node(Some(j), size, angle, 1.0))
            }
        }

        self.tree
    }
}
