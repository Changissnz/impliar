
/*
generalization of algorithm
===========================

loop:
substitute
accumulate
----------
solve
*/

use crate::enci::mat2sort;
use crate::setti::matrixf;
use std::cmp::Ordering;

use ndarray::{Array,Array1,Array2,arr1,arr2,s};
use std::collections::{HashMap,HashSet};
use std::hash::Hash;

/*
calculates a soln for vector bv of active variables to equal float f32.

bv := binary vector containing active variables.
*/
pub fn equal_dist_soln_for_f32(bv:Array1<f32>, f:f32) -> Array1<f32> {
    let l = bv.len();
    let sz = mat2sort::active_size_of_vec(bv.clone());

    // case: 0-size
    let mut sol: Array1<f32> = Array1::zeros(l);
    if sz == 0 {
        return sol;
    }

    let s = f / (sz as f32);
    for (i,x) in bv.iter().enumerate() {
        if *x != 0.0 {
            sol[i] = s / x;
        }
    }
    sol
}

/*
*/
pub fn unknown_size_of_vec(v: Array1<Option<f32>>) -> usize {
    v.iter().fold(0,|acc,&s| if s.is_none() {acc} else {acc + 1})
}

pub fn usize_f32_cmp1(s1: &(usize,f32),s2: &(usize,f32)) -> std::cmp::Ordering {
    if (*s1).1 <= (*s2).1 {
    	return Ordering::Less;
    }
    Ordering::Greater
}

/*
binary error interpolator structure.

used to possibly get soln

features:
- (brute-force)|(frequency-analysis) search for substitution.

*/
pub struct BEInt {
    // data load
    pub data: Array2<f32>, // x values
    pub e_soln: Array1<f32>, // y values
    pub r_soln:Array1<Option<f32>>, // data \dot r_soln = e_soln
    pub i:usize,
    // key is element index, value is <x values...,error term, y>
    pub sub_equalities: HashMap<usize,Array1<f32>>,
    pub fanalysis:HashMap<usize,usize>
}

pub fn build_BEInt(data:Array2<f32>,e_soln:Array1<f32>) -> BEInt {
    let rs: Array1<Option<f32>> = Array1::default(data.dim().1 + 1);
    BEInt{data:data,e_soln:e_soln,r_soln: rs,i:0,sub_equalities:HashMap::new(),fanalysis:HashMap::new()}
}

impl BEInt {
    /*
    */
    pub fn error_term(&mut self, deflt: f32) -> f32 {
        let l = self.r_soln.len() - 1;
        if self.r_soln[l].is_none() {
            return deflt;
        }
        self.r_soln[l].unwrap()
    }

    pub fn ds_arr2(&mut self) -> Array2<f32> {
        let mut sol: Array2<f32> = Array2::default((self.data.dim().0,self.data.dim().1 + 1));
        let c = self.data.dim().1;

        for i in 0..c {
            let mut c_:Array1<f32> = self.data.column(i).to_owned();
            matrixf::replace_vec_in_arr2(&mut sol, &mut c_.to_owned(),i,false);
        }
        matrixf::replace_vec_in_arr2(&mut sol, &mut self.e_soln.clone(),c,false);
        sol
    }

    /*
    initial ordering by active size
    */
    pub fn order_bfs(&mut self) {
        let mut d = self.ds_arr2();
        let lj = d.dim().1 - 1;
        let ic = Some(HashSet::from_iter([lj]));
        let pr:Array1<f32> = Array1::ones(d.dim().0);
        let (mut x1,mut x2) = mat2sort::sort_arr2_tie_breakers(d,ic,pr,mat2sort::active_size_of_vec);
        let l = x1.dim().0;

        for i in 0..l {
            let mut y1 = x1.slice(s![i,0..lj]);
            self.e_soln[i] = x1[[i,lj]];
            matrixf::replace_vec_in_arr2(&mut self.data, &mut y1.to_owned(),i,true);
        }
    }
    ///////////////////////////////// accumulator methods

    /*
    outputs whether data can be solved
    */
    pub fn solve_at(&mut self,i:usize,verbose:bool) -> bool {
        let mut c:usize = self.contradictions_in_range(0,i,true).len();
        let mut c2:usize = c.clone();
        let mut r:usize = 0;
        println!("start solve at: {:?}",self.r_soln);
        while self.contradictions_in_range(0,i,true).len() > 0 {
            c2 = self.solve_contradiction(i,verbose);
            if c2 > 0 {
                println!("contradiction @ r_{}",r);
                return false;
            }
            r += 1;
        }
        return true;
    }

