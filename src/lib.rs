pub mod graph {
    use std::collections::HashMap;
    use std::fmt;

    pub trait DiGraph {
        fn node_count(&self) -> usize;
        fn arc_count(&self) -> usize;
        fn out_degree(&self, v: u32) -> usize;

        fn add_node(&mut self, v: u32) -> Result<(), String>;
        fn add_arc(&mut self, s: u32, t: u32) -> Result<(), String>;

        fn add_nodemap(&mut self, name: &str, fill_value: i32) -> Result<(), String>;
        fn get_nm_value(&self, name: &str, k: u32) -> Option<&i32>;
        fn change_nm_value(&mut self, name: &str, k: u32, new_value: i32) -> Result<(), String>;

        fn add_arcmap(&mut self, name: &str, fill_value: i32) -> Result<(), String>;
        fn get_am_value(&self, name: &str, k: &Arc) -> Option<&i32>;
        fn change_am_value(&mut self, name: &str, k: &Arc, new_value: i32) -> Result<(), String>;

        fn node_iter(&self) -> Box<dyn Iterator<Item=&u32> + '_>;
        fn out_arc_iter(&self, v: u32) -> Box<dyn Iterator<Item=&Arc> + '_>;
        fn arc_iter(&self) -> Box<dyn Iterator<Item=&Arc> +'_>;
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

    impl fmt::Display for Arc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}->{}]", self.s, self.t)
        }
    }

    pub struct ListDigraph {
        out_arcs: HashMap<u32, Vec<Arc>>,
        arc_cnt: usize,

        nodemaps: HashMap<String, HashMap<u32, i32>>,
        arcmaps: HashMap<String, HashMap<Arc, i32>>,
    }

    impl ListDigraph {
        pub fn new() -> ListDigraph {
            ListDigraph {
                out_arcs: HashMap::<u32, Vec<Arc>>::new(),
                arc_cnt: 0,
                nodemaps: HashMap::new(),
                arcmaps: HashMap::new(),
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
                Ok(())
            }
        }

        fn add_arc(&mut self, s: u32, t: u32) -> Result<(), String> {
            if self.out_arcs.contains_key(&s) && self.out_arcs.contains_key(&t) {
                let arc_vec = self.out_arcs.get_mut(&s).unwrap();
                arc_vec.push(Arc{s, t});
                self.arc_cnt += 1;
                Ok(())
            } else {
                Err("Invalid nodes!".to_string())
            }
        }

        fn add_nodemap(&mut self, name: &str, fill_value: i32) -> Result<(), String>{
            if self.nodemaps.contains_key(name) {
                Err("A nodemap with this name already exists!".to_string())
            } else {
                let m = self.node_iter()
                    .map(|v|{(*v, fill_value)})
                    .collect::<HashMap<u32, i32>>();
                self.nodemaps.insert(name.to_string(), m);
                Ok(())
            }
        }

        fn get_nm_value(&self, name: &str, k: u32) -> Option<&i32> {
            let nm_option = self.nodemaps.get(name);
            match nm_option {
                Some(nm) => nm.get(&k),
                None => None,
            }
        }

        fn change_nm_value(&mut self, name: &str, k: u32, new_value: i32) -> Result<(), String> {
            let nm_option = self.nodemaps.get_mut(name);
            match nm_option {
                Some(nm) => {
                    let value_option = nm.get_mut(&k);
                    match value_option {
                        Some(value) => *value = new_value,
                        None => return Err("Key doesn't exists in nodemap!".to_string())
                    }
                },
                None => return Err("Nodemap doesn't exists!".to_string())
            };
            Ok(())
        }

        fn add_arcmap(&mut self, name: &str, fill_value: i32) -> Result<(), String> {
            if self.arcmaps.contains_key(name) {
                Err("An arcmap with this name already exists!".to_string())
            } else {
                let m: HashMap<Arc, i32> = self.arc_iter()
                    .map(|a|{(a.clone(), fill_value)})
                    .collect::<HashMap<Arc, i32>>();
                self.arcmaps.insert(name.to_string(), m);
                Ok(())
            }
        }

        fn get_am_value(&self, name: &str, k: &Arc) -> Option<&i32> {
            let am_option = self.arcmaps.get(name);
            match am_option {
                Some(am) => am.get(&k),
                None => None,
            }
        }
        
        fn change_am_value(&mut self, name: &str, k: &Arc, new_value: i32) -> Result<(), String> {
            let am_option = self.arcmaps.get_mut(name);
                match am_option {
                    Some(am) => {
                        let value_option = am.get_mut(&k);
                        match value_option {
                            Some(value) => *value = new_value,
                            None => return Err("Key doesn't exists in arcmap!".to_string())
                        }
                    },
                    None => return Err("Arcmap doesn't exists!".to_string())
                };
                Ok(())
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