#[derive(Debug, Clone)]
pub struct Node {
    pub parent_index: Option<usize>,
    pub children_indices: Vec<usize>,

    pub length: f64,
    pub angle: f64,
    pub thickness: f64,
}

#[derive(Debug)]
pub struct NodeGraph {
    nodes: Vec<Node>,
}

impl NodeGraph {
    pub fn new() -> Self {
        NodeGraph { nodes: Vec::new() }
    }

    pub fn add_node(
        &mut self,
        parent_index: Option<usize>,
        length: f64,
        angle: f64,
        thickness: f64,
    ) -> usize {
        let new_node = Node {
            parent_index,
            children_indices: Vec::new(),
            length,
            angle,
            thickness,
        };

        let node_index = self.nodes.len();
        self.nodes.push(new_node);

        if let Some(parent_index) = parent_index {
            self.nodes[parent_index].children_indices.push(node_index);
        }

        node_index
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn graph_add_node_works() {
        let mut ng = NodeGraph::new();
        ng.add_node(None, 5f64, 5f64, 5f64);
    }
}