    /*
    solves subarr2 at i, outputs the number of remaining contradictions
    */
    pub fn solve_contradiction(&mut self,i:usize,stat:bool) -> usize {

        let acc = self.accumulate(0,i);
        let sm = self.substitution_repr(acc.clone(),0,i,stat);

        let (mut sc,mut score) = self.substitute_solve_chain(acc.clone(),sm.clone(),stat);

        let sc2: Array1<f32> = sc.into_iter().map(|x| if x.is_none() {0.} else {x.unwrap()}).collect();
        self.save_sol_to_rsoln(sc2);
        self.deduce_elements_from_rsoln(0,i,stat);
        self.contradictions_in_range(0,i,false).len()
    }


    /*
    accumulates the samples in the range [si,ei]
    output is [var coeff|error coeff|output]

    output is according to the running solution of known variables.
    */
    pub fn accumulate(&mut self, si:usize,ei:usize) -> Array1<f32> {
        let mut sol:Array1<f32> = Array1::zeros(self.data.dim().1 + 2);
        let mut j = 0;
        for i in si..ei + 1 {
            let mut sol2:Vec<f32> = self.data.row(i).to_owned().into_iter().collect();
            sol2.push(1.);
            sol2.push(self.e_soln[i].clone());
            let mut sol2_:Array1<f32> = sol2.into_iter().collect();
            sol = sol.clone() + sol2_;
        }

        // apply running soln to expr
        let mut sol2:Array1<f32> = sol.slice(s![0..self.data.dim().1 + 1]).to_owned();
        let mut rs = self.running_soln_of_sample(sol2.clone());
        self.apply_running_soln_to_expr(sol,rs)
    }

    /*
    outputs an updated expr after running soln rs is applied

    expr := [var coeff|error coeff|output]
    rs := [var values|error value]
    */
    pub fn apply_running_soln_to_expr(&mut self, expr:Array1<f32>,rs:Array1<f32>) -> Array1<f32> {
        let mut rssum = rs.sum();
        let mut sol = expr.clone();
        sol[rs.len()] = sol[rs.len()] - rssum;

         // zero out the active values in sol;
         for (i,r) in rs.into_iter().enumerate() {
             if r != 0. {
                 sol[i] = 0.;
             }
         }

         sol
    }

    /*
    checks that variable k does not contain a contradictory substitution.
    helper method for above method `is_contradictory_substitution_map`.
    */
    pub fn check_varsub_contradiction(&mut self, k:usize, substitution_map:HashMap<usize,Array1<f32>>) -> bool {
        let x = self.bfs_sub_on_var(k,substitution_map,false);
        x.contains(&k)
    }

    /// TODO: relocate this.
    /*
    performs a BFS on the substitution of variable k; outputs the hashset of all substitution variables
    for variable k.
    */
    pub fn bfs_sub_on_var(&mut self,k:usize,substitution_map:HashMap<usize,Array1<f32>>,save_data:bool) -> HashSet<usize> {
        let mut travelled_keys : HashSet<usize> = [k].into_iter().collect();
        let mut related_indices : HashSet<usize> = self.active_indices_of_expr(substitution_map[&k].clone());
        let l = substitution_map.len();
        let mut sz2:usize = related_indices.len();

        while travelled_keys.len() != l && !related_indices.contains(&k) && sz2 > 0 {

            // iterate through related indices
            let r2 = related_indices.clone();
            let sz3:usize = related_indices.len();
            sz2 = 0;
            for r in r2.iter() {
                if save_data {
                    if !self.fanalysis.contains_key(&r.clone()) {
                        self.fanalysis.insert((*r).clone(),1);
                    } else {
                        //self.fanalysis[r] = self.fanalysis[r] + 1;
                        *self.fanalysis.get_mut(r).unwrap() += 1;
                    }
                }

                // case: already travelled
                if travelled_keys.contains(r) {
                    continue;
                }

                // case: substitution map does not have it
                if !substitution_map.contains_key(r) {
                    related_indices.remove(&r);
                    continue;
                }

                let s = self.active_indices_of_expr(substitution_map[r].clone());
                sz2 += s.len();
                related_indices.extend(&s);
                travelled_keys.insert(*r);
            }
        }

        related_indices
    }

