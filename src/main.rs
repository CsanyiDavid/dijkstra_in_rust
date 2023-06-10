use dijkstra_in_rust::graph::{ListDigraph, DiGraph, Arc};

fn main() {
    let mut g = ListDigraph::new();
    for i in 0..10 {
        g.add_node(i);
    }
    g.add_arc(2, 3);
    g.add_arc(2, 6);
    g.add_arc(2, 7);
    g.add_arc(1, 2);
    g.add_arc(5, 1);
    g.add_arc(5, 9);
    g.add_arc(4, 3);
    g.add_arc(8, 4);
    g.add_arc(7, 4);
    for v in g.node_iter() {
        print!("{} ", v);
    }
    print!("\n");

    g.add_arcmap("c", 0);
    g.change_am_value("c", Arc::new(2, 6), 10);
    for a in g.arc_iter() {
        print!("{} {}, ", a, g.get_am_value("c", *a).unwrap());
    }
    print!("\n");
}
