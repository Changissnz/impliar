//! contains interpolator over sequence of vectors 
#[allow(non_snake_case)]
use crate::enci::mat2sort;
use crate::enci::i_mem;
use crate::setti::matrixf;
use std::cmp::Ordering;

use ndarray::{Array,Array1,Array2,arr1,arr2,s};
use std::collections::{HashMap,HashSet};
use std::hash::Hash;

/// # description 
/// calculates a soln for vector `bv` of active variables to equal `f`
/// 
/// # arguments 
/// bv := binary vector containing active variables.
/// f := target value 
///
/// # return
/// <arr1\<f32\>> `a` such that `a * bv = f` 
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

/// # return
/// number of unknowns in  `v`
pub fn unknown_size_of_vec(v: Array1<Option<f32>>) -> usize {
    v.iter().fold(0,|acc,&s| if s.is_none() {acc} else {acc + 1})
}

/// # description
/// (usize,f32) pair comparator function 1; used for sorting
pub fn usize_f32_cmp1(s1: &(usize,f32),s2: &(usize,f32)) -> std::cmp::Ordering {
    if (*s1).1 <= (*s2).1 {
    	return Ordering::Less;
    }
    Ordering::Greater
}

/// binary error interpolator structure.
/// Used to {possibly} get soln.
///
/// features:
/// - brute-force
/// - frequency-analysis search for substitution.
///
///
/// generalization of algorithm:
/// - loop
///     - accumulate
///     - substitute
///     - solve
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct BEInt {
    /// x-values
    pub data: Array2<f32>,
    /// y-values
    pub e_soln: Array1<f32>,
    /// value for each `data` variable (column)
    pub r_soln:Array1<Option<f32>>,
    /// 
    pub i:usize,
    /// 
    pub fanalysis:HashMap<usize,usize>,
    /// 
    pub ranalysis:HashMap<usize,Array1<f32>>,
    pub im: i_mem::IMem
}

#[allow(non_snake_case)]
pub fn build_BEInt(data:Array2<f32>,e_soln:Array1<f32>) -> BEInt {
    let rs: Array1<Option<f32>> = Array1::default(data.dim().1 + 1);
    let im = i_mem::build_one_imem();
    let mut b = BEInt{data:data,e_soln:e_soln,r_soln: rs,i:0,fanalysis: HashMap::new(),ranalysis:HashMap::new(),im:im};
    b.initiaado();
    b
}

impl BEInt {

    /// # description
    /// orderes de data es gracias y gorolobos es stela cumar mi yo si
    /// IMem.
    pub fn initiaado(&mut self) {
        self.order_bfs();
    }

