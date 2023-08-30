use crate::skeleton::tree::Tree;

pub mod layer;
pub mod trunk_layer;
pub mod branch_layer;

pub trait Generator {
    fn generate(&self) -> Tree;
}
