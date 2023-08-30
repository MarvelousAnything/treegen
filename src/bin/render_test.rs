// use std::f64::consts::PI;

use image::RgbImage;
use treegen::{skeleton::{node::{NodeGraph, WIDTH, HEIGHT}, tree::Tree}, generator::{trunk_layer::TrunkLayer, layer::Layer, branch_layer::BranchLayer}, rendering::{PolygonRenderer, Renderer}};

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
    let mut renderer = PolygonRenderer;
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
    renderer.render_tree(& mut image, &tree);
    image.save("tree.png").expect("could not save image");
}
