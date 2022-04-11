use ndarray::{Array,Array1,arr1};
use std::fmt;

#[derive(Clone)]
pub struct Skew {
    pub adder: Option<i32>,
    pub multer: Option<i32>,
    pub addit: Option<Array1<i32>>,
    pub multit: Option<Array1<i32>>,
    pub skew_size: usize,
    pub ordering:Vec<usize>,
    pub subgroups:Option<Vec<Vec<usize>>>
}

/*
ordering
0 adder,
1 multer,
2 addit,
3 multit
*/
pub fn build_skew(a: Option<i32>,m: Option<i32>,
        ad: Option<Array1<i32>>, am: Option<Array1<i32>>,o:Vec<usize>,sg:Option<Vec<Vec<usize>>>) -> Skew {

    // calculate the size of the skew
    let mut ss: usize = 0;

    if !a.is_none() {
        ss += 1;
    }

    if !m.is_none() {
        ss += 1;
    }

    if !ad.is_none() {
        ss += 1;
    }

    if !am.is_none() {
        ss += 1;
    }

    Skew {adder: a,multer: m,addit: ad, multit: am,skew_size: ss,ordering:o,subgroups:sg}
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

    pub fn skew_value(&mut self, mut v : Array1<i32>) -> Array1<i32> {

        let l = self.ordering.len();
        for i in 0..l {
            v = self.apply_at(v,i);
        }
        v
    }

    /*
    pub fn subgroup_apply(&mut self, v:Array1<i32>, i: usize) -> Array1<i32> {
        // get operation
        let m =
    }
    */

    // used for no subgroups
    pub fn apply_at(&mut self, v:Array1<i32>, i:usize) -> Array1<i32> {
        assert!(i <= 3);

        let x:usize = self.ordering[i];
        if self.ordering[i] == 0 {
            return v + self.adder.unwrap();
        } else if self.ordering[i] == 1 {
            return v * self.multer.unwrap();
        } else if self.ordering[i] == 2 {
            let mut r:&Array1<i32> = self.addit.as_ref().unwrap();
            return v + r;
        } else {
            let mut r:&Array1<i32> = self.multit.as_ref().unwrap();
            return v * r;
        }
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
