use crate::setti::vs;
use crate::setti::vs::VSelect;
use ndarray::{Array1,arr1};

/*
a variant of the VSelect: the unordered vector of ranges

`index_order` is the ordering of the ranges
*/
#[derive(Clone,Debug)]
pub struct UVSelect {
    pub v: VSelect,
    pub index_order: Array1<usize>
}

pub fn build_uvselect(mut v:VSelect,io:Array1<usize>) -> UVSelect {
    let (l,l2) = (io.len(),v.len());
    assert_eq!(l,l2);
    UVSelect{v:v,index_order:io}
}


impl UVSelect {

    /*
    calculates start indices for all possible choices for next sub-range in vselect

    n: total size
    s: wanted size
    d: min distance from any range in data
    */
    pub fn available_binaries(&mut self, n:usize, s:usize,d:usize) -> Vec<usize> {
        let af = self.v.available_forward(n);
        assert!(!af.is_none());
        let mut l : usize = 0;
        let mut i: usize = 0;
        let l2: usize = self.v.len();
        let mut sol:Vec<usize> = Vec::new();

        // searches through from
        while l < n && i < l2 {
            let q = self.v.data[i].clone();

            let (mut start, mut end):(usize,usize) = (0,0);
            end = q.0 - d - s;
            if l != 0 {
                start = l + d;
            }

            for j in start..(end + 1) {
                sol.push(j);
            }

            l = q.1.clone();
            i += 1;
        }

        l = af.unwrap() + d;
        let end = n - s;

        if l >= end {
            return sol;
        }

        for j in l..(end + 1) {
            sol.push(j);
        }


        sol
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_UVSelect_available_binaries() {

        let data:Vec<(usize,usize)> = vec![(6,20),(28,50),(57,72),(80,83)];
        let mut vsel = vs::build_vselect(data);
        let mut us = build_uvselect(vsel,arr1(&[0,3,1,2]));

        let b = us.available_binaries(100,3,1);
        assert_eq!(b,vec![0, 1, 2, 21, 22, 23, 24, 51, 52,
                53, 73, 74, 75, 76, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97]);

        let b2 = us.available_binaries(100,3,2);
        assert_eq!(b2,vec![0, 1, 22, 23, 52, 74, 75, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97]);

        let b3 = us.available_binaries(100,5,0);
        assert_eq!(b3,vec![0, 1, 20, 21, 22, 23, 50, 51, 52, 72, 73, 74, 75, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95]);

    }

}
