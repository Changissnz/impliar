use ndarray::{Array1,arr1};
use crate::setti::setf;
use crate::setti::setf::Count;

/*
compares the elements of two arr1's. the score is:

SUM(VectorCounter(a1) -  VectorCounter(a2)) * (1 - existence_weight) +
SUM(VectorCounter(a1.to_binary) -  VectorCounter(a2.to_binary)) * (existence_weight)

a1 := an arr1
a2 := an arr1
existence_weight := f32 in [0,1] that weighs the binary difference of a1 and a2.
*/
pub fn cmp_arr1_pair_1(a1:Array1<i32>,a2:Array1<i32>,existence_weight:f32) -> f32 {
    assert!(existence_weight >= 0. && existence_weight <= 1.);

    let mut v1:setf::VectorCounter = setf::build_VectorCounter();
    v1.countv(a1.into_iter().collect());

    let mut v2:setf::VectorCounter = setf::build_VectorCounter();
    v2.countv(a2.into_iter().collect());

    // get absolute difference
    let d1 = v1.clone() - v2.clone();
    let d1s:f32 =d1.data.into_iter().fold(0.,|acc,s| acc + s.1.abs() as f32);

        // normalize by sum of v1 and v2 values
    let v11:f32 = v1.data.clone().into_keys().fold(0.,|acc,s| acc + v1.data.get(&s).unwrap().clone() as f32);
    let v21:f32 = v2.data.clone().into_keys().fold(0.,|acc,s| acc + v2.data.get(&s).unwrap().clone() as f32);
    let s1:f32 = d1s / ((v11 + v21) as f32);

    // existence
    v1.one_it();
    v2.one_it();

    let d2 = v1.clone() - v2.clone();
    let d2s = d2.data.len() as f32;
    let s2:f32 = d2s / ((v1.data.len() + v2.data.len()) as f32);
    ((1. - existence_weight) * s1) + (existence_weight * s2)
}

pub fn std_dev_arr1_f32(a1:Array1<f32>) -> f32 {
    if a1.len() == 0 {return 0.};
    let m = a1.clone().mean().unwrap();
    let a2:Array1<f32> = a1.into_iter().map(|x| x - m).collect();
    a2.mean().unwrap() / a2.len() as f32
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn test_cmp_arr1_pair1() -> (Array1<i32>,Array1<i32>) {
    (arr1(&[1,2]),arr1(&[1,2]))
}

pub fn test_cmp_arr1_pair2() -> (Array1<i32>,Array1<i32>) {
    (arr1(&[1,1,2]),arr1(&[1,2,1,1,2]))
}

pub fn test_cmp_arr1_pair3() -> (Array1<i32>,Array1<i32>) {
    (arr1(&[1,2,3,50,60,70]),arr1(&[4,7,9,50,60,70]))
}

pub fn test_cmp_arr1_pair4() -> (Array1<i32>,Array1<i32>) {
    (arr1(&[1,10,100,200]),arr1(&[100,10,1]))
}

pub fn test_cmp_arr1_pair5() -> (Array1<i32>,Array1<i32>) {
    (arr1(&[1,10,1,10]),arr1(&[2,22,2,22]))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cmp_arr1_pair_1() {

        // case: 5
        let (x,y) = test_cmp_arr1_pair5();
        let s51 = cmp_arr1_pair_1(x.clone(),y.clone(),0.5);
        let s52 = cmp_arr1_pair_1(x.clone(),y.clone(),0.0);
        let s53 = cmp_arr1_pair_1(x.clone(),y.clone(),1.);
        assert_eq!(s51,1.);
        assert_eq!(s52,1.);
        assert_eq!(s53,1.);

        // case: 4
        let (x,y) = test_cmp_arr1_pair5();
        let s41 = cmp_arr1_pair_1(x.clone(),y.clone(),0.5);
        let s42 = cmp_arr1_pair_1(x.clone(),y.clone(),0.0);
        let s43 = cmp_arr1_pair_1(x.clone(),y.clone(),1.);
        assert!(s51 == s52 && s52 == s53);

        // case: 2
        let (x,y) = test_cmp_arr1_pair2();
        let s21 = cmp_arr1_pair_1(x.clone(),y.clone(),0.5);
        let s22 = cmp_arr1_pair_1(x.clone(),y.clone(),0.0);
        let s23 = cmp_arr1_pair_1(x.clone(),y.clone(),1.);
        assert_eq!(s21,0.125);
        assert_eq!(s22,0.25);
        assert_eq!(s23,0.);

        // case: 3
        let (x,y) = test_cmp_arr1_pair3();
        let s31 = cmp_arr1_pair_1(x.clone(),y.clone(),0.5);
        let s32 = cmp_arr1_pair_1(x.clone(),y.clone(),0.0);
        let s33 = cmp_arr1_pair_1(x.clone(),y.clone(),1.);
        assert!(s31 == s32 && s32 == s33);

        // case: 1
        let (x,y) = test_cmp_arr1_pair1();
        let s11 = cmp_arr1_pair_1(x.clone(),y.clone(),0.5);
        let s12 = cmp_arr1_pair_1(x.clone(),y.clone(),0.0);
        let s13 = cmp_arr1_pair_1(x.clone(),y.clone(),1.);
        assert!(s11 == s12 && s12 == s13);
        assert!(s11 == 0.);

    }

}
