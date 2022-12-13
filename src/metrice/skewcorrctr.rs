//! skew corrector contains functions for improving approach by GorillaIns
use crate::setti::matrixf;
use crate::setti::bfngsrch;
use crate::metrice::bmeas;
use crate::setti::dessi;
use std::collections::{HashSet,HashMap};
use ndarray::{arr1,Array1};
use round::round;

/// # description 
/// outputs interval points in \[0,1\] given l labels.
/// - EX: l = 4 -> 
///             [0.125,0.375,0.625,0.875] 
pub fn label_intervals(l:usize) -> Array1<f32> {
    if l == 0 {
        return Array1::zeros(0);
    }

    let mut sol: Vec<f32> = Vec::new();
    let k = 1. / l as f32;
    let mut s:f32 = k / 2.;
    sol.push(s);

    for _ in 1..l {
        s += k;
        sol.push(s);
    }
    sol.into_iter().collect()
}

pub fn label_intervals_by_ordering(ordering:Vec<usize>) -> Array1<f32> {
    let l = label_intervals(ordering.len());
    let mut l2 = Array1::ones(ordering.len());
    for (i,o) in ordering.into_iter().enumerate() {
        l2[i] = l[o];
    }
    l2
}

pub fn wanted_normaln_to_interval_values(wanted_normaln:Array1<usize>,ordering:Vec<usize>) -> Array1<f32> {
    let li = label_intervals_by_ordering(ordering);
    let mut x:Array1<f32> = Array1::zeros(wanted_normaln.len());
    for (i,w) in wanted_normaln.into_iter().enumerate() {
        x[i] = li[w].clone();
    }
    x 
}

/// # description
/// 
pub fn binary_interval_ordering(s:Vec<usize>,wanted_normaln:Array1<usize>) -> Vec<usize> {
    assert!(!(s.len() > 2), "invalid ordering for binary labels");

    
    if s.len() == 2 { return s;}
    let h:usize = wanted_normaln[0].clone();

    let q:usize = (s[0] + 1) % 2;
    // case: append q
    if h == 0 {
        let s_:Vec<usize> = vec![s[0].clone(),q];
        return s_;
    }

    // case: prepend q
    let s_:Vec<usize> = vec![q,s[0].clone()];
    s_
    
}

/// # description
/// updates the selection rule score by performing function on target li using
/// bfngsrch.sr.choice
pub fn gorilla_update_selection_rule(sr: &mut bfngsrch::BFGSelectionRule,approach_out: Array1<f32>,
    mut im: HashMap<usize,Vec<usize>>, li:f32) {
    ////////////
    
    // look at the last row element (pointset label) in choice
    let l = sr.sr.choice.len();
    if l == 0 {
        return;
    }

    let c = sr.sr.choice[l - 1];
    let mut iv:Vec<usize> = Vec::new();
    if im.contains_key(&c) {
        iv = im.get_mut(&c).unwrap().clone();
    } else {
        println!("[!] CAUTION: no key for gorilla-update on selection rule");
    }

    // calculate the cumulative distance of li to each of the iv elements
    let mut s:f32 = 0.;
    for i in iv.into_iter() {
        let x = approach_out[i].clone();
        s += (x - li).abs();
    }

    if sr.score.is_none() {
        sr.score = Some(s);
    } else {
        sr.score = Some(sr.score.unwrap() + s);
    }
}

/// # description
/// iterate through <bfngsrch::BFGSearcher> and scores each <bfngsrch::BFGSelectionRule> 
///
/// # arguments
/// - bs := target BFGSearcher 
/// - approach_out := transformed values before labelling
/// - im := key is label, value is vector of indices corresponding to `li`
/// - li := interval label values (typically in range 0-1)
///         EX: if 2 labels and range is \[0,1\], then \[0.25,0.75\]
pub fn score_bfgs_tmpcache(bs: &mut bfngsrch::BFGSearcher,approach_out: Array1<f32>,
    im: HashMap<usize,Vec<usize>>,li:Array1<f32>) {
    
    let l = bs.tmp_cache.len();
    for i in 0..l {
        let mut s = bs.tmp_cache[i].clone();

        // get pertinent label interval point
        let lip:f32 = li[s.sr.choice.len() - 1].clone();
        gorilla_update_selection_rule(&mut s,approach_out.clone(),im.clone(),lip);
        bs.tmp_cache[i] = s;
    }
}