    /*
    outputs (soln, number of new variables known)

    expr := [var coeff|error coeff|output]
    */
    pub fn substitute_solve_chain(&mut self,expr:Array1<f32>,substitution_map:HashMap<usize,Array1<f32>>,verbose:bool) -> (Array1<Option<f32>>,i32) {

        let sub = self.conduct_substitution(expr.clone(),substitution_map.clone(),false);
        if verbose {
            println!("\t***substitute-solve chain***");
            println!("\t\tbefore sub");
            println!("\t{}",expr);
            println!("\t\tafter sub");
            println!("\t{}",sub);
            println!("\t\tafter conducting substitution");
        }

        let l = sub.len() - 1;
        let mut edsol = equal_dist_soln_for_f32(sub.slice(s![0..l]).to_owned(),sub[l]);
        let sc = mat2sort::active_size_of_vec(edsol.clone()) as i32;
        (edsol.into_iter().map(|x| if x != 0. {Some(x)} else {None}).collect(),sc)
    }

    /*
    outputs a substitution representation of the sample, a vector of length |s|;
    substitution minimizes the number of (active && unknown) variables.

    derives variable substitutes from the chunk [startRef,endRef].

    substitution targets variables that are not known in the running solution.

    uses a brute-force approach.

    s := expression
    */
    pub fn substitution_repr(&mut self,s: Array1<f32>, startRef:usize,endRef:usize,verbose:bool) -> HashMap<usize,Array1<f32>> {

        let mut active_var:HashSet<usize> = self.active_indices_of_expr(s.clone()).into_iter().collect();
        let ai:HashSet<usize> = self.active_indices_of_soln(self.r_soln.clone()).into_iter().collect();
        active_var = active_var.difference(&ai).into_iter().map(|x| *x).collect();
        let mut active_vars:Array1<usize> = active_var.into_iter().collect();

        let mut dummy:Array1<f32> = Array1::zeros(s.len());

        // collect into cache and determine the best
        let mut cache:Vec<(HashMap<usize,Array1<f32>>,usize)> = Vec::new();

        // load first substitution map
        let (e1,e2) : (HashMap<usize,Array1<f32>>,usize) = (HashMap::new(),0);
        cache.push((e1,e2));

        // collects all possible substitution maps, and determines the best one (results in least active vars)
        let mut best_map:HashMap<usize,Array1<f32>> = HashMap::new();
        let mut best_score:i32 = i32::MAX;//active_vars.len();


        while cache.len() > 0 {
            // get element
            let (mut c1,mut c2) = cache[0].clone();
            cache = cache[1..].to_vec();

            if verbose {
                println!("cache len {}",cache.len());
                println!("element {:?}\t\t{}",c1,c2);
            }

            // case: done w/ element, check for best
            if c2 == active_vars.len() {
                let (mut s2,mut bs2) = self.substitute_solve_chain(s.clone(),c1.clone(),verbose);
                if verbose {
                    println!("\t* done,checking");
                    println!("\t* try substituting : {}",s);
                    println!("\t* new expr: {:?}",s2);
                    println!("\t* score {}, after substitution: {:?}",bs2,s2);
                }

                if bs2 < best_score && bs2 > 0 {
                    best_map = c1.clone();
                    best_score = bs2;
                }
                continue;
            }

            // get possibilities for active var c2
            if verbose {
                println!("* var reprs for {}",active_vars[c2]);
            }

            let vr = self.var_reprs_in_range(active_vars[c2],startRef,endRef + 1);

            // add each possibility back to cache
            for v in vr.into_iter() {
                let mut c1_ = c1.clone();
                c1_.insert(active_vars[c2],v.clone());
                if verbose {
                    println!("--- adding poss. {}\nto {:?}", v, c1);
                    println!("--- contradiction is...");
                }
                // check if valid substitution map
                if !self.is_contradictory_substitution_map(c1_.clone()) {
                    if verbose {println!("--- --- YES")};
                    cache.push((c1_,c2 + 1));
                } else {
                    if verbose {println!("--- --- NO")};
                }
            }

            // add identity case to cache
            cache.push((c1,c2 + 1));
        }
        if verbose {println!("best map: {:?}",best_map)};
        best_map
    }

