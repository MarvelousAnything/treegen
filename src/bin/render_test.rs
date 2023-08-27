use treegen::skeleton::node::NodeGraph;

fn main() {
    let mut ng = NodeGraph::new();
    ng.generate_random_tree(6, 6);
    println!("{ng:#?}");
    ng.render_image("ng.png");
}
