pub mod graph {
    use std::collections::HashMap;

    pub trait DiGraph<'a> {
        fn node_count(&self) -> usize;
        fn arc_count(&self) -> usize;
        fn out_degree(&self, v: u32) -> usize;
        fn add_node(&mut self, v: u32);
        fn add_arc(&mut self, a: Arc);
        //fn node_iter<I>(&self) -> I
        //    where I: Iterator + 'a;

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
            where I: Iterator + 'a
        {
            self.nodes.iter()
        }*/
    }
}