    /*
    converts the expression into an output expression by the substitution map;
    if the substitution map does not contain a variable i, then output expression
    will use identity;

    WARNING:
    substitution algorithm will progressively substitute each variable that is present
    as a key in the substitution map; if substitution map is not structured properly, may
    lead to infinite loop.


    expr := [var coefficients|error term|output]
    */
    pub fn conduct_substitution(&mut self,expr:Array1<f32>, substitution_map:HashMap<usize,Array1<f32>>,verbose:bool) -> Array1<f32> {
        if verbose {println!("conducting one round of sub. on\n\t{}",expr)};
        let mut sol:Array1<f32> = Array1::default(expr.len());
        let l2 = expr.len() - 2;
        let mut exp1 = expr.clone();

        while true {
            // check to see if any active indices in substitution map
            let ai = self.active_indices_of_expr(exp1.clone());
            let mut bx:usize = ai.into_iter().fold(0, |acc,s| if substitution_map.clone().contains_key(&s) {acc + 1} else {acc.clone()});
            if bx == 0 {
                break;
            }
            exp1 = self.conduct_substitution_(exp1, substitution_map.clone());
            if verbose {println!("AFTER {}",exp1)};
        }
        exp1
    }

    /*
    helper function; conducts 1 round of substitution.

    substitution checks for known vars, and output deducts the sum of the known vars
    */
    pub fn conduct_substitution_(&mut self,expr:Array1<f32>, substitution_map:HashMap<usize,Array1<f32>>) -> Array1<f32> {
        let mut sol:Array1<f32> = expr.clone();
        let l2 = expr.len() - 2;

        //// println!("--- conducting 1 sub for {}", expr);

        // iterate through each variable and substitute
        for i in 0..l2 {
            let x = expr[i].clone();
            if !substitution_map.contains_key(&i) {
                continue;
            } else {
                let mut sub = substitution_map[&i].clone();
                // flip the y-value
                ////println!("adding sub {}",sub);
                sub[l2 + 1] = -1.0 * sub[l2 + 1];
                sub = sub * expr[i];
                //// println!("substitution for {}: {}", i,sub);
                sol[i] = 0.;
                // add substitution to sol
                sol = sol + sub;
                ////println!("sol after: {}",sol);
            }
        }

        // apply running soln to sub map
        let rs = self.running_soln_of_sample(sol.slice(s![0..expr.len() - 1]).to_owned());
        self.apply_running_soln_to_expr(sol,rs)
    }

    /*
    unknown_vec := arr1 of unknown variable coefficients, length is data.dim().1
    rs := arr1 of running soln values
    */
    pub fn solve_by_unknown_vars(&mut self,unknown_vec:Array1<f32>,rs:Array1<f32>,wanted_value:f32) -> bool {
        println!("[solving unknown vars]");
        println!("[wanted] {} [running] {}",wanted_value,rs);

        if mat2sort::active_size_of_vec(unknown_vec.clone()) == 0 {
            if rs.sum() != wanted_value {
                println!("! contradiction after soln-- want {} got {}",wanted_value,rs.sum());
                return false;
            }
        }

        let new_wv = wanted_value - rs.sum();
        let new_sol = equal_dist_soln_for_f32(unknown_vec.clone(),new_wv);
        println!("[unknown vec] {}",unknown_vec);
        println!("[sol] {}", new_sol);
        self.save_sol_to_rsoln(new_sol)
    }

    pub fn save_sol_to_rsoln(&mut self,soln:Array1<f32>) -> bool {
        let mut rsoln2:Array1<Option<f32>> = self.r_soln.clone();

        for (i,s) in soln.into_iter().enumerate() {
            if s != 0. {
                if rsoln2[i].is_none() {
                    rsoln2[i] = Some(s.clone());
                } else {
                    return false;
                }
            }
        }
        self.r_soln = rsoln2;
        true
    }

