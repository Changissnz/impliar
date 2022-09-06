//! struct used to contain element that can
//! generate new elements based on schematic.
//! element. Notation of new elements is in the
//! form of parenthetical notation.
//!
//! # Example: merge n elements
//! merge `s1,t2,u3,v4,w5` results in
//!                 s1_t2_u3_v4_w5
//! # Example: increment element
//! increment `s1_t2_u3_v4_w5` results in 
//!                 s1_t2_u3_v4_w5(s,t,u,v,w,1)
//! # Example: increment element
//! increment `s1_t2_u3_v4_w5(s,t,u,v,w,1)` results in 
//!                 s1_t2_u3_v4_w5(s,t,u,v,w,1)(s,t,u,v,w,2)
//!
//! # Rule for increment
//! given a multi-chunk, take the last chunk,
//! strip all elements of the chunk of their numbers,
//! merge those elements, and assign them a one. 
//!
//! # Calculation of existence and implication
//! # Example: values for merged
//! existence and implication for `s1_t2_u3_v4_w5` is
//!                 `f(s1) * f(t2) * f(u3) * f(v4) * f(w5)` 
use std::string::ToString;
use std::string::String;
use std::ops::{Add, Sub};
use crate::enci::ohop;
use crate::setti::setf;

/// # description
/// alphabetical stringization of vector `s`
pub fn merged_elements_string(s: Vec<String>) -> String {
    let s2: Vec<String> = s.into_iter().map(|x| ohop::str_alphabebetical_filter(x)).collect();
    setf::vec_to_str(s2,'_')
}

/// # description
/// 
pub fn set_source_string_increment(s: String,i:usize) -> String {
    let mut x2 = ohop::build_order_of_operator(s);
    x2.process();
    let cf = ohop::parse_OrderOfOperator__comma_format(&mut x2,ohop::str_alphabebetical_filter);
    let mut w1 = "(".to_string() + setf::vec_to_str(cf,',').as_str();
    w1 = w1 + ",";
    w1 = w1 + i.to_string().as_str();
    w1 + ")"
}

#[derive(Clone)]
pub struct ImpSetSource {
    pub idn: String,
    pub next_counter:usize,
    pub l: usize
}

pub fn build_ImpSetSource(s:String) -> ImpSetSource {
    ImpSetSource{idn:s,next_counter:0,l:1} 
}

impl Add for ImpSetSource {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut q1 = self.idn.clone();
        /*
        if self.l > 1 {
            q1 = "(".to_string() + q1.as_str() + ")";
        }
        */

        let mut q2 = other.idn.clone();
        /*
        if other.l > 1 {
            q2 = "(".to_string() + q2.as_str() + ")";
        }
        */ 
        let s1:String = q1 + "_" + q2.as_str();
        ImpSetSource{idn:s1,next_counter:0,l: self.l + other.l} 
    }
}

impl ImpSetSource {
    
    pub fn increment(&mut self) -> ImpSetSource {
        let q = set_source_string_increment(self.idn.clone(),self.next_counter);
        self.next_counter += 1;
        ImpSetSource{idn: self.idn.clone() + q.as_str(),next_counter:0,l:self.l + 1}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test__ImpSetSource_increment() {
        // case 1
        let mut iss = ImpSetSource{idn:"lussian".to_string(),next_counter:0,l:1};
        let mut iss2 = iss.increment();
        let mut x2 = ohop::build_order_of_operator(iss2.idn.clone());
        x2.process();
        let cf = ohop::parse_OrderOfOperator__comma_format(&mut x2,ohop::str_alphabebetical_filter);
        assert_eq!(cf,vec!["lussian".to_string(), "lussian".to_string()]);

        // case 2
        let mut iss3 = iss + iss2.clone(); 
        assert_eq!(iss3.idn, "lussian_lussian(lussian,0)".to_string()); 
        assert_eq!(3,iss3.l); 

        let mut iss4 = iss2 + iss3; 
        assert_eq!(iss4.idn, "lussian(lussian,0)_lussian_lussian(lussian,0)".to_string()); 
        assert_eq!(5,iss4.l);
    }

}