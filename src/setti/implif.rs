use std::collections::HashMap;
use crate::setti::setf::{build_VectorCounter,VectorCounter,Count};
//use crate::enci::imp_element; 
extern crate savefile;
use savefile::prelude::*;

/// # description
/// calculates the score of 
pub fn impli_element_score_1(e:f32,i:f32,ew: f32,iw: f32,f:f32,l:f32) -> f32  {
    let d = f / l;
    if d == 0. {
        return 0.; 
    }
    (e * ew + i * iw) / d 
}

/// # description
/// used to calculate the score of a set of elements;
/// outputs the average score of `s`
pub fn impli_set_score_1(s:Vec<f32>) -> f32 {
    let el = s.len();
    if el == 0 {
        return 0.;
    }

    s.into_iter().sum::<f32>() / (el as f32) 
}

/// struct that represents the threshold function for seed elements
/// of a k-statement
pub struct ImpElementSeedSizeF {
    /// multiplier weight for score 
    pub m: f32,
    /// threshold value for score
    pub t: f32
}

pub fn build_ImpElementSeedSizeF(m:f32,t:f32) -> ImpElementSeedSizeF {
    ImpElementSeedSizeF{m:m,t:t}
}

impl ImpElementSeedSizeF {

    /// # description
    /// number of new seed elements given element score `s` and set-source
    /// size `s1`
    pub fn size(&mut self,s:f32,s1:i32) -> usize {
        if s < self.t {
            return 0;
        }
        let d = s - self.t;
        (d * s1 as f32 * self.m).round() as usize
    }
}

/// struct used by Impli, set source function. 
/// ----------------------------------------------------------------------
/// input of arguments to function `f` are:
///     - unique element
///     - element existence
///     - element implication
///     - element frequency
///     - element lifespan
/// ----------------------------------------------------------------------
/// struct is also used to record existence/implication/frequency/lifespan
/// scores
pub struct ImpliSSF { 
    pub f: fn(f32,f32,f32,f32,f32,f32) -> f32,
    pub f2: fn(Vec<f32>) -> f32, 
    /// hashmap remembers element to existence/implication
    pub ht: HashMap<String,(f32,f32)>,
    /// vector counter for frequency
    vc1:VectorCounter, 
    /// vector counter for lifespan
    vc2:VectorCounter,
    /// existence weight
    pub ew: f32, 
    /// implication weight
    pub iw: f32
}

pub fn build_ImpliSSF(f: fn(f32,f32,f32,f32,f32,f32) -> f32,
    f2: fn(Vec<f32>) -> f32, ht: HashMap<String,(f32,f32)>,ew:f32,iw:f32) -> ImpliSSF {

    let x: Vec<String> = ht.clone().into_keys().collect();
    let mut vc1 = build_VectorCounter();
    let mut vc2 = build_VectorCounter();

    vc1.countv(x.clone());
    vc2.countv(x.clone());

    ImpliSSF{f:f,f2:f2,ht:ht,vc1:vc1,vc2:vc2,
        ew:ew,iw:iw} 
}

impl ImpliSSF {

    /// # description
    /// applies the function `f` to the element `e`; used to determine
    /// options for next k-statement of <impli::Impli>. 
    pub fn apply(&mut self,e:String) -> f32 {
        let x = self.ht.get_mut(&e); 
        assert!(!x.is_none());
        let x2 = x.unwrap().clone();
        //println!("VALUE OF: {} ",e.clone());
        //println!("VC1");
        //println!("{:?}",self.vc1.data);
        
        let d1 = self.vc1.value(e.clone());
        let d2 = self.vc2.value(e.clone());

        (self.f)(x2.0,self.ew,x2.1,self.iw,d1 as f32,d2 as f32) 
    }

    /// # description
    /// applies the function `f2` to `ev`, the k-set of an <impli::Impli>
    /// k-statement.  
    pub fn apply2(&mut self,ev:Vec<String>) -> f32 {
        let s:Vec<f32> = ev.into_iter().map(|x| self.apply(x)).collect(); 
        (self.f2)(s) 
    }

    pub fn update_element(&mut self,s:String,ei: Option<(f32,f32)>) {
        // case: new element
        if !ei.is_none() {
            self.ht.insert(s.clone(),ei.unwrap());
        }

        self.vc1.countv(vec![s.clone()]); 
        self.vc2.countv(vec![s]); 
    }

    /// # description
    /// records all elements in k-statement into `ht` and
    /// outputs the set-source with the highest score
    pub fn record_k_statement(&mut self, k:Vec<Vec<String>>) -> Vec<String> {
        
        // iterate through and 
        Vec::new()
    }

    /// # description
    /// 
    pub fn record_k_node(&mut self,k: Vec<String>) -> Vec<String> {
        let ne = self.new_elements_from_k_node(k); 
        Vec::new()
    }

    /// # description
    /// 
    pub fn new_elements_from_k_node(&mut self,k:Vec<String>) -> Vec<String> {
        // get new elements
        let mut v3 = build_VectorCounter();
        v3.countv(k);
        v3 = v3 - self.vc1.clone();

            // iterate through and get new elements
        let mut x: Vec<String> = Vec::new(); 
        for k in v3.data.into_keys() {
            if !self.vc1.contains(k.clone()) {
                x.push(k.clone()); 
            }
        }
        x

    }

    /*
    /// # description 
    /// 
    pub fn max_ssnode(&mut self,k:Vec<Vec<String>>) -> Vec<String> {
    }

    /// # description
    /// derives the existence and implication of new element `s` from its
    /// parents. 
    pub fn bayesian_derive(&mut self,s:String) -> (f32,f32) {

        // get the pre-string 
        let mut pre = imp_element::last_parse_impli_ss_string(s); 

        (0.,0.)
    }
    */ 

}