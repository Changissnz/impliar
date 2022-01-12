/*
some close functions for sets
*/
use crate::setti::set_gen;
use crate::setti::setf;
use crate::setti::setf::Count;


use std::collections::HashSet;
use std::collections::HashMap;

use factorial::Factorial;

/*
*/
pub fn is_closed_implication(s:Vec<HashSet<String>>, k:usize) -> bool {
    let mut vc = setf::VectorCounter{data:HashMap::new()};
    let mut unique = HashSet::new();

    let r = s[0].len();

    // case: 1-set
    if r == 1 {
        return false;
    }

    for s_ in s.iter() {
        assert_eq!(r,(*s_).len());

        // check if s_ unique
            // hash set to string
        let mut c: Vec<String> = Vec::from_iter((s_.clone()));
        let mut cv = set_gen::stringized_srted_vec(&mut c);
        if !unique.contains(&cv) {
            unique.insert(cv);
        } else {
            continue;
        }

        vc.countv(c);
    }

    if unique.len() != number_of_m_intersections(k,r) {
        return false;
    }
    let ans = unique_object_frequency_requirement(k,r);

    for (k,v) in vc.data.into_iter() {
        if (v as usize) != ans {
            return false
        }
    }
    true
}

// CAUTION: does not support ordering
pub fn decompose_set(s:HashSet<String>, ns:usize) -> Vec<HashSet<String>> {

    // hash set to vec
    let mut v: Vec<String> = Vec::from_iter(s.clone());
    let mut sg: set_gen::SGen = set_gen::SGen{value:v,data:Vec::new(),
                                next:Vec::new()};
    sg.fcollect_all(ns);
    sg.data
}

pub fn nCr(n: usize,r: usize) -> usize {
    assert!(n > 0 && r > 0);
    assert!(n >= r, "got {} {}",n,r);
    n.factorial() / (r.factorial() * (n - r).factorial())
}


pub fn number_of_m_intersections(n: usize,m:usize) -> usize {
    let mut s = 0 as usize;
    let mut maxxy = n - (m - 1);
    for i in 1..(maxxy + 1) {
        //println!("@ I={},(n,m)={},{}",i,n-i,m-1);
        let mut c = nCr(n - i,m - 1);
        s += c;
    }
    s
}

pub fn unique_object_frequency_requirement(n: usize,m:usize) -> usize {
    nCr(n-1,m-1)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_number_of_m_intersections() {
        assert_eq!(number_of_m_intersections(7,4),nCr(7,4));
        assert_eq!(number_of_m_intersections(5,2),nCr(5,2));
    }

    #[test]
    fn test_is_closed_implication() {

        let mut value = vec!["arbitrox".to_string(), "bartinuell".to_string(),
            "radinox".to_string(), "reskeyiazam".to_string(),"garbitol".to_string(),
            "weinfarsitol".to_string()];

        let mut sg = set_gen::SGen{value:value,data:Vec::new(),
                                    next:Vec::new()};

        for i in 2..7 {
            sg.fcollect_all(i);
            let b = is_closed_implication(sg.data,6);
            assert_eq!(b,true);
            sg.data = Vec::new();
            sg.next = Vec::new();
        }
    }

}
