use crate::skeleton::tree::Tree;

pub mod layer;

pub trait Generator {
    fn generate(&self) -> Tree;
}
