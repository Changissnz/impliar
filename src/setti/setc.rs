/*
some close functions for sets
*/
use crate::setti::set_gen;
use std::collections::HashSet;

use factorial::Factorial;

// CAUTION: does not support ordering
pub fn decompose_set(s:HashSet<String>, ns:usize) -> Vec<HashSet<String>> {

    // hash set to vec
    let mut v: Vec<String> = Vec::from_iter(s.clone());
    let mut sg: set_gen::SGen = set_gen::SGen{value:v,data:Vec::new(),
                                next:Vec::new()};
    sg.fcollect_all(ns);
    sg.data
}

//pub fn implication_closing_number
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



}
