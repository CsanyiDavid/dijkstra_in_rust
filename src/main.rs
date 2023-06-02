use dijkstra_in_rust::graph::{ListDigraph, Arc, Graph};

fn main() {
    let mut g = ListDigraph::new();
    println!("Node cnt: {}", g.node_count());
    println!("Arc cnt: {}", g.arc_count());
    g.add_node(0);
    g.add_node(1);
    g.add_arc(Arc::new(0, 1, 0));
    println!("Node cnt: {}", g.node_count());
    println!("Arc cnt: {}", g.arc_count());
}
