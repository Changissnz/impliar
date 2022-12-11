use crate::metrice::btchcorrctr;
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
