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
