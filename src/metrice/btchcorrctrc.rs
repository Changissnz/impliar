/*
batch correcting algorithm used for factorization
of skews
*/
extern crate round;
use round::round;

use crate::setti::dessi;
use crate::enci::skewf32;
use crate::enci::skew;
use crate::enci::fatorx;
use ndarray::{arr1,Array1};
use std::cmp::Ordering;
use std::collections::HashSet;


/////////////////////////////////////////////// multiplication
/*
pub fn best_mfactor_for_skew_batch_type_a(sb: Vec<skewf32::SkewF32>,reference:Vec<Array1<f32>>) {
    // scale values first
}
*/

pub fn i32_pair_cmp1(s1: &(i32,i32),s2: &(i32,i32)) -> std::cmp::Ordering {
    if (*s1).1 <= (*s2).1 {
        return Ordering::Less;
    }
    Ordering::Greater
}

// option (best multiple,score), (multiples,scores)
pub fn multiple_score_pair_vec_on_skew_batch_type_a(sb: Vec<skew::Skew>,reference:Vec<Array1<i32>>) -> Vec<(i32,i32)> {
    let l = reference.len();
    assert_eq!(l,sb.len());

    let mut vm_: HashSet<i32> = HashSet::new();
    for i in 0..l {
        let y2 = fatorx::cheapest_multiple_vec(reference[i].clone(),sb[i].addit.clone().unwrap());
        vm_.extend(y2.into_iter());
    }
    vm_.insert(0);
    let mut vm: Vec<i32> = vm_.into_iter().collect();

    // sort vm by distance to mean
    let l2 = vm.len() as f32;
    let mn:i32 = (vm.clone().into_iter().sum::<i32>() as f32 / l2).round() as i32;

    let mut vm2: Vec<(i32,i32)> = Vec::new();
    for v in vm.into_iter() {
        vm2.push((v, (v - mn).abs()));
    }
    vm2.sort_by(i32_pair_cmp1);

    // iterate through and get the one with the best score
    let mut bms:Vec<(i32,i32)> = Vec::new();
    for (m,_) in vm2.into_iter() {
        let (v1,v2) = m_refactor_skew_batch_type_a(sb.clone(),reference.clone(),m);
        let s12:i32 = v2.into_iter().map(|x| x.skew_size as i32).into_iter().sum::<i32>() + m;
        bms.push((m,s12));
    }
    bms
}


pub fn best_multiple_for_skew_batch_type_a(sb: Vec<skew::Skew>,reference:Vec<Array1<i32>>) -> (i32,i32) {
    // multiples,scores
    let smms = multiple_score_pair_vec_on_skew_batch_type_a(sb,reference);
    let l = smms.len();
    let i:usize = (0..l).into_iter().fold(0,|b,b2| if smms[b].1 < smms[b2].1 {b} else {b2});
    smms[i]
}

/*
m-refactor of a skew batch

outputs
(Skew{multer},Vec<Skew{addit}>)
*/
pub fn m_refactor_skew_batch_type_a(sb: Vec<skew::Skew>,reference:Vec<Array1<i32>>,m:i32) ->
    (skew::Skew, Vec<skew::Skew>) {

    let mut sol: Vec<skew::Skew> = Vec::new();
    let l = sb.len();
    for i in 0..l {
        let r2 = reference[i].clone() * m;
        let a2 = sb[i].addit.clone().unwrap() - r2;
        let sk = skew::build_skew(None,None,Some(a2),None,vec![2],None);
        sol.push(sk);
    }
    let sk1 = skew::build_skew(None,Some(m),None,None,vec![1],None);
    (sk1,sol)
}

////////////////////////////////////////////// addition

pub fn adder_score_pair_vec_on_skew_batch_type_a(sb: Vec<skewf32::SkewF32>) -> (Vec<(i32,f32)>,usize) {
    let (mut ta2,mut k) = scale_skewf32_batch_type_a(sb.clone());
    let (m1,m4,mn) = min_max_mean_of_skew_batch_type_a(ta2.clone());

    // initial score
    let ins:f32 = sb.iter().map(|x| x.clone().skew_size()).into_iter().sum();

    // get others
    let (m1,m4,mn) = min_max_mean_of_skew_batch_type_a(ta2.clone());
    let (vh1,vm1,s1) = a_refactor_skewf32_batch_type_a(ta2.clone(),k,m1);
    let (vh2,vm4,s2) = a_refactor_skewf32_batch_type_a(ta2.clone(),k,m4);
    let (vh3,vmn,s3) = a_refactor_skewf32_batch_type_a(ta2.clone(),k,mn);
    (vec![(0,ins),(m1,s1),(m4,s2),(mn,s3)],k)
}

