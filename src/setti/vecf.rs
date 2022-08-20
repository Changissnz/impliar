use std::collections::HashMap;
use std::string::ToString;
use std::string::String;
use std::cmp::Ordering;
use std::cmp::PartialEq;

/// outputs (index of `t` in `v`, if element `t` in vector `v`) 
pub fn is_in_vector<T>(v:Vec<T>,t:T) -> (usize,bool)
where
T: PartialEq + Clone
{

    for (i,v_) in v.iter().enumerate() {
        if *v_ == t {
            return (i,true);
        }
    }
    (0,false)
}

pub fn subvector_indices_to_indices<T>(si:Vec<T>, v:Vec<T>) -> Vec<usize>
where
T: ToString + Clone + PartialEq,
{
    let mut q:Vec<T> = si.clone();
    let mut sol:HashMap<String,usize> = HashMap::new();
    for (i,s) in v.iter().enumerate() {

        if q.len() == 0 {
             break;
        }

        let (_,b): (usize,bool) = is_in_vector(si.clone(),(*s).clone());

        if b {
            sol.insert(s.to_string(),i);
        }
    }

    let mut sol2: Vec<usize> = Vec::new();
    for si_ in si.iter() {
        sol2.push(sol[&si_.to_string()]);
    }
    sol2
}
