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
use enci::skew::Skew;
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

fn main() {

    // test std_rng
    /*
    let mut v: Vec<f32> = Vec::new();
    for i in 0..100 {
        let x = enci::std_rng::random_i32_in_range((0,100));
        //println!("PUSHING {}",x);
        v.push(x as f32);
    }
    println!("---");
    let q = enci::demo_rng::default_FrqncCl(v,(0.,100.));
    println!("Q: {}",q.analysis);
    */
    ////////////
    /*
    let mut s = enci::ns::Si32{r:(2,100)};
    for _ in 0..100 {
        let q = s.r_next();
        println!("Q: {}",q);
    }
    */

    //////
    /*
    let mut s = setti::ssi::build_SSI(10,5);
    while !s.is_finished() {
        println!("{:?}",s.v);
        s.next();
    }
    */
    //////

    ///// CAUTION: will have to scale f32 to i32
    /*
    //let mut s = enci::ns::Si32{r:(2,100)};
    let mut s = enci::ns::Sf32{r:(2.,100.)};
    let r = (2.,100.);
    let n = 10;
    let srate = 2;
    let ar = (0.,1.);
    let mut rt = enci::demo_rng::build_RTest1(Box::new(s),r,n,srate,ar);
    rt.run();
    println!("------");
    println!("{:?}",rt.data);
    */
    /////////////////////////////////////////////////////////////////////
    //// practice w/ selection rule
    /*
    let mut x: Array2<i32> = Array2::zeros((5,4));
    let res: selection_rule::Restriction = selection_rule::Restriction{data:x.clone()};
    let req: selection_rule::Requirement = selection_rule::Requirement{data:-1 * x.clone()};
    let mut srs: selection_rule::SelectionRule = selection_rule::SelectionRule{res:res,req:req,choice:Vec::new()};
    
    for i in 0..4 {
        let c = srs.choices_at_col_index(i);
        println!("{:?}",c);
    }
    */
    //////////////////////////////////
    /*
    let mut c = vec![3,4,10,12];
    let mut n = 12;
    pub fn next_available_forward(c,n:usize,distance:usize) -> Option<Vec<usize>> {
    */
    ////////////////////////////////////////////////////////
    
    let cvec = vec![3,4,5,6,7];
    let mut si = setti::set_imp::build_set_imp(cvec,3,0.5);
    while !si.finished {
        let sin = si.next();
        println!("{:?}",sin);
    }
    
    ////////////////////////////////////////////////////////
    /*
    let a1:Array1<f32> = arr1(&[3.0,0.3333,0.45,0.55,0.1]); 
    let a2:Array1<f32> = arr1(&[0.5,0.5,0.2,2.]); 
    let a3:Array1<f32> = arr1(&[5.,0.7]); 
    let a4:Array1<f32> = arr1(&[0.8,0.3,3.0]); 
    let b = vec![a1,a2,a3,a4];
    
    let f = "src/data/i_f1.csv";
    //metrice::vcsv::arr1_seq_to_csv(b,f,"w");
    let vv = vec![0.2,0.4,0.5,0.6];
    let qr = (0.,1.);
    let mut ifs = setti::impf::file_to_ImpFSeq(f,vv,qr);

    let mut res:Vec<f32> = Vec::new();
    for i in 0..4 {
        let q = round(ifs.next(i) as f64,5) as f32;
        res.push(q); 
        //println!("{:?}",ifs.next(i));
    }

    println!("{:?}",res);
    assert_eq!(res,vec![0.6, 0.2, 1.0, 0.48]);
    */
    /////////////////////////////////////////////////////////
}
