//extern crate itertools;
#[allow(dead_code)]
#[allow(non_snake_case)]
mod setti;
mod enci;
mod metrice;

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

/*
#[macro_use]
extern crate savefile_derive;


#[derive(Savefile)]
struct Player {
    name : String,
    strength : u32,
    inventory : Vec<String>,
}

fn save_player(player:&Player) {
    save_file("save.bin", 0, player).unwrap();
}

fn load_player() -> Player {
    load_file("save.bin", 0).unwrap()
}
*/ 
//////////////


fn main() {
    //impli::Impli{};

    //////////////////////////////////////
    // linear congruential generator
    /*
    let m:f32 = 4.5;
    let c:f32 = 7.;
    let l: f32 = 30.;
    let mut s:f32 = 2.;
    let mut i: usize = 10;
    while i > 0 {
        s = (s * m + c) % l;
        println!("{}",s);
    }
    */ 
    //////////////////////////////////////
    /*
    let x: Vec<String> = vec!["at".to_string(),"being".to_string(),"cat".to_string()];
    let x2: Vec<String> = vec!["at".to_string(),"bein".to_string(),"ca".to_string()];

    let mut vc1 = setti::setf::build_VectorCounter();

    vc1.countv(x);
    let mut vc2 = vc1.clone(); 
    vc2.countv(x2);
    let q = vc2 - vc1.clone(); 
    println!("{}",q);
    let q2 = vc1 - q; 
    println!("{}",q2);

    // to get new elements: 
    */ 
    ////////////////////////////////////////////
    /*
    let player = Player { name: "Steve".to_string(), strength: 42,
    inventory: vec!(
        "wallet".to_string(),
        "car keys".to_string(),
        "glasses".to_string())};

    save_player(&player);
    let reloaded_player = load_player();
    assert_eq!(reloaded_player.name,"Steve".to_string());
    */ 

    /*
    setti::impf::sample_ImpF_vec_e_1_save_to_file("ife".to_string());
    setti::impf::sample_ImpF_vec_i_1_save_to_file("ifi".to_string());
    */

    setti::impf::load_ImpF("ife1");
    
}