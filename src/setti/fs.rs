use crate::enci::be_int;
use crate::metrice::bmeas;
use std::collections::HashSet;
use ndarray::Array1;

/*
the f32 version that is the VSelect from setti::vs.
`data` is an unordered vec of proper bounds in `bounds`.
elements of `data` can intersect.

*/
pub struct FSelect {
    pub data: Vec<(f32,f32)>,
    pub data_labels:Array1<usize>,
    pub bounds: (f32,f32)
}

pub fn empty_FSelect() -> FSelect {
    FSelect{data:Vec::new(),data_labels:Array1::default(0),bounds:(0.,0.)}
}

pub fn build_FSelect(data: Vec<(f32,f32)>, data_labels:Array1<usize>,bounds: (f32,f32)) -> FSelect {
    assert!(bmeas::is_proper_bounds(bounds.clone()));
    assert!(data_labels.len() == data.len());
    let mut md: f32 = f32::MAX;
    let mut md1: f32 = f32::MIN;
    for d in data.clone().into_iter() {
        assert!(bmeas::is_proper_bounds(d.clone()));
        assert!(bmeas::in_bounds(bounds.clone(),d.0) && bmeas::in_bounds(bounds.clone(),d.1));
    }

    FSelect{data:data,data_labels:data_labels,bounds:bounds}
}

impl FSelect {


    pub fn mod_existing(&mut self,i:usize,b:(f32,f32),y:Option<usize>) {
        self.data[i] = b.clone();

        if y.is_none() {
            return;
        }
        self.data_labels[i] = y.unwrap().clone();
    }

    /*
    sorts bounds by closest distance to f. distance is 0. if f in b.
    */
    pub fn sorted_bounds_by_bdistance_to_f32(&mut self,f:f32) -> Array1<usize> {
        let mut v2:Vec<(usize,f32)> = self.data.clone().into_iter().enumerate().map(
            |(i,b)| (i,bmeas::closest_distance_to_subbound(self.bounds.clone(),b,f.clone()).abs())).collect();
        v2.sort_by(be_int::usize_f32_cmp1);
        v2.into_iter().map(|x| x.0).collect()
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
}
