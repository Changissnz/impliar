use std::collections::HashMap;
use std::collections::HashSet;
use crate::setti::setf;

/*
index-range-to-function hashmap
*/
pub struct IndexRange2FunctionHM {
    pub hm: HashMap<String,fn(String) -> String>
}

impl IndexRange2FunctionHM {

    /*
    */
    pub fn at(&mut self,i:usize) -> Option<fn(String) -> String> {

        // iterate through
        for k in self.hm.clone().into_keys() {
            let h:HashSet<usize> = setf::str_to_vec(k.clone()).into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
            if h.contains(&i) {
                return Some(*(self.hm.get_mut(&k).unwrap()));
            }
        }
        None
    }

}
