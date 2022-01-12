use crate::setti::set_gen;
use crate::setti::setf;
use crate::setti::inc;
use crate::setti::setf::Count;

use std::collections::HashSet;
use std::collections::HashMap;
use factorial::Factorial;
use std::string::ToString;
use std::string::String;
use substring::Substring;
use std::fmt;
use asciis::asc::Asciis;
use crate::setti::inc::Inc;

pub fn generic_vec_to_stringvec<T>(v:Vec<T>) -> Vec<String>
    where
    T: ToString
 {

    let mut v2:Vec<String> = Vec::new();
    for v_ in v.iter() {
        let mut v3 = (*v_).to_string();
        v2.push(v3);
    }
    v2
}


pub fn is_proper_string(s:String) -> bool {
    s.chars().all(char::is_alphabetic)//numeric)
}

pub fn inc1string_to_i32(s:String) -> u32 {
    assert_eq!(is_proper_string(s.clone()),true);
    let mut x:u32 = (122 * (s.len() - 1)) as u32;
    let asc = Asciis{};
    let mut t = (s.substring(s.len() -1,s.len())).to_owned();
    let mut r:u32 = (u32::try_from(asc.ord(t.as_str()).unwrap())).unwrap();// as u32;
    x + r
}


pub fn inc1string_vector_max(s:Vec<String>) -> String {
    let s2:Vec<u32> = s.iter().map(|x| inc1string_to_i32((*x).clone() )).collect();
    // get max
    let s2m:u32 = *s2.iter().max().unwrap();

    // get index of max
    let im = s2.iter().position(|&r| r == s2m).unwrap();

    s[im].clone()
}


/*
SetImp is a struct that can generate sets of strings that satisfy implication rules.

String characters are restricted to the following:
- alphabetic characters
*/
pub struct SetImp<T> {
    pub start_value: Vec<T>,
    pub operating_start:Vec<String>,
    // frequencies
    pub data_table: setf::VectorCounter,
    pub possible_next: HashMap<String,String>,
    pub i: inc::Incr<inc::Inc1String>
}

pub fn build_set_imp<T>(v: &mut Vec<T>,countInitial:bool) ->SetImp<T>
    where
    T:ToString + Clone,
 {
    let mut r = generic_vec_to_stringvec((*v).clone());
    let mut dt= setf::VectorCounter{data:HashMap::new()};
    if countInitial {
        dt.countv(r.clone());
    }

    // get the highest one
    let vm = inc1string_vector_max(r.clone());
    let mut e :inc::Incr<inc::Inc1String> = inc::Incr{x:inc::Inc1String{value:vm}};

    SetImp{start_value:(*v).to_vec(),operating_start:r.clone(),
        data_table:dt,possible_next:HashMap::new(),i:e}
}

impl<T> fmt::Display for SetImp<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut s2 = setf::vec_to_str(self.operating_start.clone());
        s.push_str(s2.as_str());
        write!(f, "({})", s)
    }
}

impl<T> SetImp<T> {

    pub fn update_data_table(&mut self, element:Vec<String>) {
        self.data_table.countv(element);
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc1string_vector_max() {
        let mut v3 = vec!["ant".to_string(),"balkans".to_string(),"blacks".to_string()];
        let mut s3 = inc1string_vector_max(v3);
        assert_eq!(s3,"balkans".to_string());
    }

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
