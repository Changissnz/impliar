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

    // test DSBGen
    /*
    let o :Vec<usize> = (0..20).into_iter().collect();
    let mut b = setti::ds_fob_c::build_DSBGen(20,5,2,2,o);

    while b.stat {
        b.next();
    }

    println!("{}",b.c);
    assert_eq!(b.c,16);
    */

    ////////////////////////
    /*
    let
     stack![Axis(1), a, b] or stack(Axis(1), &[a.view(), b.view()])
     */
     ///////////////////////////////////////////////////////////

     /*
    let a = arr2(&[[2., 2.],
                   [3., 3.]]);

    let mut b = arr1(&[5.,10.]);

    let c = arr2(&[[2., 2.,4.5],
                   [3., 24.1,13.]]);
    b = a.row(0).to_owned();
    //b[0] = 11.;

    let mut h: HashSet<usize> = HashSet::new();
    h.insert(5);
    h.insert(10);
    h.insert(110);
    let mut h2: Vec<usize> = Vec::from_iter(h.clone().into_iter());

    let mut q = h.clone().into_iter().min().unwrap();
    //println!("{}",q);
    h.remove(&q);
    println!("{:?}",h);
    */
    ///////////////////////////////////////////////////////////////

    /*
    println!("{}",a);
    println!("-----------");
    println!("{}",b);
    println!("{}",a);
    */
    //b = append![Axis(0),b.clone(),b.clone()];

    /*
    let c = a.row(0).to_owned();
    */


    /*
    assert!(
        stack![Axis(0), a, a]
        == arr2(&[[2., 2.],
                  [3., 3.],
                  [2., 2.],
                  [3., 3.]])
    );
    */
    /*
    let c = stack![Axis(0), a, b];
    println!("{}",c);
    */

    ///////////////////////////////////////////////////

}
