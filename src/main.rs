use dijkstra_in_rust::graph::{ListDigraph, DiGraph, Arc};

fn main() {
    let mut g = ListDigraph::new();
    for i in 0..10 {
        g.add_node(i).unwrap();
    }
    g.add_arc(2, 3).unwrap();
    g.add_arc(2, 6).unwrap();
    g.add_arc(2, 7).unwrap();
    g.add_arc(1, 2).unwrap();
    g.add_arc(5, 1).unwrap();
    g.add_arc(5, 9).unwrap();
    g.add_arc(4, 3).unwrap();
    g.add_arc(8, 4).unwrap();
    g.add_arc(7, 4).unwrap();
    for v in g.node_iter() {
        print!("{} ", v);
    }
    print!("\n");

    g.add_arcmap("c", 0).unwrap();
    g.change_am_value("c", &Arc::new(2, 6), 10).unwrap();
    for a in g.arc_iter() {
        print!("{} {}, ", a, g.get_am_value("c", a).unwrap());
    }
    print!("\n");
    g.fill_arcmap("c", 42).unwrap();
    for a in g.arc_iter() {
        print!("{} {}, ", a, g.get_am_value("c", a).unwrap());
    }
    print!("\n");
}
