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
//////////////////////////////////////////////////////////////////////////////////////////
use ndarray::array;
use ndarray::{Array2, Array1,arr1,arr2,arr3, stack,s,Axis,Dim};

use std::option;

fn main() {
    //let mut sol : Vec<T> = indices.iter().map(|i| fn(v[i])).collect();

        // mapping function on s.v.
        let mut axx : Vec<u32> = vec![1,2,43,56,6111];
        let mut modI:Vec<usize> = vec![0,2];
        modI.iter().map(|i| {axx[*i] = 0;});//axx[*i] * 2;});

        for x in axx.iter() {
            println!("X: {}",x);
        }

        let mut q = setti::matrixf::map_function_on_subvector(&mut axx,|x| x + 3 + 3 * x ,modI,true);
        println!("-------------------------------");
        for x in axx.iter() {
            println!("X: {}",x);
        }
}
