//! file contains a data structure with a name that comes from
//! Disclude-Include, disinc.
use crate::setti::setf;
use crate::setti::setf::Count;
use crate::setti::strng_srt;

use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Display;

pub trait FAt<T> {
    fn at(&mut self,i:usize) -> HashSet<T>;
    fn at_range(&mut self,s:usize,e:usize) -> Vec<HashSet<T>>;
}

/// difference-space data container
#[derive(Clone)]
pub struct Discludio<T> {
    data: Vec<HashSet<T>>
}

/// intersection-space data container 
#[derive(Clone)]
pub struct Includia<T> {
    data: Vec<HashSet<T>>
}

impl<T> FAt<T> for Includia<T>
where T: Clone
{

    //
    fn at(&mut self,i:usize) -> HashSet<T> {
        assert!(! (i >= self.data.len()));
        self.data[i].clone()
    }

    fn at_range(&mut self,s:usize,e:usize) -> Vec<HashSet<T>> {
        assert!(s < e);
        assert!(self.data.len() >= e);
        self.data[s..e + 1].to_vec()
    }
}

impl<T> FAt<T> for Discludio<T>
where T: Clone + Hash
{

    //
    fn at(&mut self,i:usize) -> HashSet<T> {
        assert!(! (i >= self.data.len()));
        self.data[i].clone()
    }

    fn at_range(&mut self,s:usize,e:usize) -> Vec<HashSet<T>> {
        assert!(s < e);
        assert!(self.data.len() >= e);
        self.data[s..e + 1].to_vec()
    }
}

impl<T> Includia<T>
where
    T:Clone + Display
{

    pub fn push(&mut self,elemente: HashSet<T>) -> bool {
        self.data.push(elemente.clone());
        true
    }
}

/// a struct that uses <disinc::Discludia> and <disinc::Includio>
/// to perform pairwise merging of Vec<String> based on `float_range`
#[derive(Clone)]
pub struct DisIncRule {
    pub float_range:(f32,f32),
    pub vec_merge_rule: fn(Vec<String>,Vec<String>) -> Vec<String>
}

impl DisIncRule {

    /// # description
    /// if `f` in `float_range` then `vec_merge_rule(v1,v2)`, otherwise no merge
    pub fn bool_decision(&mut self,v1:Vec<String>,v2:Vec<String>,f:f32) -> Option<Vec<String>> {
        if f >= self.float_range.0 && f <= self.float_range.1 {
            return Some((self.vec_merge_rule)(v1,v2));
        }
        None
    }


}

/// # description 
/// rule for merging two vectors, outputs ordered vector of unique elements
pub fn vec_merge_rule_v1(v1:Vec<String>,v2:Vec<String>) -> Vec<String> {
    let mut h1: HashSet<String> = v1.into_iter().collect();
    let mut h2: HashSet<String> = v2.into_iter().collect();
    h1.extend(h2);
    let mut v1: Vec<String> = h1.into_iter().collect();
    strng_srt::sort_inc1string_vector(&mut v1);
    v1
}

/// acts as a intersection/difference-like data container 
///
/// Struct extends `ref_vec` through calling `decision_process` on
/// Vec\<String\> arguments that satisfy a <disinc::DisIncRule>.
///
/// for use in closure measures concerning:
/// - pairwise set
/// - (seq|set)\<set\>
#[derive(Clone)]
pub struct DisIncForwardChainHead {
    /// identifier for data structure
    ///
    /// typical use:
    ///
    ///     - a stringized sequence of indices for a sequence of sets.
    identifier: String,
    /// reference set
    pub ref_vec: Vec<String>,
    /// include: the intersection of the elements
    inc_e: Includia<String>,
    /// disclude: the "cumulative" difference of the elements
    dis_e: Discludio<String>,
    /// sequence of `Includia/Discludio` pairs corresponding
    /// to <Vec\<String\>> arguments
    mem: Vec<(Includia<String>,Discludio<String>)>,
    /// counter that aids in intersection-difference operations
    vcproc: setf::VectorCounter,
    /// rule used for merging a new <Vec\<String\>>
    pub dsr: DisIncRule,
}

pub fn build_DisIncForwardChainHead(idn: String,rv:Vec<String>,dsr:DisIncRule) -> DisIncForwardChainHead {
    let mut vcp: setf::VectorCounter = setf::build_VectorCounter();
    vcp.countv(rv.clone());

    let q : Vec<HashSet<String>> = Vec::new();
    DisIncForwardChainHead{identifier:idn,ref_vec:rv,inc_e:Includia{data:q.clone()},dis_e:Discludio{data:q.clone()},
        mem:Vec::new(),vcproc:vcp,dsr:dsr}
}