/// # description
/// helper method for <skewcorrctr::gorilla_improve_approach_tailn__labels>
pub fn process_bfgsearcher_tailn__labels(approach_out: Array1<f32>,wanted_normaln:Array1<usize>,l:usize) -> bfngsrch::BFGSearcher {
    let x:usize = HashSet::<usize>::from_iter(wanted_normaln.clone()).len();
    assert!(x <= l,"invalid `l`");

    // get map : label -> index of wanted normaln
    let hm = matrixf::label_to_iset_map(wanted_normaln.clone().into_iter().collect());
    let li = label_intervals(l);

    let bfgsr = bfngsrch::default_BFGSelectionRule(hm.len(),li.len());
    let mut fgs = bfngsrch::build_BFGSearcher(bfgsr);
    let mut stat:bool = true;

    // brute-force approach
    while stat {
        // search for next label interval
        stat = fgs.process();
        if !stat { continue;}

        // score each in tmp_cache
        score_bfgs_tmpcache(&mut fgs,approach_out.clone(),hm.clone(),li.clone());
        fgs.next_srs(None);
    }
    fgs
}

/// # description
/// calculates the cheapest array f that when added to `approach_out` results in a labelling ~ `wanted_normaln`. 
/// 
/// # note
/// code could be simplified
///
/// # return
/// (skew correction vector, wanted f32 vector)
pub fn correction_for_bfgrule_approach_tailn__labels(ordering:Vec<usize>,approach_out:Array1<f32>,
    wanted_normaln:Array1<usize>) -> (Array1<f32>,Array1<f32>) {
    let wanted = wanted_normaln_to_interval_values(wanted_normaln.clone(),ordering);
    (wanted.clone() - approach_out,wanted.clone())
}

/// # description
/// calculates the best <bfngsrch::BFGSelectionRule> such that for `approach_out` and its `wanted_normaln`,
/// the rule is the best `label -> f32` mapping, in which `label = unique(wanted_normaln)`. 
pub fn gorilla_improve_approach_tailn__labels(approach_out: Array1<f32>,wanted_normaln:Array1<usize>,l:usize) -> bfngsrch::BFGSelectionRule {
    let fgs = process_bfgsearcher_tailn__labels(approach_out,wanted_normaln,l);    
    let q = fgs.all_cache[0].clone();
    fgs.all_cache.into_iter().fold(q, |v1: bfngsrch::BFGSelectionRule,v2: bfngsrch::BFGSelectionRule| if v1.score.unwrap() < v2.score.unwrap() {v1} else {v2})
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_correction_for_bfgrule_approach_tailn__labels() {
        let ao:Array1<f32> = arr1(&[0.05,0.2,0.3,0.32,0.4,0.5,0.7,0.8]);
        let l:Array1<usize> = arr1(&[0,1,0,1,0,1,0,1]);
        let mut bfgsr = gorilla_improve_approach_tailn__labels(ao.clone(),l.clone(),2);
        
        bfgsr.sr.choice = binary_interval_ordering(bfgsr.sr.choice.clone(),l.clone());
        let (corr,_) = correction_for_bfgrule_approach_tailn__labels(bfgsr.sr.choice.clone(),ao.clone(),l.clone());

        let sol1:Array1<f32> = arr1(&[0.2, 0.55, -0.05, 0.43, -0.15, 0.25, -0.45, -0.05]);
        
        assert!((corr.clone() - sol1).sum() < 8. * f32::powf(10.,-6.));

        let mut x = ao + corr;
        let sol2:Array1<f32> = arr1(&[0.25, 0.75, 0.25, 0.75, 0.25, 0.75, 0.25, 0.75]);
        assert_eq!(x,sol2);
    }

    #[test]
    pub fn test_gorilla_improve_approach_tailn__labels() {
        // case 1: binary labels
        let a:Array1<f32> = arr1(&[0.2,0.4,0.3,0.1,0.5]);
        let b: Array1<usize> = arr1(&[1,0,0,1,0]);
        let mut bfgs = gorilla_improve_approach_tailn__labels(a,b,2);

        assert!(bfgs.sr.choice == vec![1,0]);

        // case 2: trinary labels
        let a1:Array1<f32> = arr1(&[0.2,0.4,0.3,0.1,0.5]);
        let b1: Array1<usize> = arr1(&[1,0,2,2,1]);        
        let mut bfgs2 = gorilla_improve_approach_tailn__labels(a1,b1,3);
        assert!(bfgs2.sr.choice == vec![2,1,0]);
    }
}
