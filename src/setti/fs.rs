use crate::enci::be_int;
use crate::metrice::bmeas;
use std::collections::HashSet;
use ndarray::{arr1,Array1};

/*
the f32 version that is the VSelect from setti::vs.
`data` is an unordered vec of proper bounds in `bounds`.
elements of `data` can intersect.

*/
#[derive(Clone)]
pub struct FSelect {
    pub data: Vec<(f32,f32)>,
    pub data_labels:Array1<usize>,
    pub bounds: (f32,f32),
    pub score:Option<f32>
}

pub fn empty_FSelect() -> FSelect {
    FSelect{data:Vec::new(),data_labels:Array1::default(0),bounds:(0.,1.),score:None}
}

/*
*/
///////
/*
pub fn merge_intersections_of_bounds_vec(bv: Vec<(f32,f32)>) -> Vec<(f32,f32)> {
}
*/



pub fn build_FSelect(data: Vec<(f32,f32)>, data_labels:Array1<usize>,bounds: (f32,f32)) -> FSelect {
    assert!(bmeas::is_proper_bounds(bounds.clone()));
    assert!(data_labels.len() == data.len());
    let mut md: f32 = f32::MAX;
    let mut md1: f32 = f32::MIN;
    for d in data.clone().into_iter() {
        assert!(bmeas::is_proper_bounds(d.clone()));
        assert!(bmeas::in_bounds(bounds.clone(),d.0) && bmeas::in_bounds(bounds.clone(),d.1));
    }

    FSelect{data:data,data_labels:data_labels,bounds:bounds,score:None}
}

impl FSelect {

    /*
    sorts bounds by closest distance to f. distance is 0. if f in b.
    */
    pub fn sorted_bounds_by_bdistance_to_f32(&mut self,f:f32) -> Array1<usize> {
        let mut v2:Vec<(usize,f32)> = self.data.clone().into_iter().enumerate().map(
            |(i,b)| (i,bmeas::closest_distance_to_subbound(self.bounds.clone(),b,f.clone()).abs())).collect();
        v2.sort_by(be_int::usize_f32_cmp1);
        v2.into_iter().map(|x| x.0).collect()
    }

    /*
    deletes bounds at index i and outputs its ((start,end),label)
    */
    pub fn delete_bounds(&mut self,i:usize) -> ((f32,f32),usize) {
        let d = self.data[i].clone();
        let d2 = self.data_labels[i].clone();

        let l = self.data.len();
        let h:Vec<usize> = (0..l).into_iter().filter(|j| *j != i).collect();
        self.data = h.clone().into_iter().map(|j| self.data[j].clone()).collect();
        self.data_labels = h.clone().into_iter().map(|j| self.data_labels[j].clone()).collect();

        (d,d2)
    }

    //// fselect selectors
    pub fn index_of_f32(&mut self, f:f32) -> Option<usize> {
        if !bmeas::in_bounds(self.bounds.clone(),f) {
            return None;
        }

        for (i,x) in self.data.clone().into_iter().enumerate() {
            if bmeas::in_bounds(x.clone(),f) {
                return Some(i);
            }
        }
        None
    }

    pub fn label_of_f32(&mut self, f:f32) -> Option<usize> {
        let i = self.index_of_f32(f);

        if i.is_none() {
            return None;
        }

        Some(self.data_labels[i.unwrap()].clone())
    }

    pub fn label_exists(&mut self,l:usize) -> bool {
        let r:HashSet<usize> = self.data_labels.clone().into_iter().collect();
        r.contains(&l)
    }

    pub fn index_to_data(&mut self,i:usize) -> (f32,f32) {
        self.data[i].clone()
    }

    pub fn indexvec_to_data_labels(&mut self,v:Vec<usize>) -> Vec<((f32,f32),usize)> {
        let mut sol: Vec<((f32,f32),usize)> = Vec::new();

        for v_ in v.into_iter() {
            let x = (self.data[v_].clone(),self.data_labels[v_].clone());
            sol.push(x);
        }
        sol
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_bdistance_of_f32pair() {
        let d: Vec<(f32,f32)> = vec![(2.,4.),(10.,11.5), (20.,26.)];
        let dl: Array1<usize> = arr1(&[0,1,2]);
        let mut fsel = build_FSelect(d,dl,(0.,28.));

        assert_eq!(Some(0),fsel.label_of_f32(3.));
        assert_eq!(Some(1),fsel.label_of_f32(10.3));
        assert_eq!(Some(2),fsel.label_of_f32(25.4444444));
        assert_eq!(None,fsel.label_of_f32(215.4444444));
    }

}
