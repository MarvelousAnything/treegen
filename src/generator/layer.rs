use crate::skeleton::tree::Tree;

pub trait Layer<Params> where Params : Copy {
    fn generate(
        tree: Tree,
        params: &Params
    ) -> Tree;
}
