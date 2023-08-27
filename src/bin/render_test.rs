use treegen::{skeleton::{node::NodeGraph, tree::Tree}, generator::layer::{TrunkLayer, Layer}};

fn main() {
    let mut tree = Tree { nodes: NodeGraph::new() };
    let mut trunk = TrunkLayer { tree: &mut tree };
    trunk.generate(0.75, 0.3, 0.6, 0.3, 0.5);
    println!("tree generated with {} nodes", tree.nodes.nodes.len());
    tree.nodes.render_image("tree.jpg");
}
