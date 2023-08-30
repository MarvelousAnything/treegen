use crate::skeleton::{node::Node, tree::Tree};

use super::canvas::Canvas;

pub trait Renderer<ColorType: Copy> {
    fn initialize(&mut self, canvas: &mut dyn Canvas<ColorType>);
    fn render_node(&self, canvas: &mut dyn Canvas<ColorType>, node: &Node);
    fn render_tree(&mut self, canvas: &mut dyn Canvas<ColorType>, tree: &Tree) {
        self.initialize(canvas);
        tree.nodes.iter(0).for_each(|node| self.render_node(canvas, node));
    }
    fn get_color_for_node(node: &Node) -> ColorType;
}
