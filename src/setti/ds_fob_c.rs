//! implementation of distance-size (forward|binary) collector

use crate::setti::vs;
use crate::setti::vs::VSelect;
use crate::setti::uvs;
use crate::setti::set_gen;

use crate::setti::uvs::UVSelect;
use std::str::FromStr;
use std::fmt;


use ndarray::{Array1,arr1};

pub trait NE<T> {
    fn next_element(&mut self) -> Option<T>;
}

/// Calculates the vector of range options for a ds-forward element of 
/// - size `wanted_size` and 
/// - >= `distance`.
pub fn options_for_dsf_element(mut vs:VSelect, n:usize, k:usize, d:usize,s:usize, ws:usize) -> Vec<(usize,usize)> {
    assert!(d > 0);
    if ws < s {
        return Vec::new();
    }

    // case: too large
    if ws > n {
        return Vec::new();
    }

    if s > k - vs.size() {
        return Vec::new();
    }

    // get the first available forward including the distance
    let x2 = vs.available_forward(n);

    // case: no more
    if x2.is_none() {
        return Vec::new();
    }

    let mut x:usize = x2.unwrap();
    if x > 0 {
        x = x + d - 1;
    }

    let mut sol: Vec<(usize,usize)> = Vec::new();

    // iterate from
    while x < n - ws + 1 {

        // filter
        let mut vs2 = vs.clone();
        ////vs2.add_elemente(n,(x,x+ ws - 1));
        vs2.add_elemente((x,x+ ws - 1));

        if vs2.is_valid_pre_vselect(n,k,d,s) {
            sol.push((x,x+ws - 1));
        } else {
            break;
        }

        x += 1;
    }
    sol
}

/// each element is <vs::VSelect> of size k.
/// distance-size forward generator that collects qualifying elements
/// in the form of <vs::VSelect> according to its attributes
#[derive(Clone)]
pub struct DSFGen {
    /// total options
    n: usize,
    /// size of each <vs::VSelect>
    k: usize,
    /// minumum distance between each contiguous chunk of options
    d: usize,
    /// size of each contiguous chunk of options
    s: usize,
    /// elements to be considered
    pub cache: Vec<VSelect>,
    /// output
    pub results: Vec<VSelect>,
    /// more vars to be considered? 
    pub stat: bool,
    /// index during forward search
    pub c:usize
}

pub fn build_DSFGen(n: usize,k: usize,d: usize,s: usize) -> DSFGen {
    // check arguments
    assert!(n > d);
    assert!(n >= k);

    // get initial cache
    let mut c: Vec<(usize,usize)> = Vec::new();
    let mut vs: VSelect = vs::build_vselect(c);
    let mut cache: Vec<VSelect> = vec![vs];
    DSFGen{n:n,k: k,d:d,s: s,cache: cache,results: Vec::new(),stat: true,c:0}
}

/// `next_element` implementation for distance-size forward generator
impl NE<VSelect> for DSFGen {

    fn next_element(&mut self) -> Option<VSelect> {
        if !self.stat {
            return None;
        }

        if self.results.len() == 0 && self.cache.len() == 0 {
            self.stat = false;
            return None;
        }

        if self.results.len() > 0 {
            let mut r = self.results[0].clone();
            self.results = self.results[1..].to_vec();
            return Some(r);
        }

        let lc = self.cache.len();
        if lc == 0 {
            return None;
        }

        let mut vs2 = self.cache[0].clone();
        self.cache = self.cache[1..].to_vec();
        let stat = self.process_cache_element(vs2);
        if stat {
            self.stat = false;
            return None;
        }

        // case: no more results, fetch from cache
        if self.results.len() == 0 {
            return None;
        }

        let mut r = self.results[0].clone();
        self.results = self.results[1..].to_vec();
        Some(r)
    }
}

impl DSFGen {

