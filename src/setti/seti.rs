//use crate::setti::set_gen;
use crate::setti::setf;
use crate::setti::inc;
use crate::setti::strng_srt;
use crate::setti::selection_rule;
//use crate::setti::inc::Inc;
use crate::setti::setf::Count;
use std::collections::HashMap;
use std::string::ToString;
use std::string::String;
use std::fmt;
use std::cmp::Ordering;

/*
SetImp is a struct that can generate sets of strings that satisfy implication rules.

String characters are restricted to the following:
- alphabetic characters
*/
pub struct SetImp<T> {
    pub start_value: Vec<T>,
    pub operating_start:Vec<String>, // initial elements
    pub operating:Vec<String>, // all elements

    // frequencies
    pub data_table: setf::VectorCounter,

    // possible choices for next in set construction
    //pub possible_next: HashMap<String,String>,
    pub i: inc::Incr<inc::Inc1String>,
    pub pw: Vec<(String,f32)>,
    pub sr: Option<selection_rule::SelectionRule>,
}

pub fn build_set_imp<T>(v: &mut Vec<T>,count_initial:bool) ->SetImp<T>
    where
    T:ToString + Clone,
 {
    let r = setf::generic_vec_to_stringvec((*v).clone());
    let mut dt= setf::VectorCounter{data:HashMap::new()};
    if count_initial {
        dt.countv(r.clone());
    }

    // get the highest one
    let vm = strng_srt::inc1string_vector_max(r.clone());
    let e :inc::Incr<inc::Inc1String> = inc::Incr{x:inc::Inc1String{value:vm}};

    SetImp{start_value:(*v).to_vec(),operating_start:r.clone(),
        operating:r.clone(),data_table:dt,i:e, pw:Vec::new(),
        sr:None}
}

impl<T> fmt::Display for SetImp<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let s2 = setf::vec_to_str(self.operating_start.clone());
        s.push_str(s2.as_str());
        write!(f, "({})", s)
    }
}

/*
SetImp is a
*/
impl<T> SetImp<T> {

    //////////////////////////////// start: recording info.
    pub fn update_data_table(&mut self, element:Vec<String>) {
        self.data_table.countv(element);
    }

    /*
    assigns probability weights to var. and sorts `operating`
    by it
    */
    //$$
    pub fn assign_probability_weights(&mut self, pw: Vec<(String,f32)>) {
        self.pw = pw;
        let mut x:Vec<String> = self.operating.clone();
        self.operating = strng_srt::sort_elements_by_probability_weights(self.pw.clone(), &mut x);//elements: &mut Vec<String>)//-> Vec<String> {
    }


    /*
    loads batch info requirements
    */
    pub fn load_batch_info(&mut self) {

    }

    /*
    */
    pub fn next_imp(&mut self) {
    }

}

/*
SRSeed is r
*/
////////////////////////////////
/*
pub struct SRSeed {
    pub v
}
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_set_imp() {
        let mut v: Vec<String> = vec!["a".to_string().clone(),"b".to_string().clone(),"c".to_string().clone()];
        let mut v2 = vec!["b".to_string(),"d".to_string(),"e".to_string()];
        let mut x = setf::VectorCounter{data:HashMap::new()};
        x.countv(v.clone());

        let mut x2 = build_set_imp(&mut (v.clone()), true);
        x2.update_data_table(v2);

        let mut sol = HashMap::new();
        sol.insert("a".to_string(),1);
        sol.insert("b".to_string(),2);
        sol.insert("c".to_string(),1);
        sol.insert("d".to_string(),1);
        sol.insert("e".to_string(),1);
        for (k,v) in sol.iter() {
            let mut ni = (x2.data_table.data.get_mut(k).unwrap()).clone();
            assert_eq!(ni,*v);
        }
    }
}
