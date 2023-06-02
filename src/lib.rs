pub mod graph {
    pub trait Graph {
        
    }

    pub struct Arc {
        s: u32, //source
        t: u32, //target
        c: i32, //cost
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

    }
}