#![allow(dead_code)]
#![allow(unused_variables)]
// set-generating functions
pub use std::collections::HashSet;
pub use std::collections::HashMap;
pub use std::cmp::Eq;
pub use std::hash::Hash;
pub use std::borrow::Borrow;

use crate::setti::setf;
use crate::setti::strng_srt;
use crate::setti::selection_rule;
use selection_rule::SelectionRule;
use ndarray::{Dim,Array,Array1,Array2,array,arr2,s};

/// determines the first available choice in selection rule at column index
pub fn sr_op(sr: &mut SelectionRule,i:usize) -> (usize,bool) {
    let mut sr2 = SelectionRule{res:sr.res.clone(),
                req: sr.req.clone(),choice:sr.choice.clone()};
    let ch:Vec<usize> = sr2.choices_at_col_index(i).iter().map(|x| *x).collect();// -> HashSet<usize> {
    if ch.len() > 0 {
        return (ch[0],true);
    }
    (0,false)
}

/// decides a k-vector for an n x k SelectionRule by the greedy
/// approach:
/// first available element (top to bottom) at each column (left to right). 
pub fn greedy_decision(sr :&mut SelectionRule) -> Vec<usize> {
    let mut x: Vec<usize> = Vec::new();
    let q = sr.dimso().1;
    let mut sr2:SelectionRule = sr.clone();

    for i in 0..q {
        let (us, bo): (usize,bool) = sr_op(&mut sr2,i);

        if !bo {
            break;
        }

        // select choice and mark off the remainder
        let ch = sr2.select_choice_at_col_index(us,i,true);
        x.push(us);
    }
    x
}

/// SGen is short for set-generator; generates k-sets from choice vector
/// `value`. 
pub struct SGen {
    pub value: Vec<String>,
    pub data: Vec<HashSet<String>>,
    pub next: Vec<HashSet<String>>,
}

/// generates all sub-vectors of length k given value (Vec<String>)
impl SGen {

    pub fn fcollect_all(&mut self,k : usize) {
        let mut x :usize = 0;
        let l = self.value.len();
        while x < l {
            self.next = fcollect(self.value.clone(), x,k as usize);
            self.add_next(k);

            if self.next.len() == 0 {
                break;
            }

            x += 1;
        }
        self.next = Vec::new();
    }

    pub fn add_next(&mut self,k:usize) {
        for x in self.next.iter() {
            let h: HashSet<String> = x.clone();
            if h.len() != k {
                continue; 
            }
            self.data.push(h);
        }
    }
}

