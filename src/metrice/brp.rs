//! range partition on labelled f32_vec of \[0,1\] values
use crate::setti::fs;
use crate::metrice::bmeas;
use ndarray::{Array1,arr1};
use std::collections::HashSet;
use std::collections::HashMap;
extern crate round;
use round::{round, round_up, round_down};

/// A struct that calculates an <fs::FSelect> instance 
/// that fits `f32_vec` with its corresponding `binary_labels` (0|1). 
/// 
/// The search for this solution can be one of:
/// - greedy approach (see <RangePartitionGF2::greedy_search__decision>)
/// - brute-force approach (see <RangePartitionGF2::brute_force_search__decision>)
pub struct RangePartitionGF2 {
    /// target vector of f32, elements in \[0,1\]
    pub f32_vec: Array1<f32>,
    /// vector of equal size to <f32_vec>, each i'th element is 0 or 1
    /// and corresponds to i'th element of <f32_vec> 
    pub binary_labels: Array1<usize>,
    /// number of partitions that `fselect` solution can have 
    pub size_threshold:usize,
    /// solution
    pub fselect: fs::FSelect,
    /// score of solution
    pub score:usize,
    /// indices in `f32_vec` such that `fselect` solution contradicts
    /// label in `binary_labels`. 
    pub contra_indices: Vec<usize>,
    /// cache of candidates in brute-force|greedy approach
    fs_cache: Vec<fs::FSelect>,
    /// basic|fm
    c_type:String
}

pub fn build_range_partition_gf2(f32_vec: Array1<f32>,binary_labels: Array1<usize>,szt:usize,c_type:String) ->RangePartitionGF2 {
    RangePartitionGF2{f32_vec:f32_vec,binary_labels:binary_labels,
        size_threshold:szt,fselect: fs::empty_FSelect(c_type.clone()),score:0,
        contra_indices:Vec::new(),fs_cache:Vec::new(),c_type:c_type}
}

impl RangePartitionGF2 {

    /// # description
    /// outputs the label of the f32
    pub fn output(&mut self,f:f32) -> Option<usize> {
        let l = self.fselect.label_of_f32(f);

        if l.is_none() {return None;}

        if self.is_contra_value(f) {
            return Some((l.unwrap() + 1) % 2);
        }
        Some(l.unwrap())
    }

    /// # return
    /// if `f` is a contradicting value according to the
    /// `contra_indices` of `fselect`. 
    pub fn is_contra_value(&mut self,f:f32) -> bool {

        for c in self.contra_indices.clone().into_iter() {
            if (self.f32_vec[c] - f).abs() > 0.00001 {
                return true;
            }
        }
        false
    }

    /// # return
    /// most frequent label for elements of f32_vec[r.0..r.1] in bounds b
    pub fn best_label_for_bounds(&mut self,b:(f32,f32),r:(usize,usize)) -> usize {
        // label -> count
        let mut sol:HashMap<usize,usize> = HashMap::new();
        for i in r.0..r.1 {
            // case: in bounds, get label
            if bmeas::in_bounds(b.clone(),self.f32_vec[i]) {
                let l = self.binary_labels[i].clone();
                if sol.contains_key(&l) {
                    *(sol.get_mut(&l).unwrap()) += 1;
                } else {
                    sol.insert(l,1);
                }
            }
        }

        if sol.len() == 0 {
            return usize::MAX;
        }

        // get the key wih the max value
        let sol2: Vec<(usize,usize)> = sol.into_iter().collect();
        let x = sol2[0].clone();
        let sol3: (usize,usize) = sol2.iter().fold(x, |acc,s| if s.1 > acc.1 {s.clone()} else {acc.clone()});
        sol3.0
    }

    /// # description
    /// used for `mode` == "fm" 
    pub fn label_fm(&mut self,f: &mut fs::FSelect,f2:f32) {
        if f.mode != "fm".to_string() {
            return;
        }

        // get index of fselect
        let index = f.index_of_f32(f2);
        let mut q = f.fm.clone().unwrap();//.clone();
        q[index.unwrap()].update_values(f2);
        f.fm = Some(q);
    }

