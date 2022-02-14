use ndarray::{Array,Array1,arr1};
use crate::enci::skew::Skew;
use crate::enci::skew;
use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;

// find cheapest IndexFractionNotation formation to replicate a sequence

/*
*/
/////////////////////////
/*
calculates a skew that can transform v1 to v2
*/
/*
pub fn find_cheapest_skew(v1:Array1<i32>,v2:Array1<i32>) -> Skew {
    // m,a
    //
}
*/

/*
skewInst is ordered set of usizes in [0,3];
0->adder,1->multer,2->addit,3->multit
*/
//////
/*
pub fn attempt_skew(v1:Array1<i32>,v2:Array1<i32>,skewInst:Vec<usize>) -> Option<Skew> {
    let (mut a,mut m,mut av,mut mv):(Option<i32>,Option<i32>,Option<Array1<i32>>,Option<Array1<i32>>) =
                            (None,None,None,None);

    for s in skewInst.iter() {


    }

    //pub fn cheapest_multiple(v1:Array1<i32>,v2:Array1<i32>); //-> i32 {
}
*/

//////////////////////////////////////////////////////////////////////////////

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
