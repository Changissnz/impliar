/*
skew corrector contains functions for improving approach by GorillaIns
*/
/////////////////////////////////////////////////////////////
use crate::setti::matrixf;
use crate::setti::bfngsrch;
use crate::metrice::bmeas;
use crate::setti::dessi;
use std::collections::{HashSet,HashMap};
use ndarray::Array1;
use round::round;

/*
outputs interval points on [0,1] given l labels.
*/
pub fn label_intervals(l:usize) -> Array1<f32> {
    if l == 0 {
        return Array1::zeros(0);
    }

    let mut sol: Vec<f32> = Vec::new();
    let k = 1. / l as f32;
    let mut s:f32 = k / 2.;
    sol.push(s);

    for i in 1..l {
        s += k;
        sol.push(s);
    }
    sol.into_iter().collect()
}

/*
updates the selection rule score by performing function on target li using
bfngsrch.sr.choice
*/
pub fn gorilla_update_selection_rule(sr: &mut bfngsrch::BFGSelectionRule,approach_out: Array1<f32>,
    mut im: HashMap<usize,Vec<usize>>, li:f32) {

    // look at the last row element (pointset label) in choice
    let l = sr.sr.choice.len();
    if l == 0 {
        return;
    }
    let c = sr.sr.choice[l - 1];
    let iv = im.get_mut(&c).unwrap().clone();

    // calculate the cumulative distance of li to each of the iv elements
    let mut s:f32 = 0.;
    for i in iv.into_iter() {
        let x = approach_out[i].clone();
        s += bmeas::bdistance_of_f32pair((x,li),(0.,1.)).abs();
    }

    if sr.score.is_none() {
        sr.score = Some(s);
    } else {
        sr.score = Some(sr.score.unwrap() + s);
    }
}

/*
iterates through and scores
*/
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

pub fn process_bfgsearcher_tailn__labels(approach_out: Array1<f32>,wanted_normaln:Array1<usize>) -> bfngsrch::BFGSearcher {

    // get map : label -> index of wanted normaln
    let hm = matrixf::label_to_iset_map(wanted_normaln.clone().into_iter().collect());
    let li = label_intervals(hm.len());

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

/*
*/
pub fn correction_for_bfgrule_approach_tailn__labels(b: bfngsrch::BFGSelectionRule,approach_out:Array1<f32>,
    wanted_normaln:Array1<usize>) -> Array1<f32> {

    pub fn bounded_cheapest_add_target_i32__(v1:Array1<f32>,li:f32) -> Array1<f32> {
        // convert to i32 form
        let mut x: Vec<f32> = v1.clone().into_iter().collect();
        x.push(li);
        let mut y:usize = x.into_iter().map(|x1| dessi::f32_decimal_length(x1,Some(5))).into_iter().max().unwrap();
        let x2: Array1<i32> = v1.clone().into_iter().map(|x1| (x1 * y as f32) as i32).collect();

        // find cheapest by
        let b2:(i32,i32) = (0, i32::pow(10,y as u32));
        let li2:i32 = (li * y as f32) as i32;
        let mut sol_ = bmeas::bounded_cheapest_add_target_i32_(x2,b2,li2);

        // convert back to f32 form
        let mut sol: Array1<f32> = sol_.into_iter().map(|x| (x as f32) / f32::powf(10.,y as f32)).collect();
        sol
    }

    // re-call function
    let mut hm = matrixf::label_to_iset_map(wanted_normaln.clone().into_iter().collect());
    let li = label_intervals(hm.len());
    let mut soln: Array1<f32> = Array1::zeros(approach_out.len());

    // iterate through interval points
    for (i,l) in li.into_iter().enumerate() {
        // get iset for t=i
        let ist = hm.get_mut(&i).unwrap().clone();
        let sv:Array1<f32> = ist.clone().into_iter().map(|i2| approach_out[i2].clone()).collect();

        // calculate the cheapest
        let v2 = bounded_cheapest_add_target_i32__(sv,l);
        for (i_,i2) in ist.into_iter().enumerate() {
            soln[i2] = v2[i_];
        }
    }

    soln
}

/*
*/
pub fn gorilla_improve_approach_tailn__labels(approach_out: Array1<f32>,wanted_normaln:Array1<usize>) -> bfngsrch::BFGSelectionRule {
    let mut fgs = process_bfgsearcher_tailn__labels(approach_out,wanted_normaln);
    let q = fgs.all_cache[0].clone();
    fgs.all_cache.into_iter().fold(q, |v1: bfngsrch::BFGSelectionRule,v2: bfngsrch::BFGSelectionRule| if v1.score.unwrap() < v2.score.unwrap() {v1} else {v2})
}