/*
determines if  1|min|max|mean will reduce the cumulative score,
then outputs an a-factor and the refactored skews of sb.
*/
pub fn best_afactor_for_skewf32_batch_type_a(sb: Vec<skewf32::SkewF32>) -> (Option<skewf32::SkewF32>,Vec<skewf32::SkewF32>,f32) {

    let (q,k) = adder_score_pair_vec_on_skew_batch_type_a(sb.clone());
    let index:usize = (0..4).into_iter().fold(0,|i,j| if q[i].1 < q[j].1 {i} else {j});

    let (m,s) = q[index].clone();

    // double call
    let (mut ta2,mut k) = scale_skewf32_batch_type_a(sb.clone());
    let (s1,s2,s3) = a_refactor_skewf32_batch_type_a(ta2.clone(),k,m);
    (if m != 0 {Some(s1)} else {None},s2,s3)
}

pub fn a_refactor_skewf32_batch_type_a(vs: Vec<skew::Skew>, k:usize,head: i32) -> (skewf32::SkewF32,Vec<skewf32::SkewF32>,f32)  {
    let h1_ = skew::build_skew(Some(head),None,None,None,vec![0],None);
    let mut h1 = skewf32::SkewF32{sk:h1_.clone(),s:k};
    let (vsk,_) = refactor_skew_batch_type_a(h1_.clone(), vs.clone());
    let mut sfv = skew_to_skewf32_batch_type_a(vsk, k);
    let score:f32 = sfv.clone().into_iter().map(|mut x| x.skew_size()).into_iter().sum::<f32>() + h1.skew_size();
    (h1,sfv,score)
}

pub fn skew_to_skewf32_batch_type_a(sk: Vec<skew::Skew>, bs: usize) -> Vec<skewf32::SkewF32> {
    let mut sol: Vec<skewf32::SkewF32> = Vec::new();
    for (i,s) in sk.into_iter().enumerate() {
        let skf = skewf32::SkewF32{sk:s,s:bs};
        sol.push(skf);
    }
    sol
}


pub fn refactor_skew_batch_type_a(head: skew::Skew, sb: Vec<skew::Skew>) -> (Vec<skew::Skew>,usize) {
    let mut sol:Vec<skew::Skew> = Vec::new();
    let mut score:usize = 0;
    for s in sb.into_iter() {
        let mut s2 = s.clone();
        let a2:Array1<i32> = s2.addit.clone().unwrap() - head.adder.clone().unwrap();
        let mut sk2 = skew::build_skew(None,None,Some(a2),None,vec![2],None);
        score += sk2.skew_size;
        sol.push(sk2);
    }

    (sol,score)
}

pub fn scale_skewf32_batch_type_a(sb: Vec<skewf32::SkewF32>) -> (Vec<skew::Skew>,usize) {

    let l = sb.len();
    let mut sol: Vec<skew::Skew> = Vec::new();
    if l == 0 { return (sol,0);}

    let mut k:usize =  (0..l).into_iter().map(|x| sb[x].s.clone()).into_iter().max().unwrap();

    // iterate through and scale up each skew
    for i in 0..l {
        let d = k - sb[i].s;
        let mut x:i32 = i32::pow(10,d as u32);
        let mut r2:Array1<i32> = sb[i].sk.addit.clone().unwrap() * x;
        let s2:skew::Skew = skew::build_skew(None,None,Some(r2),None,vec![2],None);
        sol.push(s2);
    }

    (sol,k)
}

pub fn min_max_mean_of_skew_batch_type_a(sb: Vec<skew::Skew>) -> (i32,i32,i32) {
    let l = sb.len();
    assert!(l > 0);

    // iterate through and collect all addit values
    let mut coll: Vec<i32> = Vec::new();
    for s in sb.into_iter() {
        coll.extend(s.addit.clone().unwrap().into_iter());
    }

    // calculate values
    let m1 = coll.clone().iter().fold(i32::MAX, |ans,&x| if ans.abs() < x.abs() {ans} else {x});//min().unwrap();
    let m4 = coll.clone().iter().fold(0 as i32, |ans,&x| if ans.abs() > x.abs() {ans} else {x});
    let mn = (coll.iter().sum::<i32>() as f32 / l as f32).round() as i32;
    (m1,m4,mn)
}

