pub mod graph {
    use std::collections::HashMap;
    use std::fmt;

    pub trait DiGraph {
        fn node_count(&self) -> usize;
        fn arc_count(&self) -> usize;
        fn out_degree(&self, v: u32) -> usize;

        fn add_node(&mut self, v: u32) -> Result<(), String>;
        fn contains_node(&self, v: u32) -> bool;

        fn add_arc(&mut self, s: u32, t: u32) -> Result<(), String>;
        fn contains_arc(&self, s: u32, t:u32) -> bool;

        fn node_iter(&self) -> Box<dyn Iterator<Item=&u32> + '_>;
        fn out_arc_iter(&self, v: u32) -> Box<dyn Iterator<Item=&Arc> + '_>;
        fn arc_iter(&self) -> Box<dyn Iterator<Item=&Arc> +'_>;

        fn version(&self) -> usize;
        fn get_action(&self, action_id: usize) -> Option<Action>;
    }

    #[derive(Eq, PartialEq, Hash, Copy, Clone)]
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

    #[derive(Clone)]
    pub enum Action {
        AddNode(u32),
        EraseNode(u32),
        AddArc(Arc),
        EraseArc(Arc),
    }

    impl fmt::Display for Arc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}->{}]", self.s, self.t)
        }
    }

    pub struct ListDigraph {
        out_arcs: HashMap<u32, Vec<Arc>>,
        arc_cnt: usize,
        history: Vec<Action>,
    }

    impl ListDigraph {
        pub fn new() -> ListDigraph {
            ListDigraph {
                out_arcs: HashMap::<u32, Vec<Arc>>::new(),
                arc_cnt: 0,
                history: Vec::new(),
            }
        }
    }

    impl DiGraph for ListDigraph {
        fn node_count(&self) -> usize {
            self.out_arcs.len()
        }

        fn arc_count(&self) -> usize {
            self.arc_cnt
        }

        fn out_degree(&self, v: u32) -> usize {
            self.out_arcs.get(&v).unwrap().len()
        }

        fn add_node(&mut self, v: u32) -> Result<(), String> {
            if self.out_arcs.contains_key(&v){
                Err("Node already exists!".to_string())
            } else {
                self.out_arcs.insert(v, Vec::<Arc>::new());
                self.history.push(Action::AddNode(v));
                Ok(())
            }
        }

        fn contains_node(&self, v: u32) -> bool {
            self.out_arcs.contains_key(&v)
        }

        fn add_arc(&mut self, s: u32, t: u32) -> Result<(), String> {
            if self.out_arcs.contains_key(&s) && self.out_arcs.contains_key(&t) {
                let arc_vec = self.out_arcs.get_mut(&s).unwrap();
                arc_vec.push(Arc{s, t});
                self.arc_cnt += 1;
                self.history.push(Action::AddArc(Arc{s,t}));
                Ok(())
            } else {
                Err("Invalid nodes!".to_string())
            }
        }

        fn contains_arc(&self, s: u32, t: u32) -> bool {
            match self.out_arcs.get(&s) {
                Some(arc_vec) => arc_vec.contains(&Arc{s,t}),
                None => false,
            }
        }

        fn node_iter(&self) -> Box<dyn Iterator<Item=&u32> + '_> {
            Box::new(self.out_arcs.keys())
        }

        fn out_arc_iter(&self, v: u32) -> Box<dyn Iterator<Item=&Arc> + '_> {
            Box::new(self.out_arcs.get(&v).unwrap().iter())
        }

        fn arc_iter(&self) -> Box<dyn Iterator<Item=&Arc> + '_> {
            let it = self.node_iter().flat_map(
                |&v| self.out_arc_iter(v)
            );
            Box::new(it)
        }

        fn version(&self) -> usize {
            self.history.len()
        }

        fn get_action(&self, action_id: usize) -> Option<Action> {
            if 0<action_id && action_id<=self.version() {
                Some(self.history[action_id-1].clone())
            } else {
                None
            }
        }
    }
}

pub mod maps {
    use std::collections::HashMap;
    use crate::graph::{DiGraph, Action, Arc};

    pub struct NodeMap<T: Copy> {
        m: HashMap<u32, T>,
        default_value: T,
        version: usize,
    }
    
    impl<T: Copy> NodeMap<T> {
        pub fn new(default_value: T) -> NodeMap<T> {
            NodeMap {
                m: HashMap::new(),
                default_value,
                version: 0,
            }
        }

        pub fn version(&self) -> usize {
            self.version
        }

        //Warning: always must recieve the same graph for correct behaviour
        pub fn synchronize<G: DiGraph>(&mut self, g: &G) {
            if g.version() > self.version {
                for action_id in self.version()+1..g.version()+1 {
                    let act = g.get_action(action_id).unwrap();
                    match act {
                        Action::AddNode(v) => self.m.insert(v, self.default_value),
                        Action::EraseNode(v) => self.m.remove(&v),
                        _ => None,
                    };
                }
            }
        }

        pub fn get(&self, k: &u32) -> Option<&T> {
            self.m.get(k)
        }

        pub fn get_mut(&mut self, k: &u32) -> Option<&mut T>  {
            self.m.get_mut(k)
        }

        pub fn fill(&mut self, fill_value: T) {
            for value in self.m.values_mut() {
                *value = fill_value;
            }
        }
    }

    pub struct ArcMap<T: Copy> {
        m: HashMap<Arc, T>,
        default_value: T,
        version: usize,
    }
    
    impl<T: Copy> ArcMap<T> {
        pub fn new(default_value: T) -> ArcMap<T> {
            ArcMap {
                m: HashMap::new(),
                default_value,
                version: 0,
            }
        }

        pub fn version(&self) -> usize {
            self.version
        }

        //Warning: always must recieve the same graph for correct behaviour
        pub fn synchronize<G: DiGraph>(&mut self, g: &G) {
            if g.version() > self.version {
                for action_id in self.version()+1..g.version()+1 {
                    let act = g.get_action(action_id).unwrap();
                    match act {
                        Action::AddArc(a) => self.m.insert(a, self.default_value),
                        Action::EraseArc(a) => self.m.remove(&a),
                        _ => None,
                    };
                }
            }
        }

        pub fn get(&self, k: &Arc) -> Option<&T> {
            self.m.get(k)
        }

        pub fn get_mut(&mut self, k: &Arc) -> Option<&mut T>  {
            self.m.get_mut(k)
        }

        pub fn fill(&mut self, fill_value: T) {
            for value in self.m.values_mut() {
                *value = fill_value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
}