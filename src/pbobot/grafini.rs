//! graph representation structures. 
use std::collections::{HashMap,HashSet};
use crate::setti::{setf,strng_srt};

/// undirected graph representation using a
pub struct UndirectedGraph {
    pub data: HashMap<String,HashSet<String>>
}

impl UndirectedGraph {

    pub fn add_edge(&mut self, n1:String,n2:String) {
        self.add_node(n1.clone());
        self.add_node(n2.clone());
        self.add_neighbor(n1.clone(),n2.clone());
        self.add_neighbor(n2.clone(),n1.clone());
    }

    pub fn add_neighbor(&mut self, n1:String,n2:String) {
        self.data.get_mut(&n1).unwrap().insert(n2);
    }

    pub fn add_node(&mut self,n1:String) {
        assert!(is_proper_node(n1.clone()));
        if self.data.contains_key(&n1) {
            return;
        }
        self.data.insert(n1,HashSet::new());
    }

    pub fn delete_node(&mut self,n:String) {
        self.data.remove(&n);
        for (_,v) in self.data.iter_mut() {
            v.remove(&n); 
        }
    }

    pub fn delete_edge(&mut self,n1:String,n2:String) {
        self.data.get_mut(&n1).unwrap().remove(&n2);
        self.data.get_mut(&n2).unwrap().remove(&n1);
    }
}

/// entity that is represented as a graph, consists
/// of nodes and edges that follow the rule:
/// if entity has edge e from nodes n1 to n2, then
/// entity must own nodes n1 and n2. 
pub struct GraphBasedEntity {
    pub nodes: HashSet<String>, 
    pub edges: HashSet<String>
}

pub fn edge_to_str(n1:String,n2:String) -> String {
    let mut v = vec![n1,n2];
    v.sort_by(strng_srt::str_cmp3);
    setf::vec_to_str(v,'_')
}

pub fn is_proper_node(n1:String) -> bool {
    for x in n1.chars() {
        if x == '_' {
            return false;
        }
    }
    true 
}

impl GraphBasedEntity {

    pub fn add_node(&mut self,n:String) {
        assert!(is_proper_node(n.clone()));
        self.nodes.insert(n);
    }

    /// # arguments
    /// 
    pub fn add_edge(&mut self,n1:String,n2:String) {
        assert!(is_proper_node(n1.clone()));
        assert!(is_proper_node(n2.clone()));
        let s = edge_to_str(n1,n2);
        self.edges.insert(s);
    }

}