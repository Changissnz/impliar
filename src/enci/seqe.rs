use ndarray::{Array,Array1,arr1};
use crate::enci::skew::Skew;
use crate::enci::skew;
use crate::enci::fatorx;
use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;

pub fn skew_search_ordering() -> Vec<Vec<usize>> {
    vec![vec![0],vec![0,1],vec![0,1,2],vec![0,2],
    vec![0,3],vec![1],vec![1,0],vec![1,2],vec![1,3],
    vec![2],vec![3]]
}

/*
calculates a skew that can transform v1 to v2
*/
pub fn find_cheapest_skew(v1:Array1<i32>,v2:Array1<i32>) -> Skew {
    // collect all possible skews
    let sso = skew_search_ordering();
    let mut skews:Vec<Skew> = Vec::new();
    for si in sso.iter() {
        let (mut s1,mut s2):(Option<Skew>,Option<Skew>) = skews_special_case(v1.clone(),v2.clone(),si.clone());

        // check if skew satisfies
        if check_skew(s1.unwrap().clone(), v1.clone(),v2.clone()) {
            skews.push(s1.unwrap());
        }

        if !s2.is_none() {
            if check_skew(s2.unwrap().clone(), v1.clone(),v2.clone()) {
                skews.push(s2.unwrap());
            }
        }
    }

    //
    let s = skews[0].clone();
    skews.iter().fold(s, |min, val| if cheapest_skew_cost_function((*val).clone()) <
            cheapest_skew_cost_function(min.clone()) {val.clone()} else {min.clone()})
}

/*

skewInst is ordered set of usizes in [0,3];
0->adder,1->multer,2->addit,3->multit;

attempting skew for special cases:
(01,012): additionally attempts with
    = max_satisfying_mult_additive_for_vec
*/
pub fn skews_special_case(v1:Array1<i32>,v2:Array1<i32>,skewInst:Vec<usize>) -> (Option<Skew>,Option<Skew>) {
    let (mut s1, mut s2): (Option<Skew>,Option<Skew>) = (None,None);

    // make the arguments for the two skew cases
    let (mut v1_1,mut v1_2): (Array1<i32>,Array1<i32>) = (v1.clone(),v1.clone());

    let (mut a1,mut m1,mut a11,mut m11):(Option<i32>,Option<i32>,Option<Array1<i32>>,Option<Array1<i32>>) =
                            (None,None,None,None);
    let (mut a2,mut m2,mut a22,mut m22):(Option<i32>,Option<i32>,Option<Array1<i32>>,Option<Array1<i32>>) =
                            (None,None,None,None);


    for si in skewInst.iter() {
        if *si == 0 {
            let (mut vx1,mut vx2):(i32,i32) = (fatorx::max_satisfying_mult_additive_for_vec(v2.clone(),v1_1.clone()),
                                            fatorx::cheapest_add(v1_2.clone(),v2.clone()));
            a1 = Some(vx1);
            a2 = Some(vx2);

            // update v1_1,v1_2
            v1_1 = v1_1 + vx1;
            v1_2 = v1_2 + vx2;
        } else if *si == 1 {
            let (mut vx1,mut vx2):(i32,i32) = (fatorx::cheapest_multiple(v1_1.clone(),v2.clone()),
                                            fatorx::cheapest_multiple(v1_2.clone(),v2.clone()));
            m1 = Some(vx1);
            m2 = Some(vx2);

            // update v1_1,v1_2
            v1_1 = v1_1 * vx1;
            v1_2 = v1_2 * vx2;
        } else if *si == 2 {
            let (mut vx1_,mut vx2_):(Array1<i32>,Array1<i32>) = (fatorx::cheapest_add_vec(v1_1.clone(),v2.clone()),
                                            fatorx::cheapest_add_vec(v1_2.clone(),v2.clone()));
            a11 = Some(vx1_);
            a22 = Some(vx2_);
        } else {
            let (mut vx1_,mut vx2_):(Array1<i32>,Array1<i32>) = (fatorx::cheapest_multiple_vec(v1_1.clone(),v2.clone()),
                                            fatorx::cheapest_multiple_vec(v1_2.clone(),v2.clone()));
            m11 = Some(vx1_);
            m22 = Some(vx2_);
        }
    }

    let s1: Skew = skew::build_skew(a1,m1,a11,m11,skewInst.clone());
    let s2: Option<Skew> = if skewInst[0] == 0 {Some(skew::build_skew(a2,m2,a22,m22,skewInst.clone()))} else {None};
    return (Some(s1),s2)
}

pub fn check_skew(mut s:Skew,v1:Array1<i32>,v2:Array1<i32>) -> bool {
    s.skew_value(v1) == v2
}

//////////////////////////////////////////////////////////////////////////////

pub fn cheapest_skew_cost_function(s:Skew) -> f32 {
    let mut c: f32 = 0.0;

    if !s.adder.is_none() {
        let mut sm: i32 = s.adder.unwrap();
        c += sm as f32;
    }

    if !s.multer.is_none() {
        let mut sm: i32 = s.multer.unwrap();
        c += sm as f32;
    }

    if !s.addit.is_none() {
        let mut sm2: Array1<i32> = s.addit.unwrap();
        c += sm2.sum() as f32;
    }

    if !s.multit.is_none() {
        let mut sm2: Array1<i32> = s.multit.unwrap();
        c += sm2.sum() as f32;
    }

    c
}

pub struct SkewEncoder {
    pub skewChain: Vec<Skew>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
