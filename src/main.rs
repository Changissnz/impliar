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
use crate::setti::ds_fob_c;
use crate::setti::ds_fob_c::NE;
use crate::enci::be_int;

use setti::selection_rule::std_collision_score;
use setti::selection_rule::SelectionRule;
use enci::skew::Skew;
use std::mem;
use std::str::FromStr;

fn main() {

    // test out #1
    let (x,y):(Array2<f32>,Array1<f32>) = be_int::test_sample_BEInt_1();
    let mut bei = be_int::build_BEInt(x.clone(),y);
    let stat = bei.solve_at(x.dim().0 - 1,true,0);

    // print out i mem info.
    println!("IMEM");
    println!("{:?}",bei.im.soln_log);
    println!("CONTRA");
    for c in bei.im.contradiction_log.iter() {
        println!("{}",c);
    }
}
