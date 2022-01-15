//extern crate itertools;
#[allow(dead_code)]

use std::collections::HashSet;
use std::collections::HashMap;
use std::vec;
use std::any::type_name;
mod setti;
use setti::setf::Count;
use substring::Substring;
use factorial::Factorial;
use asciis::asc::Asciis;
use ndarray::array;
use ndarray::{Array2, Array1,arr1,arr2,arr3, stack,s,Axis,Dim};

use std::option;

fn main() {
    /*
    let mut ax1 : Array2<i32> = Array2::zeros((5, 4));
    let mut ax2: Array1<i32> = arr1(&[0,0,1,1]);
    /*
    let mut a_row = ax1.slice_mut(s![1, ..]);
    a_row.assign(&ax2);

    println!("a = {:?}", ax1);
    */
    setti::matrixf::replace_vec_in_arr2(&mut ax1,&mut ax2,0,true);
    println!("AFTER: {}",ax1);
    */
    //////////
    /*
    let mut a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let mut b = arr2(&[[7, 8, 9], [10, 11, 12], [13, 14, 15]]);
    let mut a_row = a.slice_mut(s![1, ..]);
    let mut b_row = b.slice_mut(s![2, ..]);
    let tmp = a_row.to_owned();
    a_row.assign(&b_row);
    b_row.assign(&tmp);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    let mut qd = a.raw_dim();
    println!("DIM = {}",qd[0]);//,a.raw_dim().to_string());

    let mut c = b.slice_mut(s![0..3,..]);
    println!("c = {}", c);
    */
    /////////////
    let mut x1: Vec<i32> = vec![120,140,31,3000,34,-2,54,61,1,31,-2];
    let mut x2: Vec<i32> = vec![-2,1,31,54,34];
    let mut s1 = setti::set_gen::ordered_vec_by_reference(x1,x2);

    let mut qsw = vec![-2,1,31,54,34,61,140,120,3000];
    let mut sr = setti::setf::vec_to_str(qsw);
    let mut s1s = setti::setf::vec_to_str(s1);
    println!("{}",s1s);
    //assert_eq!(sr,s1s);
    /*
    for s1_ in s1.iter() {
        println!("{}",s1_);
    }
    */

    ///////
    /*
    let asc = Asciis{};
    let mut r:i32 = asc.ord("0").unwrap();
    let mut r2:i32 = asc.ord("9").unwrap();
    println!("{}|{}",r,r2);
    */
}
