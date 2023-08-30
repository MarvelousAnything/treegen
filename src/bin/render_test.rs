// use std::f64::consts::PI;

use treegen::{skeleton::{node::NodeGraph, tree::Tree}, generator::{trunk_layer::TrunkLayer, layer::Layer, branch_layer::BranchLayer}};

fn main() {
    let mut tree = Tree { nodes: NodeGraph::new() };
    // tree.add_node(None, 50.0, PI / 2.0, 5.0);
    // tree.add_node(Some(0), 50.0, PI / 1.5, 5.0);
    let mut trunk = TrunkLayer { tree: &mut tree };
    trunk.generate(1.0, 1.0, 0.5, 1.0, 0.0);
    let mut branch = BranchLayer { tree: &mut tree };
    branch.generate(1.0, 1.0, 0.5, 1.0, 0.0);
    // println!("{tree:#?}");
    // tree.nodes.generate_random_tree(12, 12);
    println!("tree generated with {} nodes", tree.nodes.nodes.len());
    tree.nodes.render_lined_image("tree.png");
}