    /// # return
    /// the error term
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
            let c_:Array1<f32> = self.data.column(i).to_owned();
            matrixf::replace_vec_in_arr2(&mut sol, &mut c_.to_owned(),i,false);
        }
        matrixf::replace_vec_in_arr2(&mut sol, &mut self.e_soln.clone(),c,false);
        sol
    }

    /// # description 
    /// initial ordering by non-decreasing active size
    pub fn order_bfs(&mut self) {
        let  d = self.ds_arr2();
        let lj = d.dim().1 - 1;
        let ic = Some(HashSet::from_iter([lj]));
        let pr:Array1<f32> = Array1::ones(d.dim().0);
        let (x1,x2) = mat2sort::sort_arr2_tie_breakers(d,ic,pr,mat2sort::active_size_of_vec);
        let l = x1.dim().0;

        for i in 0..l {
            let y1 = x1.slice(s![i,0..lj]);
            self.e_soln[i] = x1[[i,lj]];
            matrixf::replace_vec_in_arr2(&mut self.data, &mut y1.to_owned(),i,true);
        }
    }

    /// # return 
    /// outputs whether data can be solved
    pub fn solve_at(&mut self,i:usize,verbose:bool,solve_mode:usize) -> bool {
        let c:usize = self.contradictions_in_range(0,i,true,false).len();
        let mut c2:usize = c.clone();
        let mut r:usize = 0;

        while self.contradictions_in_range(0,i,true,false).len() > 0 {
            c2 = self.solve_contradiction(i,verbose,solve_mode);
            if c2 > 0 {
                println!("contradiction {} @ r_{}",c2,r);
                return false;
            }
            r += 1;
        }

        return true;
    }

    /// # description 
    /// solves subarr at \[0..i + 1\], outputs the number of remaining contradictions.
    ///
    /// # arguments
    /// - `i` := end index of subarr
    /// - `stat` := print statements? 
    /// - `solve_mode` :=  0 for substitution repr., 1 for representative repr.
    ///
    /// # return
    /// number of contradictions in \[0,i\]
    pub fn solve_contradiction(&mut self,i:usize,stat:bool,solve_mode:usize) -> usize {

        let acc = self.accumulate(0,i);
        if stat {
            println!("solving contradiction @ {} by\n{}",i,acc);
        }
        let mut sm : HashMap<usize,Array1<f32>> = HashMap::new();

        if solve_mode == 0 {
            sm = self.substitution_repr(acc.clone(),0,i,stat);
        } else {
            self.representative_analysis_();
            sm = self.representative_decision_smap(acc.clone(),stat);
        }

        if stat {
            println!("best s-map: {:?}",sm);
            println!("at start: {:?}",self.r_soln);
        }

        let lxx:usize = self.active_size_of_soln(self.r_soln.clone());
        let (sc,score) = self.substitute_solve_chain(acc.clone(),sm.clone(),stat);

        let sc2: Array1<f32> = sc.into_iter().map(|x| if x.is_none() {0.} else {x.unwrap()}).collect();
        self.save_sol_to_rsoln(sc2);
        if stat {println!("$-> after solve-chain: {:?}",self.r_soln);}
        //println!("$-> after s-map deduction {:?}",self.r_soln);
        self.deduce_elements_from_rsoln(0,i,stat);
        if stat {println!("$-> after e-deduction {:?}",self.r_soln);}
        self.save_to_imem(i);

        if stat {println!("$-> delta in r-soln: {}", self.active_size_of_soln(self.r_soln.clone()) - lxx)}
        self.contradictions_in_range(0,i,false,false).len()
    }

    /// # description
    /// saves to `im` the rsoln and any contradictions (known|unknown vars allowed)
    /// in the range \[0,i\]
    pub fn save_to_imem(&mut self,i:usize) {
        // save rsoln
        self.im.soln_log.push(self.r_soln.clone());

        // check for contradictions
        let output:Array1<f32> = self.rsoln_output(0,i);

        // add the contradiction sequence
        let cs: Vec<i_mem::ContraStruct> = Vec::new();
        for j in 0..i+1 {
            if (output[j] - self.e_soln[j]).abs() < 0.01 {
                continue;
            }

            let ii: Vec<usize> = vec![self.im.i.clone(),j];
            let q = i_mem::build_contrastruct(ii,Some(self.e_soln[j].clone()),Some(output[j].clone()));
            self.im.contradiction_log.push(q);
        }
        self.im.i += 1;
    }

    /// # description
    /// determines variable solutions from cases of substitution map `sm` that 
    /// are certain (no remaining unknown) 
    pub fn deduce_smap_keys_from_rsoln(&mut self, sm: HashMap<usize,Array1<f32>>) {
        let mut sol: HashMap<usize,f32> = HashMap::new();
        let l = self.data.dim().1 + 2;

        for (k,v) in sm.into_iter() {
            // if no unknown elements, solve
            let v_: Array1<f32> = (0..l - 1).into_iter().map(|i| v[i].clone()).collect();
            let rs = self.running_soln_of_sample(v_.clone());
            let v2_ = v_.slice(s![0..l -2]).to_owned();
            let ruk = self.remaining_unknown_of_sample(v2_,rs.clone());

            if mat2sort::active_size_of_vec(ruk) == 0 {
                sol.insert(k,rs.sum());
            }
        }

        for (k,v) in sol.into_iter() {
            println!("DeDuCiNG s-map key {:?} -> {:?}",k,v);

            if !self.r_soln[k].is_none() {
                println!("\t\treplace sol'n @ {}",k);
            }
            self.r_soln[k] = Some(v);
        }

    }

    /// # description
    /// accumulates the samples in the range \[si,ei\].
    ///
    /// Output is \[var coeff|error coeff|output\].
    ///
    /// Output is according to the running solution of known variables.
    /// 
    /// # arguments
    /// - `si` := start index
    /// - `ei` := end index
    /// 
    /// # return
    /// literal value <arr1\<f32\>> of summation of subvector `data\[si..ei + 1\]`
    pub fn accumulate(&mut self, si:usize,ei:usize) -> Array1<f32> {
        let mut sol:Array1<f32> = Array1::zeros(self.data.dim().1 + 2);
        let j = 0;
        for i in si..ei + 1 {
            let mut sol2:Vec<f32> = self.data.row(i).to_owned().into_iter().collect();
            sol2.push(1.);
            sol2.push(self.e_soln[i].clone());
            let sol2_:Array1<f32> = sol2.into_iter().collect();
            sol = sol.clone() + sol2_;
        }

        // apply running soln to expr
        let sol2:Array1<f32> = sol.slice(s![0..self.data.dim().1 + 1]).to_owned();
        let rs = self.running_soln_of_sample(sol2.clone());
        self.apply_running_soln_to_expr(sol,rs)
    }

    /// # arguments
    /// - `expr` := \[var coeff|error coeff|output\]
    /// - `rs` := \[var values|error value\]
    ///
    /// # return 
    /// an updated `expr` of unknown after running soln `rs` is applied
    pub fn apply_running_soln_to_expr(&mut self, expr:Array1<f32>,rs:Array1<f32>) -> Array1<f32> {
        let rssum = rs.sum();
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

    /// # description
    /// checks that variable k does not contain a contradictory substitution.
    /// helper method for above method `is_contradictory_substitution_map`.
    pub fn check_varsub_contradiction(&mut self, k:usize, substitution_map:HashMap<usize,Array1<f32>>) -> bool {
        let x = self.bfs_sub_on_var(k,substitution_map,false);
        x.contains(&k)
    }

    /// # description
    /// performs a BFS on the substitution of variable k; outputs the hashset of all substitution variables
    /// for variable k.
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

    /// # description
    /// outputs (soln, number of new variables known)
    /// 
    /// # arguments
    /// - `expr` := \[var coeff|error coeff|output\]
    /// - `substitution_map` := variable -> substitution
    /// - `verbose` := print statement? 
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

        // solve for unknown
        let l = sub.len() - 1;
        let edsol = equal_dist_soln_for_f32(sub.slice(s![0..l]).to_owned(),sub[l]);
        let sc = mat2sort::active_size_of_vec(edsol.clone()) as i32;
        (edsol.into_iter().map(|x| if x != 0. {Some(x)} else {None}).collect(),sc)
    }

    
    /// # description
    /// outputs a substitution representation of the sample, a vector of length |s|;
    /// substitution minimizes the number of (active && unknown) variables.
    /// Derives variable substitutes from the chunk \[start_ref,end_ref\].
    /// Substitution targets variables that are not known in the running solution.
    /// Uses a brute-force approach.
    ///
    /// # argument
    /// - `s` := expression
    /// - `start_ref` := start index
    /// - `end_ref` := end index
    /// - `verbose` := print statements?
    ///
    /// # return
    /// variable -> <arr1\<f32\>> (substitution)
    pub fn substitution_repr(&mut self,s: Array1<f32>, start_ref:usize,end_ref:usize,verbose:bool) -> HashMap<usize,Array1<f32>> {

        let mut active_var:HashSet<usize> = self.active_indices_of_expr(s.clone()).into_iter().collect();
        let ai:HashSet<usize> = self.active_indices_of_soln(self.r_soln.clone()).into_iter().collect();
        active_var = active_var.difference(&ai).into_iter().map(|x| *x).collect();
        let active_vars:Array1<usize> = active_var.into_iter().collect();
        let dummy:Array1<f32> = Array1::zeros(s.len());

        // collect into cache and determine the best

        // (map,score)
        let mut cache:Vec<(HashMap<usize,Array1<f32>>,usize)> = Vec::new();

        // load first substitution map
        let (e1,e2) : (HashMap<usize,Array1<f32>>,usize) = (HashMap::new(),0);
        cache.push((e1,e2));

        // collects all possible substitution maps, and determines the best one (results in least active vars)
        let mut best_map:HashMap<usize,Array1<f32>> = HashMap::new();
        let mut best_score:i32 = i32::MAX;//active_vars.len();
        while cache.len() > 0 {
            // get element
            let (c1,c2) = cache[0].clone();
            cache = cache[1..].to_vec();

            if verbose {
                println!("cache len {}",cache.len());
                println!("element {:?}\t\t{}",c1,c2);
            }

            // case: done w/ element, check for best
            if c2 == active_vars.len() {
                let (s2,bs2) = self.substitute_solve_chain(s.clone(),c1.clone(),verbose);
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

            let vr = self.var_reprs_in_range(active_vars[c2],start_ref,end_ref + 1);

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
                    if verbose {println!("--- --- MAYBE")};

                    // check active size improvement by rep indices
                    //// let (mut s2,_) = self.substitute_solve_chain(s.clone(),c1.clone(),false);
                    /*
                    let (_,mut s1) = self.substitute_solve_chain(s.clone(),c1.clone(),false);
                    let (_,mut s2) = self.substitute_solve_chain(s.clone(),c1_.clone(),false);

                    if s2 > s1 {
                        continue;
                    }
                    */
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
    
    /// # description
    /// converts the expression into an output expression by the substitution map;
    /// if the substitution map does not contain a variable i, then output expression
    /// will use identity;
    /// 
    /// # warning
    /// substitution algorithm will progressively substitute each variable that is present
    /// as a key in the substitution map; if substitution map is not structured properly, may
    /// lead to infinite loop.
    /// 
    /// # arguments
    /// `expr` := \[var coefficients|error term|output\]
    /// `substitution_map` := variable -> substitution variables
    /// `verbose` := print statements? 
    ///
    /// # return
    /// <arr1\<f32\>> that is the substitution
    pub fn conduct_substitution(&mut self,expr:Array1<f32>, substitution_map:HashMap<usize,Array1<f32>>,verbose:bool) -> Array1<f32> {
        if verbose {
            println!("conducting one round of sub. on\n\t{}",expr);
            println!("SM:\n{:?}\n",substitution_map);
        };

        let sol:Array1<f32> = Array1::default(expr.len());
        let l2 = expr.len() - 2;
        let mut exp1 = expr.clone();

        while true {
            // check to see if any active indices in substitution map
            let ai = self.active_indices_of_expr(exp1.clone());
            let bx:usize = ai.into_iter().fold(0, |acc,s| if substitution_map.clone().contains_key(&s) {acc + 1} else {acc.clone()});
            if bx == 0 {
                break;
            }
            exp1 = self.conduct_substitution_(exp1, substitution_map.clone());
            if verbose {println!("AFTER {}",exp1)};
        }
        exp1
    }

    /// # description 
    /// helper function; conducts 1 round of substitution.
    /// substitution checks for known vars, and output deducts the sum of the known vars
    pub fn conduct_substitution_(&mut self,expr:Array1<f32>, substitution_map:HashMap<usize,Array1<f32>>) -> Array1<f32> {
        let mut sol:Array1<f32> = expr.clone();
        let l2 = expr.len() - 2;

        // iterate through each variable and substitute
        for i in 0..l2 {
            let x = expr[i].clone();
            if !substitution_map.contains_key(&i) {
                continue;
            } else {
                let mut sub = substitution_map[&i].clone();
                // flip the y-value
                sub[l2 + 1] = -1.0 * sub[l2 + 1];
                sub = sub * expr[i];
                sol[i] = 0.;
                // add substitution to sol
                sol = sol + sub;
            }
        }

        // apply running soln to sub map
        let rs = self.running_soln_of_sample(sol.slice(s![0..expr.len() - 1]).to_owned());
        self.apply_running_soln_to_expr(sol,rs)
    }

    /// # description
    /// plugs in values for unknown in `unknown_vec` so that the update running solution to
    /// `rs` equals `wanted_value`
    /// 
    /// 
    /// # arguments
    /// - `unknown_vec` := arr1 of unknown variable coefficients, length is data.dim().1
    /// - `rs` := arr1 of running soln values
    /// - `wanted_value` := target value
    ///
    /// # return
    /// plugging in values for unknown to `rs` yields `wanted_value`? 
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

    /// # arguments
    /// - `soln` := values of variables
    ///
    /// # return
    /// `soln` active vars do not collide with `rsoln`? 
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

    /// # description
    /// uses known variables in rsoln to determine values of error term and other variables;
    /// outputs whether subarr2 \[si..ei + 1\] can be solved.
    /// If can be solved, then saves soln.
    pub fn deduce_elements_from_rsoln(&mut self,si:usize,ei:usize,verbose:bool) -> bool {

        // sort range by number of unknown
        let original:Array1<Option<f32>> = self.r_soln.clone();
        let mut iunk: Vec<(usize,f32)> = Vec::new();
        for i in si..ei + 1 {
            let mut s:Vec<f32> = self.data.row(i).to_owned().into_iter().collect();
            s.push(1.);
            let r = self.running_soln_of_sample(s.clone().into_iter().collect());
            let ruk = self.remaining_unknown_of_sample(self.data.row(i).to_owned(),r.clone());
            iunk.push((i,mat2sort::active_size_of_vec(ruk) as f32));
        }

        iunk.sort_by(usize_f32_cmp1);

        let new_rsoln = self.r_soln.clone();
        let stat_solved:bool = true;

        for i in iunk.into_iter() {

            // case: contradiction for rsoln, fix error term
            let mut s:Vec<f32> = self.data.row(i.0.clone()).to_owned().into_iter().collect();
            s.push(1.);
            let r = self.running_soln_of_sample(s.clone().into_iter().collect());
            let rs = r.sum();

            if rs != self.e_soln[i.0] {
                if verbose {println!("\t\tFIX @\n{:?}",self.data.row(i.0));}
                // fix by error term
                if self.error_term(0.) == 0. {
                    if verbose {println!("\tby error");}
                    self.r_soln[self.data.dim().1] = Some(self.e_soln[i.0] - r.sum());
                } else {
                // fix by solving for var
                    let u2 = self.remaining_unknown_of_sample(s[0..self.data.dim().1].to_vec().into_iter().collect(),r);
                    let new_sol = equal_dist_soln_for_f32(u2.clone(),self.e_soln[i.0] - rs);

                    if verbose {println!("before: {:?}",self.r_soln);}

                    for (i,ss) in new_sol.into_iter().enumerate() {
                        if ss != 0. {
                            self.r_soln[i] = Some(ss.clone());
                        }
                    }

                    if verbose {println!("after {:?}",self.r_soln);}
                }

                // check for contradiction
                let mut s2:Vec<f32> = self.data.row(i.0.clone()).to_owned().into_iter().collect();
                s2.push(1.);
                let r2 = self.running_soln_of_sample(s2.clone().into_iter().collect());
                if r2.sum() != self.e_soln[i.0] {
                    if verbose {
                        println!("-----");
                        println!("[!] deduction results in contradiction @ {}:\t{:?}",i.0,s2);
                        println!("\t want {} actual {}",self.e_soln[i.0],r2.sum());
                        println!("-----");
                    }
                    self.r_soln = original;
                    return false;
                }
            }
        }

        true
    }

    /// # return
    /// literal f32 values for subarr `data\[si..ei + 1\]
    pub fn rsoln_output(&mut self,si:usize,ei:usize) -> Array1<f32> {
        let mut sol: Array1<f32> = Array1::zeros(ei - si + 1);
        for (j,i) in (si..ei + 1).into_iter().enumerate() {
            let mut r:Vec<f32> = self.data.row(i).to_owned().into_iter().collect();
            r.push(1.);
            // get the running soln
            let rs = self.running_soln_of_sample(r.into_iter().collect());
            sol[j] = rs.sum();
        }

        sol
    }

    /// # desription
    /// determines the indices of elements with all active && (known if absolut) variables known that
    /// contradict e_soln
    pub fn contradictions_in_range(&mut self,si:usize,ei:usize,absolut:bool,verbose:bool) -> HashSet<usize> {
        let mut x:HashSet<usize> = HashSet::new();

        for i in si..ei + 1 {
            let mut r:Vec<f32> = self.data.row(i).to_owned().into_iter().collect();
            r.push(1.);
            // get the running soln
            let rs = self.running_soln_of_sample(r.into_iter().collect());

            // contradiction only if known
            let q = self.remaining_unknown_of_sample(self.data.row(i).to_owned(),rs.clone());
            if mat2sort::active_size_of_vec(q) == 0 {
                if rs.sum() != self.e_soln[i] {
                    //println!("for index {}, want {}, got {}", i,self.e_soln[i],rs.sum());
                    x.insert(i);
                }
            }

            // contradiction w/o known
            if absolut {
                if (rs.sum() - self.e_soln[i]).abs() >= 0.01 {
                    if verbose {println!("for index {}, want {}, got {}", i,self.e_soln[i],rs.sum());}
                    x.insert(i);
                }
            }
        }

        x
    }

    /// # description
    /// representation of variable `vi` by element `si`.
    /// outputs an expression of the form [var coefficients|error term|output] for
    /// variable `vi`;if `vi` == 0, then None.
    ///
    /// # arguments
    /// - si := sample index
    /// - vi := variable index
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

    /// # description
    /// gathers all possible representations of variable vi in range.
    /// 
    /// # note
    /// no identity repr
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

    /// # return
    /// all possible representations of variable `vi` in range \[`si`,`ei`\] that do not use
    /// any of the `excluded_vars`. 
    pub fn var_reprs_in_range_filtered(&mut self,vi:usize,si:usize,ei:usize,excluded_vars:HashSet<usize>) -> Vec<Array1<f32>> {

        let sol_ = self.var_reprs_in_range(vi,si,ei);
        let mut sol: Vec<Array1<f32>> = Vec::new();

        // exclude any with active indices that intersect w/ excluded_vars
        for s in sol_.into_iter() {
            let ai:HashSet<usize> = mat2sort::active_indices(s.clone()).into_iter().collect();
            let it: HashSet<_> = ai.intersection(&excluded_vars).collect();

            if it.len() > 0 {
                continue;
            }
            sol.push(s.clone());
        }
        sol

    }

    /// # description
    /// determines if substitution map is contradictory.
    ///
    /// If there are two variables v1,v2 in the map in which either v2 substitutes for v1 or vice-versa, then
    /// that is a contradiction.
    /// 
    /// # return 
    /// contradictory outputs true, o.w. false.
    pub fn is_contradictory_substitution_map(&mut self, substitution_map:HashMap<usize,Array1<f32>>) -> bool {
        let keys:Vec<usize> = substitution_map.clone().into_keys().collect();
        for k in keys.into_iter() {
            let b = self.check_varsub_contradiction(k,substitution_map.clone());
            if b {
                return true;
            }
        }

        false
    }

    /// # arguments
    /// v := length is data.dim().1 + 1
    /// 
    /// # return
    /// literal f32 values of `v`
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

    /// # description 
    /// calculates the active size of the var coefficients in `expr`.
    ///
    /// # argument
    /// expr := [var coefficients|error term|output]
    pub fn active_size_of_expr(&mut self, expr: Array1<f32>) -> usize {
        let l = expr.len() - 2;
        (0..l).into_iter().fold(0,|acc,s| if expr[s] != 0.0 {acc + 1} else {acc})
    }

    /// # description
    /// calculates the active size of the var coefficients in `soln`.
    /// expr := [var coefficients|error term|output]
    pub fn active_size_of_soln(&mut self,soln:Array1<Option<f32>>) -> usize {
        let l = soln.len();
        assert_eq!(l,self.r_soln.len());
        (0..l - 1).into_iter().fold(0,|acc,s| if !soln[s].is_none() {acc + 1} else {acc})
    }

    /// # return 
    /// the active indices of `expr`
    pub fn active_indices_of_expr(&mut self, expr: Array1<f32>) -> HashSet<usize> {
        let l = expr.len() - 2;
        (0..l).into_iter().filter(|i|  expr[*i] != 0.0).collect()
    }

    /// # return
    /// the active indices of the var coefficients in `soln`
    pub fn active_indices_of_soln(&mut self, soln: Array1<Option<f32>>) -> HashSet<usize> {
        let l = soln.len();
        (0..l - 1).into_iter().filter(|i|  !soln[*i].is_none()).collect()
    }

    /// # 
    pub fn representative_indices_of_expr(&mut self, expr: Array1<f32>,sm: HashMap<usize,Array1<f32>>) -> HashSet<usize> {
        let ai = self.active_indices_of_expr(expr.clone());
        let rism = self.representative_indices_of_smap(sm.clone());
        let mut kys: HashSet<usize> = sm.into_keys().collect();
        kys = kys.difference(&rism).into_iter().map(|x| *x).collect();
        ai.difference(&kys).into_iter().map(|x| *x).collect()
    }

    /// # description
    /// representative indices of smap are those indices found in the substitution
    /// of each key in sm but are not keys themselves.
    pub fn representative_indices_of_smap(&mut self, sm: HashMap<usize,Array1<f32>>) -> HashSet<usize> {

        let keys: HashSet<usize> = sm.clone().into_keys().collect();
        let mut sol: HashSet<usize> = HashSet::new();
        for (_,v) in sm.into_iter() {
            let ai = self.active_indices_of_expr(v);
            sol.extend(ai.into_iter());
        }

        for k in keys.into_iter() { sol.remove(&k); }
        sol
    }

    /// # arguments
    /// - s := sample
    /// - r := running solution, length is |s| + 1.
    ///
    /// # return
    /// - array1 with non-zero unknown, length is |s|
    pub fn remaining_unknown_of_sample(&mut self, s: Array1<f32>, r:Array1<f32>) -> Array1<f32> {
        assert_eq!(s.len(), r.len() - 1);

        let l = s.len();
        let r_ = r.slice(s![0..l]).to_owned();
        let w:HashSet<usize> = mat2sort::active_size_intersection(s.clone(),r_.clone()).into_iter().collect();

        let unknown:Array1<f32> = s.into_iter().enumerate().map(|(i,j)| if !w.contains(&i) {j} else {0.0}).collect();
        unknown
    }

    /////////////////////////////// methods for deciding substitution variables

    /// # description 
    /// performs a frequency count of substitution map
    ///
    /// # return 
    /// variable -> frequency vector of other variables as substitutes
    pub fn submap_var_frequency(&mut self,sm:HashMap<usize,Array1<f32>>) {
        // iterate through each key in map
        self.fanalysis = HashMap::new();
        for (k,v) in sm.clone().iter() {
            self.bfs_sub_on_var(*k,sm.clone(),true);
        }
    }

    /////////////////////// methods are for representative solution
    
    /// # return 
    /// column var index -> binary arr1 of length equal to data.dim().1 (1 values correspond to possible representative)
    pub fn representative_table(&mut self) -> HashMap<usize,Array1<f32>> {
        let base:Array1<f32> = Array1::zeros(self.data.dim().1);
        let r = self.data.dim().0;

        let mut sol:HashMap<usize,Array1<f32>> = HashMap::new();
        for i in 0..self.data.dim().1{ sol.insert(i,base.clone());}

        for i in 0..r {
            let rw = self.data.row(i).to_owned();
            let av:HashSet<usize> = mat2sort::active_indices(rw.clone()).into_iter().collect();
            let base2:Array1<f32> = (0..self.data.dim().1).into_iter().map(|j| if av.contains(&j) {1.} else {0.}).collect();

            for av_ in av.into_iter() {

                // modify key
                let mut val = sol.get_mut(&av_).unwrap().clone();
                let mut base3 = base2.clone();
                base3[av_.clone()] = 0.;
                val = val + base3;
                val = val.into_iter().map(|x| if x <= 1. {x} else {1.}).collect();
                *sol.get_mut(&av_).unwrap() = val;
            }
        }
        sol
    }

    /// # description 
    /// variant of bfs_sub_on_var;
    /// obtains all variables in which var i is either a direct or indirect parent of.
    pub fn relevant_vars_of_var_in_relevance_table(&mut self,rt:HashMap<usize,Array1<f32>>,i:usize) -> Array1<f32> {
        let rvs = self.relevant_vars_structure_of_var_in_relevance_table(rt,i);
        let mut h:HashSet<usize> = HashSet::new();
        for x in rvs.into_iter() {
            h.extend(x);
        }
        (0..self.data.dim().1).into_iter().map(|x| if h.contains(&x) {1.} else {0.}).collect()
    }

    /// # description
    /// helper method for `relevant_vars_of_var_in_relevance_table`
    pub fn relevant_vars_structure_of_var_in_relevance_table(&mut self,mut rt:HashMap<usize,Array1<f32>>,i:usize) -> Vec<HashSet<usize>> {
        let g = rt.get_mut(&i);
        if g.is_none() {
            return Vec::new();
        }

        // set of all elements checked
        let mut h:HashSet<usize> = mat2sort::active_indices(g.unwrap().clone()).into_iter().collect();
        let mut sol:Vec<HashSet<usize>> = Vec::new();
        // cache of elements to check
        let mut checked:Vec<usize> = h.clone().into_iter().collect();

        while checked.len() > 0 {
            // fetch new relevant vars
            let c = checked[0].clone();
            //// println!("C: {}",c);
            let l = checked.len();
            checked = checked[1..l].to_vec();
            //// println!("CL: {}",checked.len());
            let rev_ = rt.get_mut(&c);
            //// println!("is none: {:?}",rev_);
            if rev_.is_none() {
                //// println!("cont");
                continue;
            }

            let rev = rev_.unwrap().clone();
            //// println!("after");

            // add new relevant vars
            let ai = mat2sort::active_indices(rev);
            let ai2:Vec<usize> = ai.into_iter().filter(|x| !h.contains(&x)).collect();
            checked.extend_from_slice(&(ai2.clone()));
            //// println!("A2\t{:?}",ai2);

             // add to hashset and sol
             for ai22 in ai2.clone().into_iter() {h.insert(ai22);}
             let ai3:HashSet<usize> = ai2.into_iter().filter(|&x| x != i).collect();
             //// println!("A3\t{:?}",ai3);

             if ai3.len() > 0 {sol.push(ai3);}
             //// println!("NC: {}",checked.len());
        }
        //// println!("GOTSOL");
        sol
    }

    /// # return
    /// 2-d array, row index i corresponds to variable, column index j corresponds to
    /// frequency of variable j as representative 
    pub fn representative_relevance_table(&mut self,mut rt:HashMap<usize,Array1<f32>>,index_order:Array1<usize>) -> Array2<f32> {
        let keys:Vec<usize> = index_order.clone().into_iter().collect();
        let k2 = keys.clone();
        let lk = keys.len();

        let mut sol:Array2<f32> = Array2::zeros((lk,lk));
        for (i,k) in keys.into_iter().enumerate() {
            let mut val:Array1<f32> = rt.get_mut(&k).unwrap().clone();
            // add initial
            matrixf::replace_vec_in_arr2(&mut sol,&mut val,i,true);
            let ai = mat2sort::active_indices(val.clone());

            // iterate through each active var for relevant vars
            for a in ai.into_iter() {
                let rv2 = self.relevant_vars_of_var_in_relevance_table(rt.clone(),a);
                let io1:Array1<f32> = index_order.clone().into_iter().map(|x| rv2[x].clone()).collect();
                /*
                println!("AFTER RV");
                println!("{:?}",rv2);
                println!("\n---{:?}\n---",sol);
                */
                let mut nr = sol.row(i).to_owned() + io1;
                matrixf::replace_vec_in_arr2(&mut sol,&mut nr,i,true);
            }
        }
        sol
    }

    /// # description
    /// assigns `representative_table` to `ranalysis`. 
    pub fn representative_analysis_(&mut self) {
        self.ranalysis = self.representative_table();
    }

    ///////////////////////////////// start: relevance submatrix methods
    /// # return 
    /// frequency of representatives for each variable in `key_indices`
    pub fn relevance_submatrix(&mut self,key_indices:Array1<usize>) -> HashMap<usize,Array1<f32>> {
        let mut sol:HashMap<usize,Array1<f32>> = HashMap::new();
        let k2: HashSet<usize> = key_indices.clone().into_iter().collect();
        let ki2: Array1<usize> = key_indices.clone();

        for k in key_indices.into_iter() {
            let q = self.ranalysis.get_mut(&k).unwrap().clone();
            let q_: Array1<f32> = ki2.clone().into_iter().map(|x| q[x].clone()).collect();
            sol.insert(k,q_);
        }
        sol
    }

    /// # return 
    /// representative relevance submatrix
    pub fn rr_submatrix(&mut self,key_indices:Array1<usize>) -> Array2<f32> {
        let rs = self.relevance_submatrix(key_indices.clone());
        self.representative_relevance_table(rs,key_indices)
    }

    ///////////////////////////////// end: relevance submatrix methods

    ///
    pub fn representative_analysis(&mut self,rrm:Array2<f32>, index_keys:Array1<usize>) -> Vec<(usize,f32)> {
        assert_eq!(rrm.dim().0,index_keys.len());

        // rank the elements according to their cumulative sum
        let mut rv:Vec<(usize,f32)> = Vec::new();
        for i in 0..rrm.dim().1 {
            rv.push((index_keys[i],rrm.row(i).to_owned().sum()));
            rv.push((i,rrm.row(i).to_owned().sum()));
        }

        rv.sort_by(usize_f32_cmp1);
        rv
    }

    /// # description 
    /// selects the var. repr. for variable i with the lowest active size that does not use any excluded vars
    ///
    /// # arguments 
    /// `i` := index of variable 
    /// `excluded_vars` := set of vars that cannot be used in var repr.
    ///                 for an expression, set is usually the union of inactive vars and var i's
    ///                 representative chain (variables that use var i in its var repr)
    /// 
    /// # return 
    /// representation of var `i`
    pub fn select_var_repr_by_max_candidates(&mut self,i:usize, excluded_vars: HashSet<usize>) -> Option<Array1<f32>> {
        let vr = self.var_reprs_in_range_filtered(i,0,self.data.dim().0,excluded_vars);
        if vr.len() == 0 {
            return None;
        }

        let mut sol: Array1<f32> = vr[0].clone();
        let mut solscore: usize = self.active_size_of_expr(sol.clone());
        let l = vr.len();
        for i in 1..l {
            let acs = self.active_size_of_expr(vr[i].clone());
            if acs < solscore {
                sol = vr[i].clone();
                solscore = acs;
            }
        }

        Some(sol)
    }

    /*
    produces all possible
    */
    pub fn substitutions_at_forward(&mut self, sm: HashMap<usize,Array1<f32>>, ordering: Vec<usize>) -> Option<Vec<(HashMap<usize,Array1<f32>>,Vec<usize>)>> {

        if ordering.len() == 0 {
            return None;
        }

        // use head
        let h = ordering[0].clone();

        // get excluded vars
        let ev = self.parent_vars_of_var_in_map(sm.clone(), h);
        let vr = self.var_reprs_in_range_filtered(h,0,self.data.dim().0,ev);
        let mut sol: Vec<(HashMap<usize,Array1<f32>>,Vec<usize>)> = Vec::new();

        for v in vr.into_iter() {
            let mut el: HashMap<usize,Array1<f32>> = sm.clone();
            el.insert(h,v.clone());

            if el == sm {
                continue;
            }

            // re-arrange ordering
            /*
            let mut nu_orderingh: HashSet<usize> = self.active_indices_of_expr(v.clone());
            let mut nu_ordering: Vec<usize> = nu_orderingh.clone().into_iter().collect();
            let nu_ordering2: Vec<usize> = ordering.clone().into_iter().filter(|x| !nu_orderingh.contains(&x)).collect();
            nu_ordering.extend(&nu_ordering2);
            */
            let nu_ordering = ordering[1..].to_vec();
            sol.push((el,nu_ordering));
        }

        Some(sol)
    }

    /// # description 
    /// covers vars by substitution by forward procedure:
    /// - iterate through cache C of rep_analysis (usually sorted at .1)
    ///     - pop each var \[0\]
    ///     - process its tree structure of vars repr. Re-arrange active vars of vars repr to front of cache C.
    ///
    /// # arguments
    /// - rep_analysis := output from method representative_analysis()
    /// - excluded_vars := set of vars that cannot be used in substitution (results in 0's)
    /// 
    /// # note
    /// will use entire `data` variable of elements as candidates for var. repr.
    pub fn cover_vars_by_substitution(&mut self,rep_analysis:Vec<(usize,f32)>) -> HashMap<usize,Array1<f32>> {

        // declare the cache
        let mut cache:Vec<usize> = rep_analysis.into_iter().map(|x| x.0.clone()).collect();
        let q = cache.clone();
        let sm: HashMap<usize,Array1<f32>> = HashMap::new();
        let mut l = cache.len();

        // construct the representative map
        let h: HashSet<usize> = cache.clone().into_iter().collect();
        let excluded : HashSet<usize> = (0..self.data.dim().1).into_iter().filter(|x| !h.contains(&x)).collect();

        let mut rep_map:HashMap<usize,HashSet<usize>> = HashMap::new();
        for h_ in h.into_iter() {
            rep_map.insert(h_,excluded.clone());
        }

        // run cache
        let mut sol: HashMap<usize,Array1<f32>> = HashMap::new();

        while l > 0 {

            let x = cache[0].clone();
            cache = cache[1..].to_vec();

            // select next var repr
            let ev = rep_map.get_mut(&x).unwrap().clone();
            let vr = self.select_var_repr_by_max_candidates(x, ev);
            if vr.is_none() {
                continue;
            }

            // use var repr to get active vars and rearrange cache
            let vr_ = vr.unwrap();
            let vrai = self.active_indices_of_expr(vr_.clone());

            // get all active vars that do not have an element in substitution map yet
                // put those active vars as priority in cache (at index 0 onwards)
            let mut nu_cache:Vec<usize> = Vec::new();
            let mut nu_cacheh:HashSet<usize> = HashSet::new();
            for ii in vrai.into_iter() {

                let rmv = rep_map.get_mut(&x).unwrap();
                rmv.insert(x.clone());

                if !sol.contains_key(&ii) {
                    nu_cache.push(ii.clone());
                    nu_cacheh.insert(ii.clone());
                }
            }

                // iterate through cache and push remainder
            let nu_cache2: Vec<usize> = cache.clone().into_iter().filter(|x| !nu_cacheh.contains(&x)).collect();
            nu_cache.extend(&nu_cache2);
            cache = nu_cache;
            l = cache.len();
            sol.insert(x,vr_);
        }
        sol
    }

    /// # description 
    /// uses a cache with elements (substitution map, var search ordering) to search
    /// for s-map that produces lowest non-zero unknown active size of expr.
    pub fn brute_force_cover_vars_by_substitution(&mut self,expr:Array1<f32>,rep_analysis:Vec<(usize,f32)>, verbose:bool) -> HashMap<usize,Array1<f32>> {
        // declare the cache
            // each element in cache is a substitution map and a hashset of its excluded vars
        let ordering: Vec<usize> = rep_analysis.into_iter().map(|x| x.0.clone()).collect();
        let e1: (HashMap<usize,Array1<f32>>,Vec<usize>) = (HashMap::new(),ordering.clone());

        let mut cache:Vec<(HashMap<usize,Array1<f32>>,Vec<usize>)> = Vec::new();
        cache.push(e1);

        let mut sol: HashMap<usize,Array1<f32>> = HashMap::new();
        let mut sol_score:usize = self.data.dim().1;
        if verbose {println!("$-> brute force on expr {:?}",expr);}

        let mut l = cache.len();
        while l > 0 {
            // pop element in cache, and select next, rearrange
            let q = cache[0].clone();
            cache = cache[1..].to_vec();

            let mm = self.substitutions_at_forward(q.0.clone(), q.1.clone());

            if mm.is_none() {
                continue;
            }

            let more_maps = mm.unwrap();
            println!("pushing {} maps for ",more_maps.len());
            println!("{:?}",q);

            // iterate through more_maps and get best
            for m in more_maps.into_iter() {
                println!("\t** pushing");
                println!("{:?}",m);
                cache.push(m.clone());
                let q2 = self.conduct_substitution(expr.clone(),m.0.clone(),false);
                let ri = self.active_size_of_expr(q2.clone());

                if verbose {
                    println!("%-> representatives for\n{:?}\n",m.0.clone());
                    println!("\t\t* {:?}",q2);
                    println!("\t\t* {:?}",ri);
                }

                if ri < sol_score && ri >= 1 {
                    sol = m.0.clone();
                    sol_score = ri;
                    if verbose {println!("\tUpdating");}
                }
            }
            l = cache.len();
            println!("L: {}",l);
        }

        sol
    }

    /// # description 
    /// gathers the set of vars that result in var vi as substituent from `hm`
    pub fn parent_vars_of_var_in_map(&mut self,mut hm: HashMap<usize,Array1<f32>>, vi: usize) -> HashSet<usize> {
        let mut qual: HashSet<usize> = HashSet::new();
        qual.insert(vi);
        let mut l = hm.len();
        let mut s:usize = 1;
        while l > 0 && s > 0 {

            // iterate through and check for
            let mut nk: HashSet<usize> = HashSet::new();
            for (k,v) in hm.iter() {
                // check for active vars intersection with pertinent
                let av = self.active_indices_of_expr(v.clone());
                let inters: HashSet<_> = qual.intersection(&av).collect();
                if inters.len() > 0 {
                    nk.insert(*k);
                }
            }

            l = hm.len();
            s = nk.len();

            // delete all keys in nk
            for k in nk.into_iter() {
                qual.insert(k.clone());
                hm.remove(&k);
            }
        }

        qual.remove(&vi);
        qual

    }

    /// # description
    /// outputs a substitution map for expr.
    ///
    /// If process_out is on, processes the output expression according to known variables.
    pub fn representative_decision_smap(&mut self,expr:Array1<f32>,verbose:bool) -> HashMap<usize,Array1<f32>> {

        // get remaining  ukknown
        let l = expr.len() - 2;
        let s:Array1<f32> = (0..l + 1).into_iter().map(|i| expr[i].clone()).collect();

        let r:Array1<f32> = self.running_soln_of_sample(s.clone());
        let u2 = self.remaining_unknown_of_sample(s.slice(s![0..l]).to_owned(),r);
        let ui_:Array1<usize> = mat2sort::active_indices(u2);

        // get submatrices
        let rs = self.relevance_submatrix(ui_.clone());
        let rs2 = self.rr_submatrix(ui_.clone());

        // perform analysis
        let ra = self.representative_analysis(rs2.clone(),ui_.clone());

        if verbose {
            println!("\tfetching s-map by rep decision");
            println!("\t * order {:?}",ui_);
            println!("\t * s-map {:?}",ra);
        }

        // get substitution map
        self.brute_force_cover_vars_by_substitution(expr,ra,verbose)
    }
}

#[allow(non_snake_case)]
pub fn test_sample_BEInt_1() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,1.,1.,1.,1.],
        [1.,0.,0.,0.,0.],
        [0.,0.,1.,0.,0.],
        [0.,0.,0.,1.,0.],
        [1.,1.,0.,0.,0.]]),
        arr1(&[1.,30.,21.,32.,47.]))
}

