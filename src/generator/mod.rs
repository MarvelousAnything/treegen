use crate::skeleton::tree::Tree;

pub mod layer;
pub mod trunk_layer;
pub mod branch_layer;

pub trait Generator {
    fn generate(&self) -> Tree;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Params {
    pub spread: f64,
    pub split: f64,
    pub branch: f64,
    pub variability: f64,
    pub branch_height: f64,
}
