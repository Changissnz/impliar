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
calculates a skew that can transform v1 to v2 by one of the following
orderings in `skew_search_ordering`.

CAUTION: this is a make-shift function and does not guarantee the cheapest skew.
         Parenthetical notation is missing.
         For example, the ordering (0 1 2) < ((0 1) 2) < (0 (1 2)) could happen.
*/
pub fn find_cheapest_skew(v1:Array1<i32>,v2:Array1<i32>) -> Skew {
    // collect all possible skews
    let sso = skew_search_ordering();
    let mut skews:Vec<Skew> = Vec::new();
    for si in sso.iter() {
        let (mut s1,mut s2):(Option<Skew>,Option<Skew>) = skews_special_case(v1.clone(),v2.clone(),si.clone());

        // check if skew satisfies
        if check_skew(s1.as_ref().unwrap().clone(), v1.clone(),v2.clone()) {
            skews.push(s1.unwrap().clone());
        }

        if !s2.is_none() {
            if check_skew(s2.as_ref().unwrap().clone(), v1.clone(),v2.clone()) {
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

    let s1: Skew = skew::build_skew(a1,m1,a11,m11,skewInst.clone(),None);
    let s2: Option<Skew> = if skewInst[0] == 0 {Some(skew::build_skew(a2,m2,a22,m22,skewInst.clone(),None))} else {None};
    return (Some(s1),s2)
}

pub fn check_skew(mut s:Skew,v1:Array1<i32>,v2:Array1<i32>) -> bool {
    s.skew_value(v1) == v2
}

//////////////////////////////////////////////////////////////////////////////

pub fn cheapest_skew_cost_function(s:Skew) -> f32 {
    let mut c: f32 = 0.0;

    if !s.adder.is_none() {
        let mut sm: i32 = s.adder.unwrap().abs();
        c += (sm + 1) as f32;
    }

    if !s.multer.is_none() {
        let mut sm: i32 = s.multer.unwrap().abs();
        c += (sm + 1) as f32;
    }

    if !s.addit.is_none() {
        let mut sm2: Array1<i32> = s.addit.unwrap().into_iter().map(|x| x.abs()).collect();
        c += sm2.sum() as f32;
        c += sm2.len() as f32;
    }

    if !s.multit.is_none() {
        let mut sm2: Array1<i32> = s.multit.unwrap().into_iter().map(|x| x.abs()).collect();
        c += sm2.sum() as f32;
        c += sm2.len() as f32;
    }

    c
}

pub struct SkewEncoder {
    pub skewChain: Vec<Skew>,
}

pub fn skew_vector_pair_case_1() -> (Array1<i32>,Array1<i32>) {
    (arr1(&[5,7,9]),arr1(&[15,21,27]))
}


pub fn skew_vector_pair_case_2() -> (Array1<i32>,Array1<i32>) {
    (arr1(&[5,7,9]),arr1(&[16,20,29]))
}


pub fn skew_test_case_1() -> (Option<Skew>,Option<Skew>) {
    let (mut v1,mut v2) = skew_vector_pair_case_1();
    skews_special_case(v1,v2,vec![1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skews_special_case() {
        let (mut s1,mut s2) = skew_test_case_1();

        let (mut v1,mut v2) = skew_vector_pair_case_1();

        assert_eq!(s1.is_none(),false);
        assert_eq!(s2.is_none(),true);

        let mut v3 = s1.unwrap().skew_value(v1.clone());
        assert_eq!(v3.clone(),v2.clone());
    }

    #[test]
    fn test_find_cheapest_skew() {
        let (mut s1,mut s2) = skew_test_case_1();
        let (mut v1,mut v2) = skew_vector_pair_case_1();
        let mut skew: Skew = find_cheapest_skew(v1.clone(),v2.clone());//,ordering.clone());
        assert_eq!(skew.to_string(),"*3".to_string());
        assert_eq!(4.0,cheapest_skew_cost_function(skew));

        let (mut v11,mut v12) = skew_vector_pair_case_2();
        let mut skew2: Skew = find_cheapest_skew(v11.clone(),v12.clone());//,ordering.clone());
        assert_eq!("*3+[1, -1, 2]".to_string(),skew2.to_string());
        assert_eq!(11.0,cheapest_skew_cost_function(skew2.clone()));
    }
}