    /// # return
    /// next possible VSelect of size <= k
    pub fn next(&mut self) -> Option<VSelect> {

        while self.stat {
            let mut u:Option<VSelect> = self.next_element();
            if u.is_none() {
                continue;
            }
            self.c += 1;
            return u;
        }

        None
    }

    /// # description 
    /// processes by bfs on each additional chunk of size \[d,k\]
    pub fn process_cache_element(&mut self,mut vs:VSelect) -> bool {
        let mut stat: bool = true;
        let mut r = self.k - vs.size();
        for i in self.s..r + 1 {
            if self.collect_for_pre(vs.clone(),i) {
                stat = false;
            }
        }
        stat
    }

    /// # description 
    /// collects `k_`-sized possibilities for a pre VSelect into cache
    /// 
    /// # return
    ///  any new pre-VSelects?
    pub fn collect_for_pre(&mut self,mut vs:VSelect, k_ : usize) -> bool {
        if vs.size() >= self.k {
            return false;
        }

        if self.k - vs.size() < k_ {
            return false;
        }

        let q:Vec<(usize,usize)> = options_for_dsf_element(vs.clone(), self.n,self.k,self.d,self.s,k_);
        if q.len() == 0 {
            return false;
        }

        let mut stat:bool = false;
        for q_ in q.iter() {
            let mut vs2 = vs.clone();
            vs2.add_elemente(*q_);


            //either add to cache or results
            if vs2.size() == self.k {
                self.results.push(vs2.clone());
                stat = true;
            } else {
                if vs2.is_valid_pre_vselect(self.n,self.k,self.d,self.s) {
                    self.cache.push(vs2);
                    stat = true;
                }
            }
        }

        stat
    }
}

/// # description
/// iterates through <ds_fob_c::DSFGen> and displays each element if
/// `display` mode is true
///
/// # return
/// number of elements from generator
pub fn iterate_DSFGen(mut dsfg: DSFGen,display:bool) -> usize
 {
    let mut j:usize = 0;

    let mut stat:bool = true;

    while stat {
        if !dsfg.stat {
            break;
        }
        let mut u:Option<VSelect> = dsfg.next_element();
        if u.is_none() {
            continue;
        }

        j += 1;
        if display {
            println!("\t\tXXXX {:?}",u.unwrap().data);
        }
    }
    j
}

// each element is <uvs::UVSelect> of size k.
/// distance-size forward generator that collects qualifying elements
/// in the form of <uvs::UVSelect> according to its attributes
#[derive(Clone)]
pub struct DSBGen {
    /// total options
    n: usize,
    /// size of each <vs::VSelect>
    k: usize,
    /// minumum distance between each contiguous chunk of options
    d: usize,
    /// size of each contiguous chunk of options
    s: usize,
    /// ordering of selection
    o : Vec<usize>,
    /// elements to be considered
    pub cache: Vec<UVSelect>,
    /// output
    pub results: Vec<UVSelect>,
    /// more vars to be considered? 
    pub stat: bool,
    /// index during forward search
    pub c:usize


}

pub fn build_DSBGen(n: usize,k: usize,d: usize,s: usize,o:Vec<usize>) -> DSBGen {
    assert!(s <= n);
    assert!(d < n);
    assert!(o.len() == n && *(o.iter().min().unwrap()) == 0 && *(o.iter().max().unwrap()) == n - 1);

    // make empty vselect
    let mut v:VSelect = vs::build_vselect(Vec::new());
    let mut uvs: UVSelect = uvs::build_uvselect(v,Vec::new());
    DSBGen{n:n,k:k,d:d,s:s,o:o,cache:vec![uvs],results: Vec::new(),stat:true,c:0}
}

/// `next_element` implementation for distance-size binary generator
impl NE<UVSelect> for DSBGen {

    fn next_element(&mut self) -> Option<UVSelect> {
        if self.results.len() != 0 {
            let sol = Some(self.results[0].clone());
            self.results = self.results[1..].to_vec();
            return sol;
        }

        if self.cache.len() == 0 {
            self.stat = false;
            return None;
        }

        // pop reference
        let q = self.cache[0].clone();
        self.cache = self.cache[1..].to_vec();

        // process cache elements
        self.process_cache_element(q);
        None
    }

}

