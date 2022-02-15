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
}