/// vector version of `fcollect`;
/// collects all subvectors of size k starting with s\[r\] in the subvector s\[r+1:\]
pub fn fcollect_vec(s: Vec<String>, r: usize, k: usize) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    // case: k == 1
    if k == 1 {
        let q2 = vec![s[r].clone()];
        result.push(q2);
        return result;
    }

    // make a queue and a dictionary
    // value == <index of last element, valid index for next>
    let mut d: HashMap<String,Vec<usize>> = HashMap::new();
    let mut q: Vec<String> = Vec::new();

    // initialize the queue and dictionary
    let d2: Vec<String> = vec![s[r].clone()];
    let d3 = vec![r,r+1];
    d.insert(setf::vec_to_str(d2,'+'),d3);
    q.push(s[r].clone());

    while q.len() > 0 {
        //println!("LENGO:\t\t{}",q.len());
        //println!("LENGO Q:\t\t{:?} ",q.clone());
        // fetch next base
        let x = q[0].clone();
        q = q[1..].to_vec();

        // get info for add-on
        let mut ni = (d.get_mut(&x).unwrap()).clone();
        let mut x3 = setf::str_to_vec(x.clone(),'+');

            // determine if terminate
        let slen = s.len() + 1;
    	let vil = valid_index_limit(ni[1] as i32, x3.len() as i32,
    		k as i32, slen as i32);

    	if !vil {
    		continue;
    	}

        while x3.len() < k {
    		let x32 = x3.clone();

    		// UPDATE ELEMENT
    		let n = s[ni[1]].clone();
    		x3.push(n);

    		// UPDATE OLD KEY
    		let ok = setf::vec_to_str(x32,'+');
	        let mut nok = (d.get_mut(&(ok.to_string())).unwrap()).clone();
    		nok[1] = nok[1] + 1;
    		d.insert(ok.to_string(),nok);

    		// UPDATE NEW KEY
    		let nk = setf::vec_to_str(x3.clone(),'+');
            let nk2 = nk.clone();
    			// case: new key does not exist
    		if !d.contains_key(&(nk.clone())) {
    			let ans = vec![ni[1],ni[1] + 1];
                if ni[1] + 1 >= s.len() {
                    continue;
                }
    			d.insert(nk.clone(),ans.clone());
    		} else {// case: new key exist
    			let mut ans = (d.get_mut(&(nk.clone())).unwrap()).clone();
    			ans[1] = ans[1] + 1;
                if ni[1] +1 >= s.len() {
                    continue;
                }
                d.insert(nk.clone(),ans.clone());
    		}

    		// UPDATE RELEVANT VARS
    		ni = (d.get_mut(&(nk.clone())).unwrap()).clone();

            if x3.len() >= k {
                continue;
            }
    		q.push(nk.to_string());
        }

        let nk = setf::vec_to_str(x3.clone(),'+');
        result.push(x3);
        q.push(x.clone());
   }
    result
}

/// collects all combinational sets of size k starting with s\[r\] in the subvector s\[r+1:\]
pub fn fcollect(s: Vec<String>, r: usize,k: usize) -> Vec<HashSet<String>> {
    let mut result: Vec<HashSet<String>> = Vec::new();

    // case: k == 1
    if k == 1 {
        let mut q2 = HashSet::new();
        q2.insert(s[r].clone());
        result.push(q2);
        return result;
    }

    // make a queue and a dictionary
    // value == <index of last element, valid index for next>
    let mut d: HashMap<String,Vec<usize>> = HashMap::new();
    let mut q: Vec<String> = Vec::new();

    // initialize the queue and dictionary
    let d2: Vec<String> = vec![s[r].clone()];
    let d3 = vec![r,r+1];
    d.insert(setf::vec_to_str(d2,'_'),d3);
    q.push(s[r].clone());
    while q.len() > 0 {
        // fetch next base
        let x = q[0].clone();
        q = q[1..].to_vec();

        // get info for add-on
        let mut ni = (d.get_mut(&x).unwrap()).clone();
        let mut x3 = setf::str_to_vec(x.clone(),'_');

            // determine if terminate
        let slen = s.len() + 1;
    	let vil = valid_index_limit(ni[1] as i32, x3.len() as i32,
    		k as i32, slen as i32);

    	if !vil {
    		continue;
    	}

        while x3.len() < k {
    		let x32 = x3.clone();

    		// UPDATE ELEMENT
    		let n = s[ni[1]].clone();
    		x3.push(n);

    		// UPDATE OLD KEY
    		let ok = setf::vec_to_str(x32,'_');
	        let mut nok = (d.get_mut(&(ok.to_string())).unwrap()).clone();
    		nok[1] = nok[1] + 1;
    		d.insert(ok.to_string(),nok);

    		// UPDATE NEW KEY
    		let nk = setf::vec_to_str(x3.clone(),'_');
            let nk2 = nk.clone();

    			// case: new key does not exist
    		if !d.contains_key(&(nk.clone())) {
    			let ans = vec![ni[1],ni[1] + 1];
                if ni[1] + 1 >= s.len() {
                    continue;
                }
    			d.insert(nk.clone(),ans.clone());
    		} else {// case: new key exist
    			let mut ans = (d.get_mut(&(nk.clone())).unwrap()).clone();
    			ans[1] = ans[1] + 1;
                if ni[1] +1 >= s.len() {
                    continue;
                }
                d.insert(nk.clone(),ans.clone());
    		}

    		// UPDATE RELEVANT VARS
    		ni = (d.get_mut(&(nk.clone())).unwrap()).clone();

            if x3.len() >= k {
                continue;
            }
    		q.push(nk.to_string());
        }

        let nk = setf::vec_to_str(x3.clone(),'_');
        let h: HashSet<String> = x3.into_iter().collect();
        result.push(h);
        q.push(x.clone());
   }
    result
}