pub fn k_scale_of_arr1f32_vec(va: Vec<Array1<f32>>,opt_max: Option<usize>) -> usize {
    let mut q: usize = 0;
    for v in va.into_iter() {
        assert!(v.len() > 0);
        let maxi: usize = v.into_iter().map(|x| dessi::f32_decimal_length(x,opt_max.clone())).into_iter().max().unwrap();
        if maxi > q {q = maxi;}
    }
    q
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/*
0.001,0.012,0.024,0.016,0.048
0.0004,0.004,0.00023,-0.00412
1.3,0.3,0.1,-2.3,2.
*/
pub fn afactor_test_case_1() -> Vec<skewf32::SkewF32> {

    let s1_: Array1<i32> = arr1(&[10,12,24,16,48]);
    let s1 = skew::build_skew(None,None,Some(s1_),None,vec![2],None);
    let sf1 = skewf32::SkewF32{sk:s1,s:3};

    let s2_: Array1<i32> = arr1(&[40,400,23,-412]);
    let s2 = skew::build_skew(None,None,Some(s2_),None,vec![2],None);
    let sf2 = skewf32::SkewF32{sk:s2,s:5};

    let s3_: Array1<i32> = arr1(&[13,3,1,-23,20]);
    let s3 = skew::build_skew(None,None,Some(s3_),None,vec![2],None);
    let sf3 = skewf32::SkewF32{sk:s3,s:1};
    vec![sf1,sf2,sf3]
}

pub fn mfactor_test_case_1() -> (Vec<skew::Skew>,Vec<Array1<i32>>) {
    let s1 = skew::build_skew(None,None,Some(arr1(&[32,20,26,3,81])),None,vec![2],None);
    let s2 = skew::build_skew(None,None,Some(arr1(&[9,9,9,9])),None,vec![2],None);
    let s3 = skew::build_skew(None,None,Some(arr1(&[45,47])),None,vec![2],None);
    let s4 = skew::build_skew(None,None,Some(arr1(&[10,11,12,13])),None,vec![2],None);

    let v1:Array1<i32> = arr1(&[1,3,4,15,20]);
    let v2:Array1<i32> = arr1(&[4,5,15,16]);
    let v3:Array1<i32> = arr1(&[22,12]);
    let v4:Array1<i32> = arr1(&[1,2,1,2]);

    (vec![s1,s2,s3,s4], vec![v1,v2,v3,v4])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__GorillaIns__scale_skewf32_batch_type_a() {
        let ta = afactor_test_case_1();
        let ps = ta.clone().into_iter().map(|mut x| x.skew_size()).into_iter().sum::<f32>();
        let (mut ta2,mut k) = scale_skewf32_batch_type_a(ta.clone());
        assert_eq!(ta2[0].to_string(),"+[1000, 1200, 2400, 1600, 4800]".to_string());
        assert_eq!(ta2[1].to_string(),"+[40, 400, 23, -412]".to_string());
        assert_eq!(ta2[2].to_string(),"+[130000, 30000, 10000, -230000, 200000]".to_string());
    }

    #[test]
    pub fn test__GorillaIns__refactor_skew_batch_type_a() {
        let ta = afactor_test_case_1();
        let ps = ta.clone().into_iter().map(|mut x| x.skew_size()).into_iter().sum::<f32>();
        let (mut ta2,mut k) = scale_skewf32_batch_type_a(ta.clone());

        let h1_ = skew::build_skew(Some(400),None,None,None,vec![0],None);
        let mut h1 = skewf32::SkewF32{sk:h1_.clone(),s:k};
        let (vsk,_) = refactor_skew_batch_type_a(h1_.clone(), ta2);
        let vs: Vec<usize> = ta.into_iter().map(|x| x.s).collect();
        let mut sfv = skew_to_skewf32_batch_type_a(vsk, k);
        let ps2:f32 = sfv.clone().into_iter().map(|mut x| x.skew_size()).into_iter().sum::<f32>();
        assert!(ps2 < ps);
    }

    #[test]
    pub fn test__best_afactor_for_skewf32_batch_type_a() {
        let ta = afactor_test_case_1();
        let (h,sb,_) = best_afactor_for_skewf32_batch_type_a(ta);

        assert!(!h.is_none());
        assert_eq!(h.unwrap().to_string(),"skalos 5+23".to_string());
    }

    #[test]
    pub fn test__best_multiple_for_skew_batch_type_a() {
        let (x1,x2) = mfactor_test_case_1();
        let (m,_) = best_multiple_for_skew_batch_type_a(x1,x2);
        assert_eq!(m,2);
    }
}