    /// # description
    /// - modify an fselect's bounds at index bi with float f2
    /// - check for intersection with any other bound of fselect and perform appropriate
    ///  merges.
    /// - calculates new label for bound
    ///    * based on contradiction score
    ///
    /// # arguments
    /// f := involved FSelect
    /// bi := index of bounds of FSelect f
    /// f2 := involved f32
    /// r := typically the range 0-(i - 1) right before the i'th element for FSelect f
    /// label_mode_majority := bool for relabelling of modified bounds by majority (frequency),
    ///                        alternative is SUM
    pub fn modify_and_merge_fselect_bounds(&mut self,f: &mut fs::FSelect,bi:usize,f2:f32,r:(usize,usize),
        label_mode_majority:bool) -> ((f32,f32),usize) {

        // get bounds to modify and mod distance
        let b = f.index_to_data(bi);
        //let d2 = if 1. > b.1 {1.} else {b.1.clone()};
        let d = bmeas::closest_distance_to_subbound((0.,1.),b.clone(),f2);

        let mut sol:(f32,f32) = b.clone();

        //// approach 2: relative to each end of bound b
        if f2 < b.0 {
            sol.0 = f2;
        } else if f2 > b.1 {
            sol.1 = f2;
        }

        if sol.0 > sol.1 {
            sol = (sol.1,sol.0);
        }

        // check for any intersecting bounds to sol
            // iterate through all other bounds
        let blen = f.data.len();
        let obi: Vec<usize> = (0..blen).into_iter().filter(|x| *x != bi).collect();
        let dl = f.indexvec_to_data_labels(obi);

            // get indices of all other bounds that do not intersect
        let ib = bmeas::intersecting_bounds_to_bound(dl.clone().into_iter().map(|x| x.0.clone()).collect(),sol.clone());// -> Vec<usize> {
            //
        // case: no intersecting bounds
        if ib.len() == 0 {
            // modify bi exclusively
            f.data[bi] = sol.clone();
            f.data_labels[bi] = self.best_label_for_bounds(sol.clone(),r.clone());
            f.mod_fm_at_index(bi,f2);
            return (f.data[bi].clone(),f.data_labels[bi].clone());
        } else {
        // case: intersecting bounds
            // get all other from data
            let mut ibh:HashSet<usize> = ib.clone().into_iter().collect();
            ibh.insert(bi);
            let rem_indices: Vec<usize> = (0..blen).into_iter().filter(|x| !ibh.contains(&x)).collect();
            let mut rem_data: Vec<((f32,f32),usize)> = f.indexvec_to_data_labels(rem_indices);

            // merge intersecting bi with sol
            let mut bi_data: Vec<(f32,f32)> = ib.into_iter().map(|x| f.data[x].clone()).collect();
            bi_data.push(sol);
            let nb: (f32,f32) = bmeas::merge_bounds(bi_data);

            // determine best label for new bounds
            let bestl = self.best_label_for_bounds(nb.clone(),r.clone());

            // mod the data and data_labels variables of f
            rem_data.push((nb.clone(),bestl));
            let i2 = rem_data.len() - 1;
            f.data = Vec::new();
            let mut fdl:Vec<usize> = Vec::new();
            for rd in rem_data.into_iter() {
                f.data.push(rd.0);
                fdl.push(rd.1);
            }

            f.data_labels = fdl.into_iter().collect();
            f.mod_fm_at_index(i2,f2);
            return (nb.clone(),bestl);
        }
    }

    /// # description
    /// calculates the <fs::FSelect> with the lowest score by a brute-force
    /// search approach 
    ///
    /// # return
    /// <fs::FSelect> solution or None
    pub fn brute_force_search__decision(&mut self) -> Option<fs::FSelect> {
        let l = self.f32_vec.len();

        for i in 0..l {
            self.brute_force_search__decision_at_index(i);
        }

        if self.fs_cache.len() == 0 {
            return None;
        }

        // iterate through and select the one with the best score
        self.update_cache_fselect_scores();

        let mut default_sol = fs::empty_FSelect(self.c_type.clone());
        default_sol.score = Some(f32::MAX);
        let mut q = self.fs_cache.clone().into_iter().fold(default_sol,|acc,f| if acc.score.unwrap() < f.score.unwrap() {acc} else {f});

        self.score_fselect_(&mut q,(0,l),true);

        self.fselect = q.clone();
        Some(q)
    }

