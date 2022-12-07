#[allow(dead_code)]
#[allow(non_snake_case)]
mod setti;
#[allow(non_snake_case)]
mod enci;
#[allow(non_snake_case)]
mod metrice;
//mod pbobot;

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

use std::io::Read;

use crate::setti::strng_srt;
use crate::setti::selection_rule;
use crate::setti::selection_rule::Requirement;
use crate::setti::selection_rule::Restriction;
use crate::setti::vs;
use crate::setti::uvs;
use crate::setti::ds_fob_c;
use crate::setti::ds_fob_c::NE;
use crate::enci::be_int;
use crate::setti::disinc;

use setti::selection_rule::std_collision_score;
use setti::selection_rule::SelectionRule;
use enci::skew::{build_skew,Skew};
use enci::skewf32;

use std::mem;
use std::str::FromStr;

extern crate round;
use round::{round, round_up, round_down};

use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex, Once};
use std::env;
use rand::prelude::*;
use crate::enci::ns::RN;

#[macro_use]
extern crate savefile_derive;
extern crate savefile;
use savefile::prelude::*;

fn main() {

    ///////
    /*
    let x0 = metrice::vcsv::csv_to_arr1_seq("src/data/f3_x.csv").unwrap();
    let x = metrice::vcsv::csv_to_arr1("src/data/f3_y.csv").unwrap();

    // case 1
    let t0 = x0[0].clone();
    let t1 = x[0].clone() as usize;

    let sv1: Vec<metrice::vreducer::FCast> = vec![metrice::vreducer::FCast{f:metrice::vreducer::std_euclids_reducer}];

    let mut vr21 = metrice::vreducer::build_VRed(sv1.clone(),Vec::new(),vec![0,1],
                0,Some(metrice::vreducer::FCastF32{f:metrice::gorillains::f9,ai:0.}),None);

    let l = x0.len();
    for i in 0..l {
        println!("APPLY {}: {:?}",i,vr21.apply(x0[i].clone(),0).0);
    }
    */

}