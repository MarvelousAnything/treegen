use std::f64::consts::PI;

use crate::{skeleton::tree::Tree};

pub trait Layer<'a> {
    fn generate(&'a mut self, spread: f64, split: f64, branch: f64, pull: f64, branch_height: f64) -> &'a Tree;
}

pub struct TrunkLayer<'a> {
    pub tree: &'a mut Tree,
}

impl<'a> Layer<'a> for TrunkLayer<'a> {
    fn generate(&'a mut self, _spread: f64, _split: f64, _branch: f64, _pull: f64, _branch_height: f64) -> &'a Tree {
        let _root = self.tree.add_node(None, 5f64, PI/2f64, 1f64);

        self.tree
    }
}