#[allow(non_snake_case)]
pub fn test_sample_BEInt_2() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,1.],[1.,0.],[1.,1.]]),
    arr1(&[24.,32.,133.]))
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub fn test_sample_BEInt_4() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,0.,1.,1.,1.,0.],
        [1.,0.,0.,1.,0.,0.],
        [0.,0.,1.,0.,0.,1.],
        [0.,0.,0.,1.,1.,1.],
        [1.,1.,0.,0.,0.,1.]]),
        arr1(&[70.,130.,263.,312.,474.]))
}

#[allow(non_snake_case)]
pub fn test_sample_BEInt_5() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[0.,0.,1.],
        [1.,0.,1.],
        [0.,1.,1.],
        [1.,0.,0.],
        [1.,1.,0.]]),
        arr1(&[7.,14.,13.,5.,27.]))
        //arr1(&[7.,1301.,26325.,3012.,1474.]))
}

#[allow(non_snake_case)]
pub fn test_sample_BEInt_6() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[1.,1.,0.],[0.,1.,1.]]),
        arr1(&[70.,1140.]))
}

#[allow(non_snake_case)]
pub fn test_sample_BEInt_7() -> (Array2<f32>,Array1<f32>) {
    (arr2(&[[1.,1.,0.,0.,0.,0.],
            [0.,0.,1.,1.,0.,0.],
            [0.,0.,0.,0.,1.,1.]]), arr1(&[52.,92.,107.]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_BEInt_order_bfs() {
        let (x,y):(Array2<f32>,Array1<f32>) = test_sample_BEInt_1();
        let mut be = build_BEInt(x,y);
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
        let stat = bei.solve_at(x.dim().0 - 1,true,0);
        assert!(stat);
        assert_eq!(bei.contradictions_in_range(0,x.dim().0 -1,true,false).len(),0);

        //// case 2
        /*
        let (x2,y2):(Array2<f32>,Array1<f32>) = test_sample_BEInt_4();
        let mut bei2 = build_BEInt(x2.clone(),y2);
        bei2.order_bfs();

        let stat = bei2.solve_at(x2.dim().0 - 1,true,0);
        println!("soln: {:?}",bei2.r_soln);
        println!("stat: {}",stat);
        assert!(stat);
        assert_eq!(bei2.contradictions_in_range(0,x2.dim().0 -1,true,false).len(),0);
        */

        //// case 3
        /*
        let (x3,y3):(Array2<f32>,Array1<f32>) = test_sample_BEInt_5();
        let mut bei3 = build_BEInt(x3.clone(),y3);
        bei3.order_bfs();

        let stat = bei3.solve_at(x3.dim().0 - 1,true,0);
        println!("soln: {:?}",bei3.r_soln);
        println!("stat: {}",stat);
        assert!(stat);
        assert_eq!(bei3.contradictions_in_range(0,x3.dim().0 -1,true,false).len(),0);
        */
    }

    #[test]
    fn test_BEInt_representative_table() {
        let (x,y):(Array2<f32>,Array1<f32>) = test_sample_BEInt_4();
        let mut bei = build_BEInt(x.clone(),y);
        bei.order_bfs();

        let rt = bei.representative_table();
        let rt2:HashMap<usize,Array1<f32>> = HashMap::from(
            [(1,arr1(&[1.0, 0.0, 0.0, 0.0, 0.0, 1.0])),
            (0,arr1(&[0.0, 1.0, 0.0, 1.0, 0.0, 1.0])),
            (3,arr1(&[1.0, 0.0, 1.0, 0.0, 1.0, 1.0])),
            (4,arr1(&[0.0, 0.0, 1.0, 1.0, 0.0, 1.0])),
            (2,arr1(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0])),
            (5,arr1(&[1.0, 1.0, 1.0, 1.0, 1.0, 0.0]))]);
        assert_eq!(rt,rt2);
    }
}
