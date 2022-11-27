//! structs used for partitioning of f32 range based on 
//! pre-assigned labels for values. 
use crate::enci::be_int;
use crate::metrice::bmeas;
use crate::setti::setf;
use std::collections::HashSet;
use ndarray::{arr1,Array1};
use std::fmt;

/// struct used to keep track of FSelect mean and frequency per bound
#[derive(Clone)]
pub struct FM {
    pub frequency: usize,
    pub meen: f32
}

pub fn empty_FM() -> FM {
    FM{frequency:0,meen:0.}
}

impl FM {

    /// update `frequency` and `meen` with argument `v`
    pub fn update_values(&mut self,v: f32) {
        let mut q = self.meen.clone() * self.frequency as f32;
        self.frequency += 1;
        self.meen = (self.meen + v)  / self.frequency as f32;
    }
}

/// the f32 version that is the VSelect from setti::vs.
/// `data` is an unordered vec of proper bounds in `bounds`.
/// elements of `data` cannot intersect. The main functions of
/// this struct are 
/// - <fs::FSelect::label_of_f32(f32)>
/// - <fs::FSelect::index_of_f32(f32)>; index of range
#[derive(Clone)]
pub struct FSelect {
    /// unordered vector of ranges, ranges cannot intersect
    pub data: Vec<(f32,f32)>,
    /// vector of equal length to `data`, each element is 0 or 1
    pub data_labels:Array1<usize>,
    /// f32 range that all ranges of data must exist in
    pub bounds: (f32,f32),
    /// storage value assigned to <fs::FSelect> instance;
    /// used for purposes such as ranking. 
    pub score:Option<f32>,
    /// "basic" | "fm"
    pub mode: String,
    /// used in the event of fm mode
    pub fm: Option<Vec<FM>>
}

pub fn empty_FSelect(mode:String) -> FSelect {
    let om: HashSet<String> = HashSet::from_iter(["basic".to_string(), "fm".to_string()]);
    let fm : Option<Vec<FM>> = if mode == "basic".to_string() {None} else {Some(Vec::new())};
    FSelect{data:Vec::new(),data_labels:Array1::default(0),bounds:(0.,1.),score:None,
        mode: mode, fm: fm}
}

pub fn build_FSelect(data: Vec<(f32,f32)>, data_labels:Array1<usize>,bounds: (f32,f32),mode:String) -> FSelect {
    assert!(bmeas::is_proper_bounds(bounds.clone()));
    assert!(data_labels.len() == data.len());
    let mut md: f32 = f32::MAX;
    let mut md1: f32 = f32::MIN;
    for d in data.clone().into_iter() {
        assert!(bmeas::is_proper_bounds(d.clone()));
        assert!(bmeas::in_bounds(bounds.clone(),d.0) && bmeas::in_bounds(bounds.clone(),d.1));
    }

    let fm : Option<Vec<FM>> = if mode == "basic".to_string() {None} else {Some(Vec::new())};
    FSelect{data:data,data_labels:data_labels,bounds:bounds,score:None,
        mode:mode,fm:fm}
}

impl fmt::Display for FSelect {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut q = "* fselect ".to_string();
        q.push_str(&format!("** data\n{:?}\n", self.data.clone()));
        q.push_str(&format!("** labels\n{:?}\n", self.data_labels.clone()));

        if !self.score.is_none() {
            q.push_str(&format!("** score {}",self.score.unwrap()));
        } else {
            q.push_str("** score ??");
        }
        write!(f, "{}", q)
    }
}

impl FSelect {

    pub fn mod_fm_at_index(&mut self,i:usize,f:f32) {
        if self.mode == "basic".to_string() {
            return ;
        }

        let mut fm1 = self.fm.clone().unwrap();

        while fm1.len() <= i {
            fm1.push(empty_FM());
        }

        fm1[i].update_values(f);
        self.fm = Some(fm1);
    }

    /// # description
    /// sorts bounds by closest distance to f. distance is 0. if f in b.
    /// # return
    /// arr1\<usize\>, each element is index of range 
    pub fn sorted_bounds_by_bdistance_to_f32(&mut self,f:f32) -> Array1<usize> {
        let mut v2:Vec<(usize,f32)> = self.data.clone().into_iter().enumerate().map(
            |(i,b)| (i,bmeas::closest_distance_to_subbound(self.bounds.clone(),b,f.clone()).abs())).collect();
        v2.sort_by(be_int::usize_f32_cmp1);
        v2.into_iter().map(|x| x.0).collect()
    }

    /// # description
    /// deletes bounds at index i
    /// # return
    /// ((start,end),label) of deleted bounds
    pub fn delete_bounds(&mut self,i:usize) -> ((f32,f32),usize) {
        let d = self.data[i].clone();
        let d2 = self.data_labels[i].clone();

        let l = self.data.len();
        let h:Vec<usize> = (0..l).into_iter().filter(|j| *j != i).collect();
        self.data = h.clone().into_iter().map(|j| self.data[j].clone()).collect();
        self.data_labels = h.clone().into_iter().map(|j| self.data_labels[j].clone()).collect();

        (d,d2)
    }

    /// # return
    /// index of the range that `f` falls in
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

    /// # return
    /// label of `f` based on its corresponding range
    pub fn label_of_f32(&mut self, f:f32) -> Option<usize> {
        let i = self.index_of_f32(f);

        if i.is_none() {
            return None;
        }

        Some(self.data_labels[i.unwrap()].clone())
    }

    /// # return
    /// if `l` is a label in `data_labels` 
    pub fn label_exists(&mut self,l:usize) -> bool {
        let r:HashSet<usize> = self.data_labels.clone().into_iter().collect();
        r.contains(&l)
    }

    /// # return
    /// the corresponding range at index `i` in `data`
    pub fn index_to_data(&mut self,i:usize) -> (f32,f32) {
        self.data[i].clone()
    }

    /// # description
    /// maps the vector `v` of range indices of `data` to 
    /// # return
    /// vector of (range,label) pairs
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
        let mut fsel = build_FSelect(d,dl,(0.,28.),"basic".to_string());

        assert_eq!(Some(0),fsel.label_of_f32(3.));
        assert_eq!(Some(1),fsel.label_of_f32(10.3));
        assert_eq!(Some(2),fsel.label_of_f32(25.4444444));
        assert_eq!(None,fsel.label_of_f32(215.4444444));
    }

}
