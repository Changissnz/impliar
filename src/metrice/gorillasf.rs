////! basic gorilla test functions.
use crate::setti::vs;
use crate::setti::vs::VSelect;
use crate::setti::set_gen;
use crate::setti::setf;
use crate::setti::disinc;
use crate::enci::fatorx;
use crate::metrice::deviance;

use ndarray::{Array1,arr1};
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

/// # description 
/// implementation of euclid's algorithm on v by u.
/// # return
/// equal-sized vectors
/// - \[0\] multiple
/// - \[1\] additive
pub fn euclids_sequence(u:i32,v:i32) -> (Array1<i32>,Array1<i32>) {

    // get initial c and
    let mut c: i32 = v / u;
    let mut e: i32 = v - c * u;

    let (mut sol1,mut sol2): (Vec<i32>,Vec<i32>) = (vec![c.clone()],vec![e.clone()]);
    let mut v_: i32 = u.clone();
    let mut u_: i32 = e.clone();

    // while coefficient is not 0
    while c != 0 && e != 0 {
        // update c,e,v_,u_
        c = v_ / u_;
        e = v_ - c * u_;

        sol1.push(c);
        sol2.push(e);

        v_ = u_.clone();
        u_ = e.clone();
    }

    (sol1.into_iter().collect(),sol2.into_iter().collect())
}

/*
version used for struct<GMem>
*/
pub fn euclids_sequence_(v: Array1<i32>) -> Vec<Array1<i32>> {
    assert_eq!(v.len(),2);
    let es = euclids_sequence(v[0],v[1]);//.into_iter().collect()
    vec![es.0.clone(),es.1.clone()]
}

/*
calculates a "normal" measure of sequence i in a based on fn<deviance::cmp_arr1_pair_1>.

a := vector of arr1's that may be unequally sized.
*/
pub fn normal_measure_of_sequence(a:Vec<Array1<i32>>,i:usize,existence_weight:f32) -> f32 {
    let l = a.len();
    if l == 0 {
        return 1.;
    }

    let mut s:f32 = 0.;
    for j in 0..l {
        if j == i {
            continue;
        }

        s += deviance::cmp_arr1_pair_1(a[i].clone(),a[j].clone(),existence_weight.clone());
    }

    if l == 1 {
        return 1.;
    }

    s / ((l - 1) as f32)
}

/*
average of sequence of arr1 vec
*/
pub fn sequence_analysis_(a:Vec<Array1<i32>>,existence_weight:f32) -> f32 {
    let l = a.len();
    if l == 0 {return 1.;}

    let mut s: f32 = 0.;
    for i in 0..l {
        s += normal_measure_of_sequence(a.clone(),i,existence_weight.clone());
    }
    s / l as f32
}



/*
gorilla iterates through s by 2-permutations and outputs 2 arrays o1,o2 each
of equal size to s, with values in [0,1];0 is normal, 1 is not.

o1 := the vector of scores on the comparison between a coefficient and additive vector pair.
o2 := the vector of sequence analysis scores on the coefficient and additive
      vectors, separately.

uses Euclid's algorithm as mapping function
-------

*/
pub fn gorilla_touch_arr1_basic(s: Array1<i32>, existence_weight:f32) -> (Array1<f32>,Array1<f32>) {

    assert!(existence_weight >= 0. && existence_weight <= 1.);

    // collect all elements into c and a, vec<arr1>
    let l = s.len();
    let mut c:Vec<Array1<i32>> = Vec::new();
    let mut a:Vec<Array1<i32>> = Vec::new();
    let mut sol:Vec<f32> = Vec::new();
    let mut sol2:Vec<f32> = Vec::new();

    for i in 0..l {
        let h:HashSet<usize> = (0..l).into_iter().filter(|x| *x != i).collect();
        let hl = h.len();

        // collect all elements into c and a
        c = Vec::new();
        a = Vec::new();
        let mut s0: f32 = 0.;
        for h_ in h.into_iter() {
            let (v1,v2): (Array1<i32>,Array1<i32>) = euclids_sequence(s[i].clone(),s[h_].clone());
            c.push(v1.clone());
            a.push(v2.clone());

            // compare two sequences v1 and v2
            s0 += deviance::cmp_arr1_pair_1(v1,v2,existence_weight.clone());
        }
        if hl > 0 {
            s0 = s0 / hl as f32;
        }

        // perform analysis on c and a
        let mut s2:f32 = 0.;
        s2 += sequence_analysis_(c.clone(),existence_weight.clone());
        s2 += sequence_analysis_(a.clone(),existence_weight.clone());
        s2 /= 2.;

        // push the "randomness" measure of element i in s
        sol.push(s0);
        sol2.push(s2);
    }
    (sol.into_iter().collect(),sol2.into_iter().collect())
}

pub fn gorilla_touch_arr1_gcd(s: Array1<i32>, existence_weight:f32) -> Array1<f32> {

    pub fn gcdseq_of_i(s:Array1<i32>,i:usize) -> Array1<i32> {
        let l = s.len();
        let mut solo:Vec<i32> = Vec::new();
        for j in 0..l {
            if j == i {
                continue;
            }
            solo.push(fatorx::gcd_of_i32_pair(s[i].clone(),s[j].clone()));
        }
        solo.into_iter().collect()
    }

    // iterate through and collect each gcd sequence for element
    let mut sol: Vec<Array1<i32>> = Vec::new();

    // collect the gcd seq for each var i
    let l = s.len();
    for w in 0..l {
        sol.push(gcdseq_of_i(s.clone(),w));
    }

    // calculate the normal score for each element in s
    let nsvec:Array1<f32> = (0..l).into_iter().map(|i| normal_measure_of_sequence(sol.clone(),i,existence_weight.clone())).collect();
    nsvec
}

/////////////////////////////////////////////////////////////////////////////

pub fn test_sample_gorilla_touch_arr1_1() -> Array1<i32> {
    arr1(&[14,18,81131222,75121])
}

pub fn test_sample_gorilla_touch_arr1_2() -> Array1<i32> {
    arr1(&[2,4,8,16])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gorilla_touch_arr1_basic() {
        let s = test_sample_gorilla_touch_arr1_1();
        let ew:f32 = 0.5;
        let hs = gorilla_touch_arr1_basic(s, ew);

        let q1:Array1<f32> = arr1(&[0.56666666, 0.8592593, 1.0, 0.88383836]);
        let q2:Array1<f32> = arr1(&[0.33234128, 0.952381, 0.5, 0.8333334]);
        assert!(q1 == hs.0);
        assert!(q2 == hs.1);
    }

    #[test]
    fn test_gorilla_touch_arr1_gcd() {
        let s = test_sample_gorilla_touch_arr1_2();
        let ew:f32 = 0.5;
        let hs = gorilla_touch_arr1_gcd(s, ew);
        let q1:Array1<f32> = arr1(&[0.5555556, 0.34444442, 0.28333333, 0.28333333]);
        assert!(q1 == hs);
    }
}
