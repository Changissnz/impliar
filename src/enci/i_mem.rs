/*
memory structure for interpolator
*/

use ndarray::{Array,Array1,Array2,arr1,arr2,s};
use std::collections::{HashMap,HashSet};

pub struct IMem {
    pub soln_log:Vec<Array1<Option<f32>>>,
    pub contradiction_log: Vec<(usize,HashSet<usize>)>
}

// soln difference(t1,t2) -> (+,set),(-,set)
