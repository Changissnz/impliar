/*
implementation of ds(f|b)c:
distance-size (forward|binary) collector; greedy solution.

For the arguments
`selection`:Vec<(usize,usize)>, the parameters
>= `distance`
== `size`
*/

use crate::setti::vs;
use crate::setti::vs::VSelect;

/*
Calculates the vector of range options for a ds-forward element of size `wanted_size` and distance
`distance`.
*/
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
    let mut c: Vec<(usize,usize)> = Vec::new();
    if x > 0 {
        x = x + d - 1;
    }

    let mut sol: Vec<(usize,usize)> = Vec::new();

    // iterate from
    while x < n - ws + 1 {

        // filter
        let mut vs2 = vs.clone();
        vs2.add_elemente(n,(x,x+ ws - 1));
        if vs2.is_valid_pre_vselect(n,k,d,s) {
            sol.push((x,x+ws - 1));
        } else {
            break;
        }

        x += 1;
    }
    sol
}

// each element is VSelect of size k
#[derive(Clone)]
pub struct DSFGen {
    n: usize,
    k: usize,
    d: usize,
    s: usize,
    pub cache: Vec<VSelect>,
    pub results: Vec<VSelect>,
    pub stat: bool // more vars?
}

pub fn build_DSFGen(n: usize,k: usize,d: usize,s: usize) -> DSFGen {
    // check arguments
    assert!(n > d);
    assert!(n >= k);

    // get initial cache
    let mut c: Vec<(usize,usize)> = Vec::new();
    let mut vs: VSelect = vs::build_vselect(c);
    let mut cache: Vec<VSelect> = vec![vs];
    DSFGen{n:n,k: k,d:d,s: s,cache: cache,results: Vec::new(),stat: true}
}


impl DSFGen {

    pub fn next_element(&mut self) -> Option<VSelect> {
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



    /*
    processes by bfs on each additional chunk of size [d,k];
    */
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

    /*
    collects k_-sized possibilities for a pre VSelect into cache

    return:
    - any new pre-VSelects?
    */
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
            let mut d2 = vs2.size();
            vs2.add_elemente(self.n,*q_);

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

/*
*/
pub fn iterate_DSFGen(mut dsfg:DSFGen,display:bool) -> usize {
    let mut j:usize = 0;

    let mut stat:bool = true;

    while stat {
        if !dsfg.stat {
            break;
        }
        let mut u = dsfg.next_element();
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
        assert_eq!(x,90);

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
}