impl DSBGen {

    /// # return
    /// next possible <uvs::UVSelect> of size <= k
    pub fn next(&mut self) -> Option<UVSelect> {

        while self.stat {
            let mut u:Option<UVSelect> = self.next_element();
            if u.is_none() {
                continue;
            }
            self.c += 1;
            return u;
        }

        None
    }

    /// # description 
    /// processes by bfs on each additional chunk of size \[d,k\]
    fn process_cache_element(&mut self, mut c: UVSelect) {

        // process cache element in size range s,n
        let sz = c.size();
        for i in self.s..(self.n + 1 - sz) {
            self.process_cache_element_(c.clone(),i);
        }
    }

    /// # description 
    /// processes a cache \<UVSelect\> element `c` by extending it with available ranges
    /// of minumum size `s_`
    fn process_cache_element_(&mut self, mut c: UVSelect, s_:usize) {
        assert!(!(s_ > self.n));
        let vus = c.available_binaries(self.n,s_,self.d);
        let mut sol:Vec<UVSelect> = Vec::new();
        let vus2 = set_gen::ordered_vec_by_reference(vus,self.o.clone());

        for v in vus2.into_iter() {
            let mut c2 = c.clone();
            let v_ = <usize>::from_str(v.as_str()).unwrap();
            c2.add_elemente((v_,v_ + s_));
            if c2.is_valid_pre_vselect(self.n,self.k,self.d,self.s) {
                sol.push(c2.clone());
            }

            if c2.size() == self.k {
                self.results.push(c2);
            }
        }

        sol.extend(self.cache.clone());
        self.cache = sol;
    }

}

pub fn sol_dsbg_case1() -> Vec<Vec<(usize,usize)>> {

    vec![[(7, 9)].to_vec(), [(6, 8)].to_vec(), [(5, 7)].to_vec(), [(4, 6)].to_vec(), [(3, 5)].to_vec(), [(2, 4)].to_vec(),
    [(1, 3)].to_vec(), [(0, 2)].to_vec()]
}

