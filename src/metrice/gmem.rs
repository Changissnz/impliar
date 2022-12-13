use crate::metrice::btchcorrctr;
use crate::metrice::vreducer;
use std::cmp::Ordering;
use std::collections::HashMap;
use ndarray::{arr1,Array1};
use crate::enci::{skewf32};

pub fn i32_3tuple_cmp1(s1: &(i32,f32,usize),s2: &(i32,f32,usize)) -> std::cmp::Ordering {
    if (*s1).1 <= (*s2).1 {
        return Ordering::Less;
    }
    Ordering::Greater
}

pub struct GMem {
    pub tailn_skew:Vec<skewf32::SkewF32>,
    pub vr_outputn: Vec<Array1<f32>>,
    pub base_vr: vreducer::VRed, 
    pub interval_ordering: Vec<usize>
}