/// # description
/// a set intersection-difference score of the form
///             ((include / |h1| + include / |h2|) + (disclude / |h1| + disclude / |h2|)) / 2.0 
///
/// # warning
/// fail for 0-size
pub fn set_delta_function_pair_1<T>(h1:HashSet<String>,h2:HashSet<String>,i:Includia<T>,d:Discludio<T>) -> f32
where
T: Clone
{
    assert_eq!(i.data.len(),1);
    assert_eq!(d.data.len(),2);
    assert!(h1.len() > 0);
    assert!(h2.len() > 0);

    // calculate intersection score for each h1,h2
    let i1:f32 = (i.data[0].len() as f32) / (h1.len() as f32);
    let i2:f32 = (i.data[0].len() as f32) / (h2.len() as f32);

    // calculate disclude score
    let d1:f32 = (d.data[0].len() as f32) / (h1.len() as f32);
    let d2:f32 = (d.data[1].len() as f32) / (h2.len() as f32);

    (i1 + i2) / 2.0 - (d1 + d2) / 2.0
}

/// # description
/// converts a pair of hashsets into their Includia-Discludio form.
pub fn hashset_pair_to_incdis_pair<T>(vk:HashSet<T>,vk2:HashSet<T>) -> (Includia<T>,Discludio<T>)
where
T:Clone + Hash + Eq {

    let mut d1:HashSet<T> = vk.difference(&vk2).into_iter().map(|x| (*x).clone()).collect();
    let mut d2:HashSet<T> = vk2.difference(&vk).into_iter().map(|x| (*x).clone()).collect();
    let dis = Discludio{data:vec![d1,d2]};

    let mut i2:HashSet<T> = vk.intersection(&vk2).into_iter().map(|x| (*x).clone()).collect();
    let incl = Includia{data:vec![i2]};

    (incl,dis)
}

impl DisIncForwardChainHead
 {

    
    pub fn mod_identifier(&mut self, idn: String) {
         self.identifier = idn;
     }

    pub fn idn(&mut self) -> String {
         self.identifier.clone()
     }

     
    /// # description 
    /// resets <disinc::Includia> and <disinc::discludio> after processing a <Vec\<String\>>
    pub fn reset_for_next(&mut self) {
         let x =  (self.inc_e.clone(),self.dis_e.clone());
         self.mem.push(x);

         let q:Vec<HashSet<String>> = Vec::new();
         self.inc_e = Includia::<String>{data:q.clone()};
         self.dis_e = Discludio::<String>{data:q.clone()};
     }

    /// # description 
    ///  processes `x` using vector counter and stores results into
    /// `inc_e` and `dis_e`
    pub fn process_next(&mut self,x: Vec<String>) -> f32  {
        self.reset_for_next();
        let mut vc = setf::VectorCounter{data:HashMap::new()};
        vc.countv(x);

        // get difference from previous
        let mut vk2:HashSet<String> = vc.data.clone().into_keys().collect();
        let mut vk:HashSet<String> = self.vcproc.data.clone().into_keys().collect();
        let (incl,dis) = hashset_pair_to_incdis_pair(vk.clone(),vk2.clone());

        // do vector count for includia and discludio
        self.inc_e = incl.clone();
        self.dis_e = dis.clone();
        set_delta_function_pair_1(vk,vk2,incl,dis)
    }

    /// # description
    /// the main function in <disinc::DisIncForwardChainHead>; merges
    /// `x` into `ref_vec` if `dsr(`ref_vec`,`x`)
    pub fn decision_process(&mut self,x:Vec<String>) -> bool {
        let ps = self.process_next(x.clone());
        let output = self.dsr.bool_decision(self.ref_vec.clone(),x,ps);

        if output.is_none() {
            return false;
        }

        self.ref_vec = output.unwrap().clone();
        true
    }

    pub fn summarize(&mut self) {
    }
}

//// tests
pub fn test_sample_disinc_vec_1() -> Vec<String> {
    vec!["1stringos".to_string(),
        "1gullos".to_string(),
        "1bullos".to_string()]
}

pub fn test_sample_disinc_vec_0() -> Vec<String> {
    vec!["stringos".to_string(),
        "gullos".to_string(),
        "bullos".to_string()]
}

pub fn test_sample_disinc_vec_2() -> Vec<String> {
    vec!["stringos".to_string(),
    "gullos".to_string(),
    "bullos".to_string(),
    "skullos".to_string(),
    "skillos".to_string(),
    "nullos".to_string()]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__DisInc__process_next() {
        // case: 1
        let idn = "id1".to_string();
        let rv:Vec<String> = test_sample_disinc_vec_0();


        let dsra1:(f32,f32) = (0.,1.);
        let  dsr:DisIncRule = DisIncRule{float_range:dsra1,vec_merge_rule: vec_merge_rule_v1};

        let mut dif = build_DisIncForwardChainHead(idn,rv,dsr);

        let rv2 = test_sample_disinc_vec_1();
        let mut dif_ = dif.clone();
        let s = dif_.process_next(rv2);
        assert_eq!(s,-1.);

        // case: 2
        let rv3 = test_sample_disinc_vec_2();
        dif_ = dif.clone();
        let s2 = dif_.process_next(rv3);
        assert_eq!(s2,0.5);

        // case: 3
        let rv3 = test_sample_disinc_vec_0();
        dif_ = dif.clone();
        let s2 = dif_.process_next(rv3);
        assert_eq!(s2,1.);

    }



}
