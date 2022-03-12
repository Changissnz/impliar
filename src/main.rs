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

use crate::setti::vs;
use crate::setti::uvs;

use setti::selection_rule::std_collision_score;
use setti::selection_rule::SelectionRule;
use enci::skew::Skew;
use std::mem;
use std::str::FromStr;

fn main() {

    // test DSBGen
    let o :Vec<usize> = (0..20).into_iter().collect();
    let mut b = setti::ds_fob_c::build_DSBGen(20,5,2,2,o);

    while b.stat {
        b.next();
        //println!("{}",b.c);
    }

    println!("{}",b.c);
    assert_eq!(b.c,16);

}
