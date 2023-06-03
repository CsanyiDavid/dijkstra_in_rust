pub mod graph {
    use std::collections::HashMap;

    pub trait DiGraph<'a> {
        fn node_count(&self) -> usize;
        fn arc_count(&self) -> usize;
        fn out_degree(&self, v: u32) -> usize;
        fn add_node(&mut self, v: u32);
        fn add_arc(&mut self, a: Arc);
        /*fn node_iter<I>(&self) -> I
            where I: Iterator<Item=&'a u32> + 'a;
        Why can't do this???
        Temporarily implement iterators in ListDigraph
        and try to fix this later
        */
    }

    pub struct Arc {
        s: u32, //source
        t: u32, //target
        c: i32, //cost
    }

    impl Arc {
        pub fn new(s: u32, t: u32, c: i32) -> Arc {
            Arc{s, t, c}
        }

        pub fn source(&self) -> u32 {
            self.s
        }

        pub fn target(&self) -> u32 {
            self.t
        }

        pub fn cost(&self) -> i32 {
            self.c
        }
    }

    pub struct ListDigraph {
        nodes: Vec<u32>,
        out_arcs: HashMap<u32, Vec<Arc>>,
        arc_cnt: usize,
    }

    impl ListDigraph {
        pub fn new() -> ListDigraph {
            ListDigraph {
                nodes: Vec::<u32>::new(),
                out_arcs: HashMap::<u32, Vec<Arc>>::new(),
                arc_cnt: 0,
            }
        }

        pub fn node_iter(&self) -> impl Iterator<Item=&u32> {
            self.nodes.iter()
        }

        pub fn out_arc_iter(&self, v: u32) -> impl Iterator<Item=&Arc> {
            self.out_arcs.get(&v).unwrap().iter()
        }
    }

    impl<'a> DiGraph<'a> for ListDigraph{
        fn node_count(&self) -> usize {
            self.nodes.len()
        }

        fn arc_count(&self) -> usize {
            self.arc_cnt
        }

        fn out_degree(&self, v: u32) -> usize {
            self.out_arcs.get(&v).unwrap().len()
        }

        fn add_node(&mut self, v: u32){
            self.nodes.push(v);
            self.out_arcs.insert(v, Vec::<Arc>::new());
        }

        fn add_arc(&mut self, a: Arc) {
            match self.out_arcs.get_mut(&a.source()) {
                Some(v) => {
                    v.push(a);
                    self.arc_cnt += 1;
                }
                None => panic!("")
            }
        }

        /*fn node_iter<I>(&self) -> I 
            where I: Iterator<Item=&'a u32> + 'a
        {
            self.nodes.iter()
        }*/
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::graph::{ListDigraph, DiGraph, Arc};


    #[test]
    fn add_and_count() {
        let mut g = ListDigraph::new();
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.arc_count(), 0);
        g.add_node(0);
        g.add_node(1);
        g.add_node(2);
        g.add_node(4);
        g.add_arc(Arc::new(0, 1, 0));
        g.add_arc(Arc::new(0, 4, 0));
        assert_eq!(g.node_count(), 4);
        assert_eq!(g.arc_count(), 2);
        assert_eq!(g.out_degree(0), 2);
        assert_eq!(g.out_degree(1), 0);
    }

    #[test]
    fn node_iterator() {
        let mut g = ListDigraph::new();
        let mut v: HashSet<u32> = HashSet::new();
        for i in 0..10 {
            g.add_node(i);
            v.insert(i);
        }
        let it = g.node_iter();
        assert_eq!(it.map(|x|{*x}).collect::<HashSet<u32>>(), v);
    }
}