    ////// update description
    /*
    uses known variables in rsoln to determine values of error term;

    outputs whether subarr2 [si..ei + 1] can be solved.

    if can be solved, then saves soln.
    */
    pub fn deduce_elements_from_rsoln(&mut self,si:usize,ei:usize,verbose:bool) -> bool {
        let mut original:Array1<Option<f32>> = self.r_soln.clone();

        for i in si..ei +1 {
            let mut s:Vec<f32> = self.data.row(i).to_owned().into_iter().collect();
            s.push(1.);
            let r = self.running_soln_of_sample(s.clone().into_iter().collect());
            let ruk = self.remaining_unknown_of_sample(self.data.row(i).to_owned(),r.clone());

            if verbose {
                println!("sample:\t{:?}",s);
                println!("sample rs:\t{}",r);
                println!("remaining unknown of sample:\t{:?}\n{}",ruk,mat2sort::active_size_of_vec(ruk.clone()));
            }

            if mat2sort::active_size_of_vec(ruk) == 0 {
                if r.sum() != self.e_soln[i] {
                    if self.error_term(0.) == 0. {
                        self.r_soln[self.data.dim().1] = Some(self.e_soln[i] - r.sum());
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }


    /*
    determines the indices of elements with all active && (known if absolut) variables known that
    contradict e_soln
    */
    pub fn contradictions_in_range(&mut self,si:usize,ei:usize,absolut:bool) -> HashSet<usize> {
        let mut x:HashSet<usize> = HashSet::new();

        for i in si..ei + 1 {
            let mut r:Vec<f32> = self.data.row(i).to_owned().into_iter().collect();
            r.push(1.);
            // get the running soln
            let mut rs = self.running_soln_of_sample(r.into_iter().collect());

            // contradiction only if known
            let q = self.remaining_unknown_of_sample(self.data.row(i).to_owned(),rs.clone());
            if mat2sort::active_size_of_vec(q) == 0 {
                if rs.sum() != self.e_soln[i] {
                    println!("for index {}, want {}, got {}", i,self.e_soln[i],rs.sum());
                    x.insert(i);
                }
            }

            // contradiction w/o known
            if absolut {
                if (rs.sum() - self.e_soln[i]) >= 0.01 {
                    println!("for index {}, want {}, got {}", i,self.e_soln[i],rs.sum());
                    x.insert(i);
                }
            }
        }

        x
    }

    /*
    representation of variable `vi` by element `si`.
    outputs an expression of the form [var coefficients|error term|output] for
    variable `vi`;if `vi` == 0, then None.

    si := sample index
    vi := variable index
    */
    pub fn var_repr(&mut self,si:usize,vi:usize) -> Option<Array1<f32>> {

        let mut sample: Vec<f32> = self.data.row(si).to_owned().into_iter().collect();

        // case: None
        if sample[vi] == 0.0 {
            return None;
        }

        // case: other
        let l = sample.len();

        for i in 0..l {
            if i != vi {
                sample[i] = -1.0 * sample[i];
            } else {
                sample[i] = 0.0;
            }
        }

        sample.push(-1.0);
        sample.push(self.e_soln[si]);
        let sample2:Array1<f32> = sample.into_iter().collect();
        Some(sample2)
    }

    /*
    gathers all possible representations of variable vi in range.
    note: no identity repr
    */
    pub fn var_reprs_in_range(&mut self, vi:usize,si:usize,ei:usize) -> Vec<Array1<f32>> {
        let mut sol: Vec<Array1<f32>> = Vec::new();

        for i in si..ei {
            let vr = self.var_repr(i,vi);
            if vr.is_none() {
                continue;
            }
            sol.push(vr.unwrap());
        }
        sol
    }

    /*
    Determines if substitution map is contradictory.
    If there are two variables v1,v2 in the map which use each other as substitution elements, then
    that is a contradiction.

    contradictory outputs true, o.w. false.
    */
    pub fn is_contradictory_substitution_map(&mut self, substitution_map:HashMap<usize,Array1<f32>>) -> bool {
        //println!("CONTRAA");
        //println!("{:?}",substitution_map);
        let mut keys:Vec<usize> = substitution_map.clone().into_keys().collect();
        for k in keys.into_iter() {
            //println!("checking key: {}",k);
            let b = self.check_varsub_contradiction(k,substitution_map.clone());
            //println!("check {}",b);
            if b {
                return true;
            }

        }
        false
    }

    /*
    v := length is data.dim().1 + 1
    */
    pub fn running_soln_of_sample(&mut self,v:Array1<f32>) -> Array1<f32> {
        let l = self.r_soln.len();
        assert_eq!(l,v.len());

        let mut x = Array1::zeros(l);
        for i in 0..l {
            if !self.r_soln[i].is_none() {
                x[i] = v[i] * self.r_soln[i].unwrap().clone();
            }
        }

        x
    }

    /*
    calculates the active size of the var coefficients in `expr`.

    expr := [var coefficients|error term|output]
    */
    pub fn active_size_of_expr(&mut self, expr: Array1<f32>) -> usize {
        let l = expr.len() - 2;
        (0..l).into_iter().fold(0,|acc,s| if expr[s] != 0.0 {acc + 1} else {acc})
    }

    /*
    calculates the active size of the var coefficients in `soln`.

    expr := [var coefficients|error term|output]
    */
    pub fn active_size_of_soln(&mut self,soln:Array1<Option<f32>>) -> usize {
        let l = soln.len();
        assert_eq!(l,self.r_soln.len());
        (0..l - 1).into_iter().fold(0,|acc,s| if !soln[s].is_none() {acc + 1} else {acc})
    }

    /*
    calculates the active size of the var coefficients in `expr`.
    */
    pub fn active_indices_of_expr(&mut self, expr: Array1<f32>) -> HashSet<usize> {
        let l = expr.len() - 2;
        (0..l).into_iter().filter(|i|  expr[*i] != 0.0).collect()
    }

    pub fn active_indices_of_soln(&mut self, soln: Array1<Option<f32>>) -> HashSet<usize> {
        let l = soln.len();
        (0..l - 1).into_iter().filter(|i|  !soln[*i].is_none()).collect()
    }

    /*
    s := sample
    r := running solution, length is |s| + 1.

    return:
    - array1, length is |s|
    */
    pub fn remaining_unknown_of_sample(&mut self, s: Array1<f32>, r:Array1<f32>) -> Array1<f32> {
        assert_eq!(s.len(), r.len() - 1);

        let l = s.len();
        let mut r_ = r.slice(s![0..l]).to_owned();
        let w:HashSet<usize> = mat2sort::active_size_intersection(s.clone(),r_.clone()).into_iter().collect();
        let unknown:Array1<f32> = s.into_iter().enumerate().map(|(i,j)| if !w.contains(&i) {j} else {0.0}).collect();

        unknown
    }
}

pub fn test_sample_BEInt_1() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,1.,1.,1.,1.],
        [1.,0.,0.,0.,0.],
        [0.,0.,1.,0.,0.],
        [0.,0.,0.,1.,0.],
        [1.,1.,0.,0.,0.]]),
        arr1(&[1.,30.,21.,32.,47.]))
}

