/*
binary range partition on vec
*/
use crate::setti::fs;
use crate::metrice::bmeas;

use ndarray::{Array1,arr1};
use std::collections::HashSet;
use std::collections::HashMap;

extern crate round;
use round::{round, round_up, round_down};

/*
over galois field 2

partition for binary labels is the one with
MIN(contra * size)
*/
pub struct RangePartitionGF2 {
    // x and y data
    f32_vec: Array1<f32>,
    binary_labels: Array1<usize>,
    pub size_threshold:usize,

    // solution
    fselect: fs::FSelect,
    score:usize,
    // solution attrib.

    // for each indice, flip solution from FSelect
    pub contra_indices: Vec<usize>, // from f32_vec
    // cache of candidates in brute-force approach
    fs_cache: Vec<fs::FSelect>
}

pub fn build_range_partition_gf2(f32_vec: Array1<f32>,binary_labels: Array1<usize>,szt:usize) ->RangePartitionGF2 {
    RangePartitionGF2{f32_vec:f32_vec,binary_labels:binary_labels,
        size_threshold:szt,fselect: fs::empty_FSelect(),score:0,
        contra_indices:Vec::new(),fs_cache:Vec::new()}
}

impl RangePartitionGF2 {

    /*
    most frequent label for elements of f32_vec[r.0..r.1] in bounds b
    */
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
        let mut sol2: Vec<(usize,usize)> = sol.into_iter().collect();
        let mut x = sol2[0].clone();
        let sol3: (usize,usize) = sol2.iter().fold(x, |acc,s| if s.1 > acc.1 {s.clone()} else {acc.clone()});
        sol3.0
    }

    /*
    - modify an fselect's bounds at index bi with float f2
    - check for intersection with any other bound of fselect and perform appropriate
      merges.
    - calculates new label for bound based on contradiction score


    r := typically the range 0-(i - 1) right before the i'th element for FSelect f
    */
    pub fn modify_and_merge_fselect_bounds(&mut self,f: &mut fs::FSelect,bi:usize,f2:f32,r:(usize,usize))
        -> ((f32,f32),usize) {
        // get bounds to modify and mod distance
        let b = f.index_to_data(bi);
        let d = bmeas::closest_distance_to_subbound((0.,1.),b.clone(),f2);

        let mut sol:(f32,f32) = b.clone();
        if round((bmeas::additive_in_bounds((0.,1.),f2,d) - b.0) as f64,3) == 0. {
            sol.0 = f2;
        } else if round((bmeas::additive_in_bounds((0.,1.),f2,d) - b.1) as f64,3) == 0. {
        //else if round(bmeas::additive_in_bounds(b.clone(),f2,d) - b.1,3) == 0. {
            sol.1 = f2;
        } else {
            assert!(false);
        }

        if sol.0 > sol.1 {
            sol = (sol.1,sol.0);
        }

        // check for any intersecting bounds to sol
            // iterate through all other bounds
        let blen = f.data.len();
        let obi: Vec<usize> = (0..blen).into_iter().filter(|x| *x != bi).collect();
        let mut dl = f.indexvec_to_data_labels(obi);

            // get indices of all other bounds that do not intersect
        let ib = bmeas::intersecting_bounds_to_bound(dl.clone().into_iter().map(|x| x.0.clone()).collect(),sol.clone());// -> Vec<usize> {
            //
        // case: no intersecting bounds
        if ib.len() == 0 {
            // modify bi exclusively
            f.data[bi] = sol.clone();
            f.data_labels[bi] = self.best_label_for_bounds(sol.clone(),r.clone());
            return (f.data[bi].clone(),f.data_labels[bi].clone());

        } else {
        // case: intersecting bounds
            // get all other from data
            let mut ibh:HashSet<usize> = ib.clone().into_iter().collect();
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

            f.data = Vec::new();
            let mut fdl:Vec<usize> = Vec::new();
            for rd in rem_data.into_iter() {
                f.data.push(rd.0);
                fdl.push(rd.1);
            }
            f.data_labels = fdl.into_iter().collect();

            return (nb.clone(),bestl);
        }
    }

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

        let mut default_sol = fs::empty_FSelect();
        default_sol.score = Some(l as f32 * self.size_threshold as f32);
        let mut q = self.fs_cache.clone().into_iter().fold(default_sol,|acc,f| if acc.score.unwrap() < f.score.unwrap() {acc} else {f});
        self.score_fselect(&mut q,(0,l),true);
        Some(q)
    }

    pub fn update_cache_fselect_scores(&mut self) {
        let l = self.fs_cache.len();
        let fl = self.f32_vec.len();
        for i in 0..l {
            let mut q = self.fs_cache[i].clone();
            self.score_fselect(&mut q,(0,fl),false);
            self.fs_cache[i] = q;
        }
    }

    /*
    */
    pub fn brute_force_search__decision_at_index(&mut self,i:usize) {

        // case: i = 0, re-initialize cache
        if i == 0 {
            self.fs_cache = Vec::new();
            self.fs_cache.push(fs::empty_FSelect());
        }

        // get bounds index
        let bi = self.fselect.index_of_f32(self.f32_vec[i].clone());

        // produce an FSelect for each possible decision
        let mut new_cache: Vec<fs::FSelect> = Vec::new();

        while self.fs_cache.len() > 0 {
            let mut f = self.fs_cache[0].clone();
            self.fs_cache = self.fs_cache[1..].to_vec();
            let mut cho = self.choices_at_index(f,i);
            new_cache.extend(cho);
        }
        self.fs_cache = new_cache;
    }

    /*
    outputs the possible f-selects at f32_vec[i]
    */
    pub fn choices_at_index(&mut self,f:fs::FSelect,i:usize) -> Vec<fs::FSelect> {
        // get each possible bound mod
        let bl = f.data.len();
        let v = self.f32_vec[i].clone();
        let mut sol: Vec<fs::FSelect> = Vec::new();
        for j in 0..bl {
            let mut fsel = f.clone();
            self.modify_and_merge_fselect_bounds(&mut fsel,j,v,(0,i));
            fsel.score = None;
            self.score_fselect(&mut fsel,(0,i),false);
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
            self.score_fselect(&mut f2,(0,i),false);
            sol.push(f2);
        }

        sol
    }

    /*
    if FSelect violates size threshold t,outputs f32::MAX
    */
    pub fn score_fselect(&mut self,f:&mut fs::FSelect,r:(usize,usize),save_contra:bool) -> f32 {
        // contradiction * size
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

    pub fn greedy_search__decision_for_f32(&mut self,i:usize) {
        //self.choices_at_index(f:fs::FSelect,i:usize) -> Vec<fs::FSelect> {

    }
}

pub fn test_sample_rpgf2_1() -> RangePartitionGF2 {
    let v: Array1<f32> = arr1(&[0.2,0.3,0.4,0.5,0.55,0.7,0.75,0.8]);
    let vl: Array1<usize> = arr1(&[0,1,0,1,0,1,0,1]);
    build_range_partition_gf2(v,vl,2)
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

}