    pub fn update_cache_fselect_scores(&mut self) {
        let l = self.fs_cache.len();
        let fl = self.f32_vec.len();
        for i in 0..l {
            let mut q = self.fs_cache[i].clone();
            self.score_fselect_(&mut q,(0,fl),false);
            self.fs_cache[i] = q;
        }
    }

    /// # description
    /// helper method for brute-force solution, modifies existing
    /// <fs::FSelect> instances in `fs_cache by `f32_vec\[i\]`. 
    pub fn brute_force_search__decision_at_index(&mut self,i:usize) {

        // case: i = 0, re-initialize cache
        if i == 0 {
            self.fs_cache = Vec::new();
            self.fs_cache.push(fs::empty_FSelect(self.c_type.clone()));
        }

        // get bounds index
        let bi = self.fselect.index_of_f32(self.f32_vec[i].clone());

        // produce an FSelect for each possible decision
        let mut new_cache: Vec<fs::FSelect> = Vec::new();

        while self.fs_cache.len() > 0 {
            let f = self.fs_cache[0].clone();
            self.fs_cache = self.fs_cache[1..].to_vec();
            let cho = self.choices_at_index(f,i);
            new_cache.extend(cho);
        }
        self.fs_cache = new_cache;
    }

    /// # description 
    /// outputs the possible f-selects at `f32_vec\[i\]`
    pub fn choices_at_index(&mut self,f:fs::FSelect,i:usize) -> Vec<fs::FSelect> {
        // get each possible bound mod
        let bl = f.data.len();
        let v = self.f32_vec[i].clone();
        let mut sol: Vec<fs::FSelect> = Vec::new();
        for j in 0..bl {
            let mut fsel = f.clone();
            self.modify_and_merge_fselect_bounds(&mut fsel,j,v,(0,i),true);
            fsel.score = None;
            self.score_fselect_(&mut fsel,(0,i),false);

            sol.push(fsel);
        }

        // get new bound
        if bl < self.size_threshold {
            let nb:(f32,f32) = (self.f32_vec[i].clone(),self.f32_vec[i].clone());
            let mut f2 = f.clone();
            f2.score = None;
            f2.data.push(nb);

            let mut vv:Vec<usize> = f2.data_labels.clone().into_iter().collect();
            vv.push(self.binary_labels[i].clone());
            f2.data_labels = vv.into_iter().collect();
            self.score_fselect_(&mut f2,(0,i),false);

            f2.mod_fm_at_index(i,self.f32_vec[i].clone());
            sol.push(f2);
        }

        sol
    }

    /// # description
    /// scores the <fs::FSelect> f on `f32_vec[r.0..r.1]`
    /// if `save_contra` is set to true, saves all contradicting indices into
    /// struct attribute. 
    pub fn score_fselect_(&mut self,f:&mut fs::FSelect,r:(usize,usize),save_contra:bool) -> f32 {
        if self.c_type != "fm".to_string() {
            return self.score_fselect(f,r,save_contra);
        } else {
            return self.score_fselect_fm(f,r);
        }
    }

    /// # description
    /// score function based on distance of elements in `f32_vec[r.0..r.1]`
    /// to their labels
    pub fn score_fselect_fm(&mut self,f:&mut fs::FSelect,r:(usize,usize)) -> f32 {

        let d = r.1 - r.0;
        if d == 0 {
            f.score = None;
            return f32::MAX;
        }

        let fm = f.fm.clone().unwrap();
        let mut s:f32 = 0.;
        for i in r.0..r.1 {
            let ou = f.index_of_f32(self.f32_vec[i].clone());
            let j = ou.unwrap();
            s += (fm[j].meen - self.f32_vec[i].clone()).abs();
        }

        f.score = Some(s / d as f32);
        f.score.clone().unwrap()
    }

