use treegen::skeleton::node::NodeGraph;

fn main() {
    let mut ng = NodeGraph::new();
    ng.generate_random_tree(4, 4);
    println!("{ng:#?}");
    ng.render_image("tree.jpg");
}
