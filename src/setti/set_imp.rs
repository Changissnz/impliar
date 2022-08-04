use crate::setti::setf;
//use crate::setti::inc;
use crate::setti::strng_srt;
//use crate::setti::selection_rule;
//use crate::setti::vecf;
//use crate::setti::setf::Count;
use crate::setti::setc;
use crate::setti::set_gen;
use crate::strng_srt::sort_string_vector;
use round::{round_up};
use std::collections::HashSet;
use std::string::ToString;
use std::string::String;
use std::fmt;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use ndarray::Array2;


/*
SetImp is a struct that can generate vectors of strings using the generator
`selection_rule::next_available_forward`.

SetImp takes a start vector, a base options vector, and the required number of
elements to generate.

SetImp will keep a running count of elements for each vector output.
*/
pub struct SetImp {
    // choice vector
    cvec:Vec<String>,
    // length of each sequence
    k: usize, 
    // closure rate
    c: f32,
    // closure size
    cs: usize,
    // closure index
    si: usize,
    // start index of next batch 
    bi:usize,
    batch: Vec<HashSet<String>>,
    pub finished:bool
}

pub fn build_set_imp<T>(cvec:Vec<T>,k:usize,c:f32) ->SetImp
    where 
    T:ToString + Clone + PartialEq,
{
    let sv = setf::generic_vec_to_stringvec(cvec);
    let mut si = SetImp{cvec:sv,k:k,c:c,cs:0,si:0,bi:0,batch:Vec::new(),finished:false};
    si.calculate_closure_size();
    si
}

impl SetImp {

    pub fn calculate_closure_size(&mut self) {
        let sz = setc::nCr(self.cvec.len(),self.k);
        self.cs = round_up(sz as f64 * self.c as f64,0) as usize
    }

    pub fn load_next_batch(&mut self) {
        self.batch = set_gen::fcollect(self.cvec.clone(),self.bi,self.k);
    }

    pub fn next(&mut self) -> Option<Vec<String>> {
        if self.finished {
            return None;
        }

        if self.si == self.cs {
            self.finished = true;
            return None;
        }

        if self.batch.len() == 0 {
            self.load_next_batch();
            self.bi += 1;
        }

        let mut q:Vec<String> = self.batch[0].clone().into_iter().collect();
        sort_string_vector(&mut q);
        self.batch = self.batch[1..].to_vec();
        self.si += 1;
        Some(q)
    }
}