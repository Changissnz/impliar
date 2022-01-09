#![allow(dead_code)]
#![allow(unused_variables)]
/*
set functions
*/
pub use std::collections::HashSet;
pub use std::collections::HashMap;

//use core::fmt::Display;
use std::string::ToString;
//use std::string::Substring;
//use std::
use std::string::String;
use substring::Substring;

//use std::ops::Sub;
use std::fmt;
//use std::hash::Hash;

//use itertools::join;
//use std::marker::Copy;

pub struct VectorCounter {
    pub data: HashMap<String,i32>,
}

pub trait Count<T> {
    fn countv(&mut self,v:Vec<T>);// -> &mut VectorCounter;
}

impl<T> Count<T> for VectorCounter
where T: ToString{
    fn countv(&mut self,v: Vec<T>) {

        for v_ in v.iter() {
            let x = v_.to_string();

            if self.data.contains_key(&x) {
                let x2 = self.data[&x];
                self.data.insert(x,x2 + 1);
            } else {
                self.data.insert(x,1);
            }
        }
    }
}

impl fmt::Display for VectorCounter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut s = String::from("");
            for k in self.data.keys() {
                s = s + &(k).to_string() + "|=|";// + ",";
                s = s + &(self.data[k].to_string()) +", ";
            }

            if s.len() > 0 {
                s = (s.substring(0,s.len() -1)).to_owned();
            }

            write!(f, "{}", s)//self.number)
        }
}

///////////////////////////////

pub struct SetContainer<Isize> {
    pub components: Vec<HashSet<Isize>>,
}

///////////////////////////////

pub fn vec_to_str<T>(s: Vec<T>) -> String
    where
    T : ToString {

    let mut h = String::from("");
    for s_ in s.iter() {
        h = h + &(s_.to_string());
        h.push_str("-");
    }

    if h.len() > 0 {
        h = (h.substring(0,h.len() -1)).to_owned();
    }

    h
    //s.join("-");
    //itertools::join(s, ", ")
}


pub fn str_to_vec(s:String) -> Vec<String> {
    let mut j: i32 = 0;
    let mut v: Vec<String> = Vec::new();
    while true {
        // get substring starting at j
        let sx = s.to_string().substring(j as usize, s.len()).to_owned();

        let i = next_str(sx);//(*s4).to_string());

        if i == -1 {
            v.push(s.to_string().substring(j as usize, s.len()).to_owned());
            break;
        }

        v.push(s.to_string().substring(j as usize,(j + i) as usize).to_owned());
        j = i + 1;
    }
    v
}



pub fn next_str(s:String) -> i32 {

    for (i,s_) in s.chars().enumerate() {
        if s_ == '-' {
            return i as i32;
        }
    }
    -1
}



pub fn count_hash<T,S>(s: HashSet<T,S>) -> usize {
    s.len()
}