    /// # description
    /// calculates an std. score for the <fs::FSelect> f by observing its
    /// performance on the subvector index range `r` of `f32_vec`. 
    /// Formula is:
    ///             (number of partitions of `f`) * (number of elements of `f32_vec`\[r.0..r.1\] that contradict its binary label) 
    /// 
    /// *special case*
    /// if FSelect violates size threshold t,outputs f32::MAX
    pub fn score_fselect(&mut self,f:&mut fs::FSelect,r:(usize,usize),save_contra:bool) -> f32 {
        if f.data.len() > self.size_threshold {
            return f32::MAX;
        }

        if save_contra {
            self.contra_indices = Vec::new();
        }

        let mut c: f32 = 0.;
        for i in r.0..r.1 {
            let l = f.label_of_f32(self.f32_vec[i].clone());
            assert!(!l.is_none(), "data is {:?}\nval is {}", f.data,self.f32_vec[i].clone());

            if l.unwrap() != self.binary_labels[i] {
                c += 1.;
                if save_contra {
                    self.contra_indices.push(i);
                }
            }
        }
        let dl = f.data.len();
        let score:f32 = c * dl as f32;
        f.score = Some(score.clone());
        score
    }

    /// # description
    /// greedy search approach to determine the `fselect` solution
    pub fn greedy_search__decision(&mut self) {
        let l = self.f32_vec.len();
        self.fs_cache = Vec::new();
        self.fs_cache.push(fs::empty_FSelect(self.c_type.clone()));

        for i in 0..l {
            self.greedy_search__decision_for_f32(i);
        }

        let mut ff = self.fs_cache[0].clone();
        self.score_fselect_(&mut ff,(0,l),false);

        self.fselect = ff;
    }

    /// # description
    /// takes the f32 x at `f32_vec[i]`, and iterates through each <fs::FSelect> in `fs_cache`, 
    /// modifying each one with x, and loads the best modified <fs::FSelect> into `fs_cache`. 
    pub fn greedy_search__decision_for_f32(&mut self,i:usize) {
        let mut new_cache: Vec<fs::FSelect> = Vec::new();

        while self.fs_cache.len() > 0 {
            let f = self.fs_cache[0].clone();
            self.fs_cache = self.fs_cache[1..].to_vec();
            let cho = self.choices_at_index(f,i);
            new_cache.extend(cho);
        }

        // determine the lowest score in new_cache
        let mut default_sol = fs::empty_FSelect(self.c_type.clone());
        default_sol.score = Some(f32::MAX);
        let q = new_cache.into_iter().fold(default_sol,|acc,f| if acc.score.unwrap() < f.score.unwrap() {acc} else {f});
        self.fs_cache = vec![q];
    }
}

pub fn test_sample_rpgf2_1() -> RangePartitionGF2 {
    let v: Array1<f32> = arr1(&[0.2,0.3,0.4,0.5,0.55,0.7,0.75,0.8]);
    let vl: Array1<usize> = arr1(&[0,1,0,1,0,1,0,1]);
    build_range_partition_gf2(v,vl,2,"basic".to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__rpgf2__brute_force_search__decision() {
        // case 1
        let mut srp = test_sample_rpgf2_1();
        srp.size_threshold = 8;
        let mut f = srp.brute_force_search__decision().unwrap();

        srp.score_fselect(&mut f,(0,8),false);
        assert_eq!(f.score,Some(0.));
        assert_eq!(0,srp.contra_indices.len());

        // case 2
        srp.size_threshold = 2;
        let mut f = srp.brute_force_search__decision().unwrap();

        srp.score_fselect(&mut f,(0,8),false);
        assert_eq!(f.score,Some(4.));
        assert_eq!(4,srp.contra_indices.len());
    }

    #[test]
    pub fn test__rpgf2__greedy_search__decision() {
        let mut rpgf2 = test_sample_rpgf2_1();
        rpgf2.size_threshold = 2;
        rpgf2.greedy_search__decision();
        assert!(6. == rpgf2.fselect.score.unwrap() || rpgf2.fselect.score.unwrap() == 8.);
    }
}