pub fn test_sample_BEInt_2() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,1.],[1.,0.],[1.,1.]]),
    arr1(&[24.,32.,133.]))
}

pub fn test_sample_BEInt_3() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[1.,1.,1.,1.,0.,0.,0.,0.,0.,],
    [1.,0.,1.,1.,0.,0.,1.,1.,0.,],
    [1.,0.,0.,0.,1.,0.,0.,0.,0.,],
    [0.,0.,0.,1.,1.,1.,0.,0.,0.,],
    [0.,0.,0.,0.,1.,0.,0.,1.,1.,],
    [1.,0.,0.,1.,0.,0.,0.,1.,0.,],
    [0.,0.,0.,0.,1.,1.,1.,1.,1.,],
    [1.,0.,0.,1.,0.,0.,1.,0.,0.,],
    [0.,1.,0.,1.,0.,1.,0.,1.,0.,]]),
    arr1(&[24.,42.,57.,93.,83.,61.,37.,150.,19.]))
}

pub fn test_sample_BEInt_4() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,0.,1.,1.,1.,0.],
        [1.,0.,0.,1.,0.,0.],
        [0.,0.,1.,0.,0.,1.],
        [0.,0.,0.,1.,1.,1.],
        [1.,1.,0.,0.,0.,1.]]),
        arr1(&[70.,130.,263.,312.,474.]))
}

