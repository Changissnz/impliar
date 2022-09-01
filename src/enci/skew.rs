//! struct for error-correction
use ndarray::{Array,Array1,arr1};
use std::fmt;
use std::collections::HashSet;
use std::ops::{Add};

/// struct used to map an i32 n-vector into another i32 n-vector
#[derive(Debug,Clone,PartialEq)]
pub struct Skew {
    /// additive singleton
    pub adder: Option<i32>,
    /// multiplicative singleton
    pub multer: Option<i32>,
    /// additive vector
    pub addit: Option<Array1<i32>>,
    /// multiplicative vector
    pub multit: Option<Array1<i32>>,
    /// summation of all values used in <skew::Skew>
    pub skew_size: usize,
    /// determines the ordering to apply elements of <skew::Skew>
    /// - 0 := adder 
    /// - 1 := multer
    /// - 2 := addit
    /// - 3 := multit
    pub ordering:Vec<usize>,
}

pub fn build_skew(a: Option<i32>,m: Option<i32>,
        ad: Option<Array1<i32>>, am: Option<Array1<i32>>,o:Vec<usize>,sg:Option<Vec<Vec<usize>>>) -> Skew {

    // calculate the size of the skew
    let mut ss: usize = 0;

    if !a.is_none() {
        ss += a.unwrap().abs() as usize;
    }

    if !m.is_none() {
        ss += m.unwrap().abs() as usize;
    }

    if !ad.is_none() {
        let q:Vec<i32> = ad.clone().unwrap().into_iter().map(|y| y.abs()).collect();
        ss += q.into_iter().sum::<i32>() as usize;
    }

    if !am.is_none() {
        let q:Vec<i32> = am.clone().unwrap().into_iter().map(|y| y.abs()).collect();
        ss += q.into_iter().sum::<i32>() as usize;
    }

    Skew {adder: a,multer: m,addit: ad, multit: am,skew_size: ss,ordering:o}
}


impl fmt::Display for Skew {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");
        for &o in self.ordering.iter() {

            if o == 0 || o == 2 {
                s = s + "+";
            } else {
                s = s + "*";
            }

            if o == 0 {
                s.push_str(&self.adder.unwrap().to_string());
            } else if o == 1 {
                s.push_str(&self.multer.unwrap().to_string());
            } else if o == 2 {
                s.push_str(&self.addit.as_ref().unwrap().to_string());
            } else {
                s.push_str(&self.multit.as_ref().unwrap().clone().to_string());
            }
        }

        write!(f, "{}", s)
    }
}

impl Skew {

    /// # return
    /// set of <skew::Skew> elements that are not None
    pub fn active(&mut self) -> HashSet<usize> {
        let mut hs:HashSet<usize> = HashSet::new();

        if !self.adder.is_none() {
            hs.insert(0);
        }

        if !self.multer.is_none() {
            hs.insert(1);
        }

        if !self.addit.is_none() {
            hs.insert(2);
        }

        if !self.multit.is_none() {
            hs.insert(3);
        }
        hs
    }

    /// # description
    /// applies elements (adder|addit|multer|multit) by `ordering` onto v
    pub fn skew_value(&mut self, mut v : Array1<i32>) -> Array1<i32> {
        let l = self.ordering.len();
        for i in 0..l {
            v = self.apply_at(v,i);
        }
        v
    }

    /// # description
    /// applies the i'th element in `ordering` of <skew::Skew> on vector `v`
    /// # return
    /// modified `v`
    pub fn apply_at(&mut self, v:Array1<i32>, i:usize) -> Array1<i32> {
        assert!(i <= 3);

        let x:usize = self.ordering[i];
        if self.ordering[i] == 0 {
            return v + self.adder.unwrap();
        } else if self.ordering[i] == 1 {
            return v * self.multer.unwrap();
        } else if self.ordering[i] == 2 {
            assert_eq!(v.len(),self.addit.clone().unwrap().len());
            let mut r:&Array1<i32> = self.addit.as_ref().unwrap();
            return v + r;
        } else {
            assert_eq!(v.len(),self.multit.clone().unwrap().len());
            let mut r:&Array1<i32> = self.multit.as_ref().unwrap();
            return v * r;
        }
    }

    // # description
    // 
    /*
    pub fn inversion(&mut self)  -> Skew {

    }
    */
}

/// add schematic for Skew is the following:
///
/// - use the skew ordering of self. 
/// - go through <?adder,?addit,?multer,?multit> and
///                 - if self or other is None, take !None
///                 - otherwise, add self + other
impl Add for Skew {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut a1: Option<i32> = None;
        let mut a2: Option<Array1<i32>> = None;
        let mut m1: Option<i32> = None;
        let mut m2: Option<Array1<i32>> = None;

        if !self.adder.is_none() {
            if !other.adder.is_none() {
                let a = self.adder.unwrap() + other.adder.unwrap();
                a1 = Some(a);
            } else {
                a1 = self.adder.clone(); 
            }
        }

        if !self.addit.is_none() {
            if !other.addit.is_none() {
                let a_ = self.addit.unwrap() + other.addit.unwrap();
                a2 = Some(a_);
            } else {
                a2 = self.addit.clone(); 
            }
        }

        if !self.multer.is_none() {
            if !other.multer.is_none() {
                let m = self.multer.unwrap() + other.multer.unwrap();
                m1 = Some(m);
            }  else {
                m1 = self.multer.clone(); 
            }
        }

        if !self.multit.is_none() {
            if !other.multit.is_none() {
                let m_ = self.multit.unwrap() + other.multit.unwrap();
                m2 = Some(m_);
            }  else {
                m2 = self.multit.clone(); 
            }
        }

       build_skew(a1,m1,a2,m2,self.ordering.clone(),None) 
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Skew_skew_value() {
        let x1: i32 = 2;
        let x2: i32 = 4;
        let ordering:Vec<usize> = vec![0,1];
        let mut s: Skew = build_skew(Some(x1),Some(x2),None,None,ordering,None);
        let mut v: Array1<i32> = arr1(&[0,1,2,5]);
        let mut r: Array1<i32> = s.skew_value(v);
        let mut v2: Array1<i32> = arr1(&[8, 12, 16, 28]);
        assert_eq!(r,v2);
    }

}
