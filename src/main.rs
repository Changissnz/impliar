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
    //// demo 1: prediction
    /*
    let vr = metrice::vreducer::sample_vred_euclids_reducer_tail1();
    let mut gj = metrice::gorillaj::build_GorillaJudge("src/data/f3_x.csv".to_string(),None,
        false,vr.clone(),2,20); 
    gj.process_next(false);
    
    let x = metrice::vcsv::csv_to_arr1_seq("src/data/f3_x.csv").unwrap();
    let y = metrice::vcsv::csv_to_arr1_seq("src/data/f3_y.csv").unwrap();

    let l = x.len();

    for i in 0..l {
        let p = gj.predict_sequence(x[i].clone());
        println!("{}",p);
        println!("actual: {:?}\n",y[i].clone());
    }
    */

    //// demo 2: repeatedly refactor until no longer able to.
    let vr = metrice::vreducer::sample_vred_euclids_reducer();
    let mut gj = metrice::gorillaj::build_GorillaJudge("src/data/f3_x.csv".to_string(),Some("src/data/f3_y3.csv".to_string()),
        true,vr.clone(),2,20); 

    gj.process_next(false);
    gj.skew_meter();
    println!("---");
    
    println!("bc1: {}",gj.bc.refn.len());
    println!("bc2: {}",gj.bc2.refn.len());
    
    gj.refactor();
    gj.skew_meter();
    println!("---");

    gj.refactor();
    gj.skew_meter();
}