pub fn test_sample_BEInt_5() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,0.,1.],
        [1.,0.,1.],
        [0.,1.,1.],
        [1.,0.,0.],
        [1.,1.,0.]]),
        arr1(&[7.,14.,13.,5.,27.]))
        //arr1(&[7.,1301.,26325.,3012.,1474.]))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_BEInt_order_bfs() {
        let (x,y):(Array2<f32>,Array1<f32>) = test_sample_BEInt_1();
        let mut be = build_BEInt(x,y);
        be.order_bfs();

        let sol = arr2(&[[1., 0., 0., 0., 0.],
                        [0., 0., 1., 0., 0.],
                        [0., 0., 0., 1., 0.],
                        [1., 1., 0., 0., 0.],
                        [0., 1., 1., 1., 1.]]);
        assert_eq!(be.data,sol);
    }


    #[test]
    fn test_BEInt_var_reprs_in_range() {
        let (x,y):(Array2<f32>,Array1<f32>) = test_sample_BEInt_2();
        let mut bei = build_BEInt(x,y);
        bei.order_bfs();

        let mut vr0 = bei.var_reprs_in_range(0,0,3);
        let mut vr1 = bei.var_reprs_in_range(1,0,3);
        assert_eq!(vr1.len(),2);
        assert_eq!(vr1.len(),vr0.len());
    }

    #[test]
    fn test_BEInt_accumulate() {
        let (x,y):(Array2<f32>,Array1<f32>) = test_sample_BEInt_3();
        let mut bei = build_BEInt(x,y);
        let mut acc = bei.accumulate(0,8);

        let ans1:Array1<f32> = arr1(&[5.0, 2.0, 2.0, 6.0, 4.0, 3.0, 3.0, 5.0, 2.0, 9.0, 566.0]);
        assert_eq!(acc.clone(),ans1);

        bei.r_soln[0] = Some(23.);
        bei.r_soln[3] = Some(61.3);
        bei.r_soln[5] = Some(21.8);
        acc = bei.accumulate(0,8);
        let ans2:Array1<f32> = arr1(&[0.0, 2.0, 2.0, 0.0, 4.0, 0.0, 3.0, 5.0, 2.0, 9.0, 17.800049]);
        assert_eq!(acc.clone(),ans2);
    }


    #[test]
    fn test_BEInt_substitute_solve_chain() {
        // case 2
        let (mut test_sample_1,mut test_samplesol_1) = test_sample_BEInt_2();
        let mut bei = build_BEInt(test_sample_1,test_samplesol_1);
        bei.order_bfs();
        let mut acc = bei.accumulate(0,2);
        let sm:HashMap<usize,Array1<f32>> = HashMap::from([(0,arr1(&[0.,-1.,-1.,133.]))]);
        let (mut sc,mut score) = bei.substitute_solve_chain(acc.clone(),sm.clone(),false);
        assert_eq!(sc,arr1(&[None,None,Some(-77.)]));
        assert_eq!(score,1);
    }

    #[test]
    fn test_BEInt_solve_at() {

        //// case 1
        let (x,y):(Array2<f32>,Array1<f32>) = test_sample_BEInt_1();
        let mut bei = build_BEInt(x.clone(),y);
        bei.order_bfs();

        let stat = bei.solve_at(x.dim().0 - 1,true);
        println!("soln: {:?}",bei.r_soln);
        println!("stat: {}",stat);
        assert!(stat);
        assert_eq!(bei.contradictions_in_range(0,x.dim().0 -1,true).len(),0);

        //// case 2
        let (x2,y2):(Array2<f32>,Array1<f32>) = test_sample_BEInt_4();
        let mut bei2 = build_BEInt(x2.clone(),y2);
        bei2.order_bfs();

        let stat = bei2.solve_at(x2.dim().0 - 1,true);
        println!("soln: {:?}",bei2.r_soln);
        println!("stat: {}",stat);
        assert!(stat);
        assert_eq!(bei2.contradictions_in_range(0,x2.dim().0 -1,true).len(),0);

        //// case 3
        let (x3,y3):(Array2<f32>,Array1<f32>) = test_sample_BEInt_5();
        let mut bei3 = build_BEInt(x3.clone(),y3);
        bei3.order_bfs();

        let stat = bei3.solve_at(x3.dim().0 - 1,true);
        println!("soln: {:?}",bei3.r_soln);
        println!("stat: {}",stat);
        assert!(stat);
        assert_eq!(bei3.contradictions_in_range(0,x3.dim().0 -1,true).len(),0);

    }

}
