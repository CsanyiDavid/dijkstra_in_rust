pub mod graph {
    pub trait Graph {
        fn node_count(&self) -> usize;
        fn arc_count(&self) -> usize;
        fn add_node(&mut self, v: u32);
        fn add_arc(&mut self, a: Arc);

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
        arcs: Vec<Arc>,
    }

    impl ListDigraph {
        pub fn new() -> ListDigraph {
            ListDigraph {
                nodes: Vec::<u32>::new(),
                arcs: Vec::<Arc>::new(),
            }
        }
    }

    impl Graph for ListDigraph{
        fn node_count(&self) -> usize {
            self.nodes.len()
        }

        fn arc_count(&self) -> usize {
            self.arcs.len()
        }

        fn add_node(&mut self, v: u32){
            self.nodes.push(v);
        }

        fn add_arc(&mut self, a: Arc) {
            self.arcs.push(a);
        }
    }
}