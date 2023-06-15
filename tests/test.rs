#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use dijkstra_in_rust::dijkstra::dijkstra;
    use dijkstra_in_rust::graph::{ListDigraph, DiGraph, Arc};
    use dijkstra_in_rust::maps::{ArcMap};

    fn create_graph() -> (ListDigraph, ArcMap<u32>) {
        let mut g = ListDigraph::new();
        let mut c: ArcMap<u32> = ArcMap::new(0);
        for i in 0..12 {
            g.add_node(i).unwrap();
        }
        let arc_list = vec![
            (1, 2, 1), (2, 3, 7), (2, 6, 6), (2, 7, 5), (4, 3, 1),
            (5, 1, 1), (5, 9, 1), (7, 4, 3), (8, 4, 1), (8, 3, 4),
            (9, 10, 1), (10, 8, 1), (3, 2, 4), (0, 1, 0), (1, 5, 2),
        ];
        for (s, t, cost) in arc_list {
            g.add_arc(s, t).unwrap();
            c.synchronize(&g);
            *c.get_mut(&Arc::new(s, t)).unwrap() = cost;
        }
        println!("{}", g.version());
        (g, c)
    }

    #[test]
    fn test_graph() {
        let (g, c) = create_graph();
        assert_eq!(g.version(), 27);
        assert_eq!(c.version(), 27);
        assert_eq!(g.node_count(), 12);
        assert_eq!(g.arc_count(), 15);
        let out_degrees = HashMap::from([
            (0, 1), (1, 2), (2, 3), (3, 1), (4, 1),
            (5, 2), (6, 0), (7, 1), (8, 2), (9, 1),
            (10, 1), (11, 0),
        ]);
        for v in g.node_iter() {
            assert_eq!(g.out_degree(*v), out_degrees[&v],
                "Incorrect out degree of node {}", *v);
        }
        let arc_list = vec![
            (1, 2, 1), (2, 3, 7), (2, 6, 6), (2, 7, 5), (4, 3, 1),
            (5, 1, 1), (5, 9, 1), (7, 4, 3), (8, 4, 1), (8, 3, 4),
            (9, 10, 1), (10, 8, 1), (3, 2, 4), (0, 1, 0), (1, 5, 2),
        ];
        assert_eq!(
            arc_list.iter().map(|(s, t, _c)|{Arc::new(*s, *t)}).collect::<HashSet<Arc>>(),
            g.arc_iter().map(|a|{*a}).collect::<HashSet<Arc>>()
        );
        for (s, t, cost) in arc_list {
            assert_eq!(cost, *c.get(&Arc::new(s, t)).unwrap());
        }
    }

    #[test]
    fn test_dijkstra() {
        let (g, c) = create_graph();
        let (dist, prev) = dijkstra(&g, &c, 0).unwrap();
        println!("{:?}", dist);
        println!("{:?}", prev);
        let target_dist: Vec<Option<u32>> = vec![
            Some(0), Some(0), Some(1), Some(7),
            Some(6), Some(2), Some(7), Some(6),
            Some(5), Some(3), Some(4), None
        ];
        for v in g.node_iter() {
            assert_eq!(*dist.get(v).unwrap(), target_dist[*v as usize],
                "Incorrect distance calculated for node {}", *v    
            );
        }
    }
}