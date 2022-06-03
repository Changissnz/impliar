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
/////////

use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::fs::OpenOptions;

fn main() {


}
