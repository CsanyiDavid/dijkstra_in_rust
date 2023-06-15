use std::collections::HashMap;

use dijkstra_in_rust::graph::{ListDigraph, DiGraph};
use dijkstra_in_rust::maps::{ArcMap};
use dijkstra_in_rust::dijkstra::dijkstra;

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

    let mut c: HashMap<u32, u32> = HashMap::new();
    for v in g.node_iter() {
        c.insert(*v, 0);
    }

    print!("\n");
    let mut c = ArcMap::<u32>::new(0);
    c.synchronize(&g);

    dijkstra(&g, &c, 0).unwrap();

}
