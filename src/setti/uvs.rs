use crate::setti::vs;
use crate::setti::vs::VSelect;
use ndarray::{Array1,arr1,Axis};
use std::fmt;

/// a variant of the VSelect: the unordered vector of ranges
/// `index_order` is the ordering of the ranges
#[derive(Clone,Debug)]
pub struct UVSelect {
    pub v: VSelect,
    pub index_order: Vec<usize>
}

pub fn build_uvselect(mut v:VSelect,io:Vec<usize>) -> UVSelect {
    let (l,l2) = (io.len(),v.len());
    assert_eq!(l,l2);
    UVSelect{v:v,index_order:io}
}


impl UVSelect {

    pub fn is_valid_pre_vselect(&mut self,n:usize,k:usize,d:usize,s:usize) -> bool {
        // check for
        let sz = self.v.size();
        sz + self.v.max_possible_option_size(n,d,s,0) >= k
    }

    pub fn data(&mut self) -> Vec<(usize,usize)> {
        self.index_order.clone().into_iter().map(|x| self.v.data[x].clone()).collect()
    }

    pub fn size(&mut self) -> usize {
        self.v.size()
    }

    pub fn add_elemente(&mut self, e:(usize,usize)) -> Option<usize> {
        let o = self.v.add_elemente(e);
        if o.is_none() {
            return o;
        }

        // o is ordered index, len(v) - 1 is actual
        self.index_order.push(o.unwrap());
        self.update_index_order(o.unwrap());
        o
    }

    pub fn update_index_order(&mut self, o:usize) {
        self.index_order = self.index_order.clone().into_iter().map(|x| if x >= o {x + 1} else {x}).collect();
    }

    /*
    calculates start indices for all possible choices for next sub-range in vselect

    n: total size
    s: wanted size
    d: min distance from any range in data
    */
    pub fn available_binaries(&mut self, n:usize, s:usize,d:usize) -> Vec<usize> {
        let mut c = self.v.complement(n,d);

        let mut sol:Vec<usize> = Vec::new();
        for r in c.data.iter() {
            if r.1 - r.0 + 1 >= s {
                for x in r.0..(r.1 + 1 - s) {
                    sol.push(x);
                }
            }
        }
        sol
    }

}

impl fmt::Display for UVSelect {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UVS {:?}", self.v.data)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_UVSelect_available_binaries() {

        let data:Vec<(usize,usize)> = vec![(6,20),(28,50),(57,72),(80,83)];
        let mut vsel = vs::build_vselect(data);
        let mut us = build_uvselect(vsel,vec![0,3,1,2]);

        let b = us.available_binaries(100,3,1);
        assert_eq!(b,vec![0, 1, 2, 21, 22, 23, 24, 51, 52, 53, 73, 74, 75, 76, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96]);
        let b2 = us.available_binaries(100,3,2);
        assert_eq!(b2,vec![0, 1, 22, 23, 52, 74, 75, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96]);

        let b3 = us.available_binaries(100,5,0);
        assert_eq!(b3,vec![0, 1, 20, 21, 22, 23, 50, 51, 52, 72, 73, 74, 75, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94]);
    }

}