/// orders the vector `v2` by the vector `reference`; for remaining elements of `v2`
/// not in `reference, method orders them according to `strng_srt::sort_inc1string_vector`. 
pub fn ordered_vec_by_reference<T>(v2:Vec<T>,reference:Vec<T>) -> Vec<String>
where
T:ToString + Clone,
{
    let href: HashSet<String> = reference.iter().map(|r| (*r).to_string()).collect();
    assert_eq!(reference.len(),href.len());

    // make a hash set for v2
    let mut cache: Vec<String> = Vec::new();
    let mut hm: HashMap<String,usize> = HashMap::new();

    // reference first
    for v2_ in v2.iter() {
        let q = (*v2_).to_string();
        if !href.contains(&q) {
            cache.push(q);
        } else {
            if !hm.contains_key(&q) {
                hm.insert(q.clone(),1);
            } else {
                let xy = *(hm.get_mut(&q).unwrap()) + 1;
                hm.insert(q.clone(),xy);
            }
        }
    }

    let mut sol: Vec<String> = Vec::new();
    for l in reference.iter() {
        let l_ = (*l).to_string();

        if hm.contains_key(&l_) {
            let x = hm.get(&l_).unwrap();
            for x_ in 0..*x {
                sol.push((l_).to_string())
            }
        }
    }

    strng_srt::sort_inc1string_vector(&mut cache);
    sol.extend_from_slice(&cache);

    sol

}


/// method used for determining validity of index for `fcollect` methods
/// 
/// i := index at
/// r := present length
/// k := wanted length
/// l := length of entire sequence
pub fn valid_index_limit(i: i32, r: i32, k: i32, l:i32) -> bool {

    if i > l - 1 {
        return false;
    }

    if i + k - r < l {
        return true;
    }

    false
}


///////////////////////////////////////////////////

/// outputs a k-vector with all elements equal to `v`. 
pub fn identity_vector(k: usize,v:usize) ->Vec<usize> {

    let mut x: Vec<usize> = Vec::new();
    for r2 in 0..k {
        x.push(v);
    }
    x
}

/// outputs the vector map for restriction|requirement
pub fn identity_k_vector_map(k:usize, rs: usize) -> Vec<(usize,Vec<usize>)> {
    let mut vm: Vec<(usize,Vec<usize>)> = Vec::new();
    let idv: Vec<usize> = Array::range(0.0,(rs) as f64, 1.0).into_iter().map(|x| x as usize).collect();
    for r in 0..k {
        vm.push((r,idv.clone()));
    }
    vm
}

/// sample SelectionRule; unused. 
/// restriction and requirement is all
pub fn selection_rule_sample_1() -> SelectionRule {
    let rs:usize = 10;
    let k:usize = 6;

    let rest:Vec<(usize,Vec<usize>)> = identity_k_vector_map(k,rs);
    let rst = selection_rule::build_restriction(rs,rest.clone(),k);
    let req = selection_rule::build_requirement(rs,rest,k);
    let sr = selection_rule::SelectionRule{res:rst,req:req,
        choice:Vec::new()};
    sr
}

