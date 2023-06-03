use dijkstra_in_rust::graph::{ListDigraph, Arc, DiGraph};

fn main() {
    let mut g = ListDigraph::new();
    g.add_node(0);
    g.add_node(1);
    g.add_nodemap("c", 5);

}
