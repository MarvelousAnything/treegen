use super::node::NodeGraph;

#[derive(Debug, Clone)]
pub struct Tree {
    pub nodes: NodeGraph,
}

impl Tree {
    pub fn new() -> Self {
        Self { nodes: NodeGraph::new() }
    }
    pub fn add_node(
        &mut self,
        parent: Option<usize>,
        length: f64,
        angle: f64,
        thickness: f64,
    ) -> usize {
        self.nodes.add_node(parent, length, angle, thickness)
    }

    pub(crate) fn get_tip_nodes(&self) -> Vec<usize> {
        self.nodes
            .iter(0)
            .enumerate()
            .filter(|(_, node)| node.thickness >= 0.0)
            .map(|(index, _)| index)
            .collect()
    }
}
