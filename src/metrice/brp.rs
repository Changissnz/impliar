/*
binary range partition on vec
*/
use crate::setti::fs;

/*
over galois field 2

partition for binary labels is the one with
MIN(contra * size)
*/
pub struct RangePartitionGF2 {
    // x and y data
    f32_vec: Array1<f32>,
    binary_labels: Array1<usize>,
    size_threshold:usize,

    // solution
    fselect: FSelect,
    score:usize,
    // solution attrib.

    // for each indice, flip solution from FSelect
    contra_indices: Vec<usize>, // from f32_vec
    // cache of candidates in brute-force approach
    fs_cache: Vec<FSelect>
}

pub fn build_range_partition_gf2(f32_vec: Array1<f32>,binary_labels: Array1<usize>,szt:usize) ->RangePartitionGF2 {
    RangePartitionGF2{f32_vec:f32_vec,binary_labels:binary_labels,
        size_threshold:szt,fselect: fselect::empty_FSelect(),score:0,
        contra_indices:Vec::new()}
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
                    *(sol.get_mut(&l)) += 1;
                } else {
                    sol.insert(l,1);
                }
            }
        }

        // get the key wih the max value
        let sol2: Vec<(usize,usize)> = sol.into_iter().collect();
        let sol3: (usize,usize) = sol2.into_iter().fold(0, |acc,s| if s.1 > acc.1 {s.clone()} else {acc}).collect();
        sol3.0
    }

    /*
    - modify an fselect's bounds at index bi with float f2
    - check for intersection with any other bound of fselect and perform appropriate
      merges.
    - calculates new label for bound based on contradiction score


    r := typically the range 0-(i - 1) right before the i'th element for FSelect f
    */
    pub fn modify_and_merge_fselect_bounds(&mut self,&mut f:FSelect,bi:usize,f2:f32,r:(usize,usize))
        -> ((f32,f32),usize) {
        // get bounds to modify and mod distance
        let b = f.index_to_data(bi);
        let d = bmeas::closest_distance_to_subbound(self.bounds.clone(),b.clone(),f2);

        let mut sol:(f32,f32) = b.clone();
        if (bmeas::additive_in_bounds(b.clone(),f2,d) - b.0,3) == 0. {
            sol.0 = f2;
        } else if (bmeas::additive_in_bounds(b.clone(),f2,d) - b.1,3) == 0. {
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
        let obi: Vec<usize> = (0..blen).into_iter().filter(|x| x != bi).collect();
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

    pub fn brute_force_search__decision(&mut self) -> Option<FSelect> {
        let l = self.f32_vec.len();

        for i in 0..l {
            self.brute_force_search__decision_at_index(i);
        }

        if self.fs_cache.len() == 0 {
            return None;
        }

        // iterate through and select the one with the best score
        let mut default_sol = fs::empty_FSelect();
        default_sol.score = Some(l as f32 * self.size_threshold as f32);
        let q = self.fs_cache.into_iter().fold(default_sol,|acc,f| if acc.score.unwrap() < f.score.unwrap() {acc} else {f});
        Some(q)
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

        // produce an FSelect for each possible bound mod
        let mutinitial: FSelect = self.fselect.clone();

        // produce an FSelect
        let mut new_cache: Vec<FSelect> = Vec::new();

        while self.fs_cache.len() > 0 {
            let f = self.fs_cache[0];
            self.fs_cache = self.fs_cache[1..].to_vec();
            let mut cho = self.choices_at_index(f,i);
            new_cache.extend(&cho);
        }
        self.fs_cache = new_cache;
    }

    /*
    outputs the possible f-selects at f32_vec[i]
    */
    pub fn choices_at_index(&mut self,f:FSelect,i:usize) -> Vec<FSelect> {
        // get each possible bound mod
        let bl = f.data.len();
        let v = self.f32_vec[i].clone();
        let mut sol: Vec<FSelect> = Vec::new();
        for j in 0..bl {
            let mut fsel = f.clone();
            let new_data = self.modify_and_merge_fselect_bounds(&mut fsel,j,v,(0,i));
            //let dd:Vec<(f32,f32)> = new_data.clone().into_iter().map(|x| x.0.clone()).collect();
            //let dd2:Array1<usize> = new_data.clone().into_iter().map(|x| x.1.clone()).collect();
            //let mut fsel = fs::build_FSelect(dd,dd2,(0.,1.));
            self.score_fselect(&mut fsel,(0,i));
            sol.push(fsel);
        }

        // get new bound
        if bl < t {
            let nb:(f32,f32) = (self.f32_vec[i].clone(),self.f32_vec[i].clone());
            let mut f2 = f.clone();
            f2.data.push(nb);
            let mut vv:Vec<usize> = f2.data_labels.clone().into_iter().collect();
            vv.push(self.binary_labels[i].clone());
            f2.data_labels = vv.into_iter().collect();
        }

        sol.push(f2);
        sol
    }

    /*
    if FSelect violates size threshold t,outputs f32::MAX
    */
    pub fn score_fselect(&mut self,&mut f:FSelect,r:(usize,usize)) -> f32 {
        // contradiction * size
        let mut c: f32 = 0.;
        for i in r.0..r.1 {
            let l = f.label_of_f32(self.f32_vec[i].clone());
            if l != self.binary_labels[i] {
                c += 1.
            }
        }
        let dl = f.data.len();
        let score:f32 = c * dl as f32;
        f.score = Some(score.clone());
        score
    }

    pub fn greedy_search__decision_for_f32(i:usize) {

    }
}

pub fn test_sample_rpgf2_1() {

}
