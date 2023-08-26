pub trait Generator {
    fn create_node(&mut self);
    fn add_child(&mut self);
}
