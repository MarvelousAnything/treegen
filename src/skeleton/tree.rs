use super::node::NodeGraph;

#[derive(Debug)]
pub struct Tree {
    nodes: NodeGraph,
}

impl Tree {
    pub fn add_node(&mut self, parent: Option<usize>, length: f64, angle: f64, thickness: f64) -> usize {
        self.nodes.add_node(parent, length, angle, thickness)
    }
}
