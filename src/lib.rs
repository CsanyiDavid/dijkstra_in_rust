pub mod graph {
    use std::collections::HashMap;

    pub trait DiGraph {
        fn node_count(&self) -> usize;
        fn arc_count(&self) -> usize;
        fn out_degree(&self, v: u32) -> usize;
        fn add_node(&mut self, v: u32);
        fn add_arc(&mut self, a: Arc);
        fn add_nodemap(&mut self, name: &str, fill_value: i32);
        fn get_nm_value(&self, name: &str, v: u32) -> Option<&i32>;
        fn change_nm_value(&mut self, name: &str, v: u32, new_value: i32);
        /*fn node_iter<I>(&self) -> I
            where I: Iterator<Item=&'a u32> + 'a;
        Why can't do this???
        Temporarily implement iterators in ListDigraph
        and try to fix this later
        */
    }

    #[derive(Eq, PartialEq, Hash)]
    pub struct Arc {
        s: u32, //source
        t: u32, //target
    }

    impl Arc {
        pub fn new(s: u32, t: u32) -> Arc {
            Arc{s, t}
        }

        pub fn source(&self) -> u32 {
            self.s
        }

        pub fn target(&self) -> u32 {
            self.t
        }
    }

    pub struct ListDigraph {
        nodes: Vec<u32>,
        out_arcs: HashMap<u32, Vec<Arc>>,
        arc_cnt: usize,

        nodemaps: HashMap<String, HashMap<u32, i32>>,
    }

    impl ListDigraph {
        pub fn new() -> ListDigraph {
            ListDigraph {
                nodes: Vec::<u32>::new(),
                out_arcs: HashMap::<u32, Vec<Arc>>::new(),
                arc_cnt: 0,
                nodemaps: HashMap::new(),
            }
        }

        pub fn node_iter(&self) -> impl Iterator<Item=&u32> {
            self.nodes.iter()
        }

        pub fn out_arc_iter(&self, v: u32) -> impl Iterator<Item=&Arc> {
            self.out_arcs.get(&v).unwrap().iter()
        }
    }

    impl DiGraph for ListDigraph {
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

        fn add_nodemap(&mut self, name: &str, fill_value: i32){
            let mut m = self.node_iter()
                .map(|v|{(*v, fill_value)})
                .collect::<HashMap<u32, i32>>();
            self.nodemaps.insert(name.to_string(), m);
        }

        fn get_nm_value(&self, name: &str, k: u32) -> Option<&i32> {
            self.nodemaps[name].get(&k)
        }

        fn change_nm_value(&mut self, name: &str, k: u32, new_value: i32){
            *self.nodemaps.get_mut(name)
                .unwrap()
                .get_mut(&k)
                .unwrap() = new_value;
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
        g.add_arc(Arc::new(0, 1));
        g.add_arc(Arc::new(0, 4));
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

    #[test]
    fn nodemap() {
        let mut g = ListDigraph::new();
        g.add_node(0);
        g.add_node(1);
        g.add_nodemap("c", 5);
        assert_eq!(*g.get_nm_value("c", 0).unwrap(), 5);
        assert_eq!(*g.get_nm_value("c", 1).unwrap(), 5);
        g.change_nm_value("c", 0, 42);
        assert_eq!(*g.get_nm_value("c", 0).unwrap(), 42);
    }
}