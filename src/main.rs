//extern crate itertools;
#[allow(dead_code)]
mod setti;
mod enci;

use std::collections::HashSet;
use std::collections::HashMap;
use std::vec;
use std::any::type_name;
use setti::setf::Count;
use substring::Substring;
use factorial::Factorial;
use asciis::asc::Asciis;
use ndarray::array;
use ndarray::{Array2, Array1,arr1,arr2,arr3, stack,s,Axis,Dim};
use crate::setti::selection_rule::Requirement;
use crate::setti::selection_rule::Restriction;
use setti::selection_rule::std_collision_score;
use setti::selection_rule::SelectionRule;
use enci::skew::Skew;

fn main() {

    let (x1,x2) = enci::fatorx::sample_arr1_pair_3();
    //let mut skew_search_ordering: Vec<Vec<usize>> = vec![vec![1]];

    let q = enci::fatorx::max_satisfying_mult_additive_for_vec(x2.clone(),x1.clone());
    println!("{}",q);
}
