//extern crate itertools;
#[allow(dead_code)]
#[allow(non_snake_case)]
mod setti;
mod enci;
mod metrice;
mod pbobot;

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
    //setti::impf::sample_ImpFI32_save_to_file();

    
    let q = setti::setc::nCr(8,4);
    println!("Q: {}",q); 

    let mut i = setti::impli::sample_Impli_1();
    let v = i.output_ei_generated("abasco".to_string(),10); 
    println!("{:?}",v);

    let v2 = i.output_k_generated(10);
    println!("{:?}",v2);

    let v3 = i.output_options_ratio_generated(10);
    println!("{:?}",v3);

    let v4 = i.output_closure_ratio_generated(10);
    println!("{:?}",v4);

    i.output_statement(true);    
    println!("---");
    i.output_statement(true);
    println!("---");
    i.output_statement(true);
    i.ei_statement.summarize_at(0);
    i.ei_statement.summarize_at(1);
    i.ei_statement.summarize_at(2);
     
    //////////
}