pub fn sol_dsbg_case2() -> Vec<Vec<(usize,usize)>> {

    let s:Vec<Vec<(usize,usize)>> = vec![[(5, 9)].to_vec(), [(4, 8)].to_vec(), [(3, 7)].to_vec(),
    [(2, 6)].to_vec(), [(1, 5)].to_vec(), [(0, 4)].to_vec(), [(4, 5), (7, 9)].to_vec(), [(3, 4), (7, 9)].to_vec(),
    [(2, 3), (7, 9)].to_vec(), [(1, 2), (7, 9)].to_vec(), [(0, 1), (7, 9)].to_vec(), [(3, 4), (6, 8)].to_vec(),
    [(2, 3), (6, 8)].to_vec(), [(1, 2), (6, 8)].to_vec(), [(0, 1), (6, 8)].to_vec(), [(2, 3), (5, 7)].to_vec(),
    [(1, 2), (5, 7)].to_vec(), [(0, 1), (5, 7)].to_vec(), [(4, 6), (8, 9)].to_vec(), [(1, 2), (4, 6)].to_vec(),
    [(0, 1), (4, 6)].to_vec(), [(3, 5), (8, 9)].to_vec(), [(3, 5), (7, 8)].to_vec(), [(0, 1), (3, 5)].to_vec(),
    [(2, 4), (8, 9)].to_vec(), [(2, 4), (7, 8)].to_vec(), [(2, 4), (6, 7)].to_vec(), [(1, 3), (8, 9)].to_vec(),
    [(1, 3), (7, 8)].to_vec(), [(1, 3), (6,7)].to_vec(), [(1, 3), (5, 6)].to_vec(), [(0, 2), (8, 9)].to_vec(),
    [(0, 2), (7, 8)].to_vec(), [(0, 2), (6, 7)].to_vec(), [(0, 2), (5, 6)].to_vec(), [(0, 2), (4, 5)].to_vec(),
    [(4, 6), (8, 9)].to_vec(), [(3, 5), (8, 9)].to_vec(), [(2, 4), (8, 9)].to_vec(), [(1, 3), (8, 9)].to_vec(),
    [(0, 2), (8, 9)].to_vec(), [(3, 5), (7, 8)].to_vec(), [(2, 4), (7, 8)].to_vec(), [(1, 3), (7, 8)].to_vec(),
    [(0, 2), (7, 8)].to_vec(), [(2, 4), (6, 7)].to_vec(), [(1, 3), (6, 7)].to_vec(), [(0, 2), (6, 7)].to_vec(),
    [(1, 3), (5, 6)].to_vec(), [(0, 2), (5, 6)].to_vec(), [(4, 5), (7, 9)].to_vec(), [(0, 2), (4, 5)].to_vec(),
    [(3, 4), (7, 9)].to_vec(), [(3, 4), (6, 8)].to_vec(), [(2, 3), (7, 9)].to_vec(), [(2, 3), (6, 8)].to_vec(),
    [(2, 3),(5, 7)].to_vec(), [(1, 2), (7, 9)].to_vec(), [(1, 2), (6, 8)].to_vec(), [(1, 2), (5, 7)].to_vec(),
    [(1, 2), (4, 6)].to_vec(), [(0, 1), (7, 9)].to_vec(), [(0, 1), (6, 8)].to_vec(), [(0, 1), (5, 7)].to_vec(),
    [(0, 1), (4, 6)].to_vec(), [(0, 1), (3, 5)].to_vec()];
    s
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_options_for_dsf_element() {

        let mut vs = vs::build_vselect(vec![(0,3)]);
        let mut o = options_for_dsf_element(vs.clone(),20,8,4,5,4);
        assert!(o.len() == 0);

        o = options_for_dsf_element(vs.clone(),20,8,4,4,4);
        assert!(o.len() == 10);

        o = options_for_dsf_element(vs.clone(),20,8,4,3,4);
        assert!(o.len() == 10);

        o = options_for_dsf_element(vs.clone(),20,8,4,3,1);
        assert!(o.len() == 0);
    }

    #[test]
    fn test_DSFGen_next_element() {
        let mut dg = build_DSFGen(20,8,2,4);
        let mut x = iterate_DSFGen(dg.clone(),false);
        assert_eq!(x,91);

        dg = build_DSFGen(20,8,2,8);
        x = iterate_DSFGen(dg.clone(),false);
        assert_eq!(x,13);

        dg = build_DSFGen(20,20,2,19);
        x = iterate_DSFGen(dg.clone(),true);
        assert_eq!(x,1);

        dg = build_DSFGen(20,19,2,20);
        x = iterate_DSFGen(dg.clone(),true);
        assert_eq!(x,0);

        dg = build_DSFGen(20,19,2,19);
        x = iterate_DSFGen(dg.clone(),true);
        assert_eq!(x,2);
    }

    #[test]
    fn test_DSBGen_next() {

        let o = vec![9,8,7,6,5,4,3,2,1,0];
        let mut dsbg0 = build_DSBGen(10,3,2,1,o.clone());
        let mut sol0: Vec<Vec<(usize,usize)>> = Vec::new();
        while dsbg0.stat {
            let mut x = dsbg0.next();

            if x.is_none() {
                break;
            }
            sol0.push(x.unwrap().v.data);
        }
        assert_eq!(sol0,sol_dsbg_case1());


        let mut dsbg = build_DSBGen(10,5,2,1,o.clone());
        let mut sol: Vec<Vec<(usize,usize)>> = Vec::new();
        while dsbg.stat {
            let mut x = dsbg.next();

            if x.is_none() {
                break;
            }
            sol.push(x.unwrap().v.data);
        }
        assert_eq!(sol,sol_dsbg_case2());
    }

}
