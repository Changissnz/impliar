use crate::setti::setf;
use crate::setti::inc;
use crate::setti::strng_srt;
use crate::setti::selection_rule;
use crate::setti::vecf;
use crate::setti::setf::Count;
use std::collections::HashMap;
use std::string::ToString;
use std::string::String;
use std::fmt;
use std::cmp::Ordering;
use std::cmp::PartialEq;

/*
SetImp is a struct that can generate sets of strings using the generator
`selection_rule::next_available_forward`.

SetImp takes a start vector, a base options vector, and the required number of
elements to generate.

SetImp will keep a running count of elements for each vector output.
*/
pub struct SetImp<T> {
    pub start_value: Vec<T>,
    pub operating_start:Vec<String>, // initial elements
    pub operating:Vec<String>,
    pub operating_index:Vec<usize>,
    pub base_options:Vec<String>, // all elements
    pub options:Vec<String>,
    // frequencies
    pub data_table: setf::VectorCounter,
    pub i: inc::Incr<inc::Inc1String>,

    //
    pub distance:usize,
    pub requiredSize: usize
}

impl<T> SetImp<T> {

    pub fn update_data_table(&mut self, element:Vec<String>) {
        self.data_table.countv(element);
    }

    pub fn next(&mut self) -> Vec<String> {
        // get next available forward
        let l = self.options.len();
        let mut g: Option<Vec<usize>> = selection_rule::next_available_forward(self.operating_index.clone(),l,self.distance);
        let mut sol:Vec<String> = Vec::new();

        if g.is_none() {
            // generate new here
            return Vec::new();
        }

        //
        let mut h: Vec<usize> = g.unwrap();
        for g_ in h.iter() {
            sol.push(self.options[*g_].clone());
        }

        // update
        self.operating_index = h;
        self.operating = sol.clone();
        sol
    }

}

pub fn build_set_imp<T>(v: Vec<T>,baseOptions:Vec<T>,rs:usize,distance:usize) ->SetImp<T>
    where
    T:ToString + Clone + PartialEq,
 {
    let r = setf::generic_vec_to_stringvec((v).clone());
    let mut dt= setf::build_VectorCounter();
    dt.countv(r.clone());

    // get the highest one
    let vm = strng_srt::inc1string_vector_max(r.clone());
    let e :inc::Incr<inc::Inc1String> = inc::Incr{x:inc::Inc1String{value:vm}};
    let mut bo = setf::generic_vec_to_stringvec(baseOptions.clone());

    let osi = vecf::subvector_indices_to_indices(v.clone(), baseOptions);

    SetImp{start_value:v,operating_start:r.clone(), operating:r.clone(),
        operating_index: osi, base_options:bo.clone(),options:bo.clone(),
        data_table:dt,i:e, requiredSize:rs,distance:distance}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_set_imp() {
        let mut v: Vec<String> = vec!["a".to_string().clone(),"b".to_string().clone(),"c".to_string().clone()];
        let mut v2 = vec!["b".to_string(),"d".to_string(),"e".to_string()];
        let mut x = setf::VectorCounter{data:HashMap::new()};
        x.countv(v.clone());

        let mut x2 = build_set_imp(v.clone(), v.clone(), 3,2);
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
