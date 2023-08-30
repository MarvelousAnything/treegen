use crate::skeleton::tree::Tree;

pub trait Layer<'a, Params> where Params : Copy {
    fn generate(
        &'a mut self,
        params: Params
    ) -> &'a Tree;
}