/// sample SelectionRule; unused. 
/// requirement is all, restriction is none
pub fn selection_rule_sample_2() -> SelectionRule {
    let rs:usize = 3;
    let k:usize = 3;

    let rq:Vec<(usize,Vec<usize>)> = identity_k_vector_map(k,rs);
    let rest: Vec<(usize,Vec<usize>)> = Vec::new();

    let rst = selection_rule::build_restriction(rs,rest.clone(),k);
    let req = selection_rule::build_requirement(rs,rq,k);

    let sr = selection_rule::SelectionRule{res:rst,req:req,
        choice:Vec::new()};
    sr
}

/// sample SelectionRule; unused. 
pub fn selection_rule_sample_3() -> SelectionRule {
    let rs:usize = 10;
    let k:usize = 6;

    let rq:Vec<(usize,Vec<usize>)> = identity_k_vector_map(k,rs);
    let rest: Vec<(usize,Vec<usize>)> = Vec::new();

    let rst = selection_rule::build_restriction(rs,rest.clone(),k);
    let req = selection_rule::build_requirement(rs,rq,k);

    let sr = selection_rule::SelectionRule{res:rst,req:req,
        choice:Vec::new()};
    sr
}

///////////////////////////////////////////////////


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fcollect() {
        let mut x = vec!["a".to_string(), "ar".to_string(), "bxx".to_string(),
            "d".to_string(), "dr".to_string(), "dxx".to_string()];

        let mut y = strng_srt::stringized_srted_vec(&mut x);
        let mut s = fcollect(x,0,3);
        assert_eq!(s.len(),10);
    }


    #[test]
    fn test_SGen_fcollect() {
        // case 1
        let mut value = vec!["arbitrox".to_string(), "bartinuell".to_string(),
            "radinox".to_string(), "reskeyiazam".to_string(),"garbitol".to_string(),
            "weinfarsitol".to_string()];

        let mut sg = SGen{value:value,data:Vec::new(),
                                    next:Vec::new()};
        sg.fcollect_all(3);

        for x in sg.data.iter() {
            let mut v: Vec<String> = Vec::from_iter((*x).clone());
            let mut v2 = strng_srt::stringized_srted_vec(&mut v);
            println!("");
        }
        assert_eq!(20,sg.data.len());

        // case 2
        let q = vec!["a".to_string(),"b".to_string(),"a".to_string(),
        "b".to_string()];
        let mut sg = SGen{value:q,data:Vec::new(),next:Vec::new()};

        sg.fcollect_all(3);
        assert!(sg.data.len() == 0);
    }

    #[test]
    fn test_ordered_vec_by_reference() {
        let mut x1: Vec<i32> = vec![120,140,3000,34,54,61,1,31,-2];
        let mut x2: Vec<i32> = vec![-2,1,31,54,34];
        let mut s1 = ordered_vec_by_reference(x1,x2.clone());

        let mut qsw = vec![-2,1,31,54,34,61,140,120,3000];
        let mut sol = setf::vec_to_str(qsw,'_');
        let mut s1s = setf::vec_to_str(s1,'_');
        assert_eq!(sol,s1s);

        let mut x1_: Vec<i32> = vec![120,140,31,3000,34,-2,54,61,1,31,-2];
        let mut s2 = ordered_vec_by_reference(x1_,x2);
        let mut s2s = setf::vec_to_str(s2,'_');
        let mut sol2 = "-2_-2_1_31_31_54_34_61_140_120_3000".to_string();
        assert_eq!(sol2,s2s);
    }

    #[test]
    fn test_sr_op() {
        let (mut rs,mut rq) = selection_rule::test_rule_contents_2();
        let mut sr = selection_rule::SelectionRule{res:rs,req:rq,choice:Vec::new()};
        let mut c = 0;
        while true {
            let (mut x1,mut x2) = sr_op(&mut sr,c);
            if !x2 {
                break;
            }

            let ch = sr.select_choice_at_col_index(x1,c,true);

            c += 1;
            if c >= 6 {
                break;
            }

        }
        assert!(c >=3);
    }

}
