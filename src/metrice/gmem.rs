use crate::metrice::btchcorrctr;
use std::cmp::Ordering;
use std::collections::HashMap;
use ndarray::{arr1,Array1};
use crate::enci::{skewf32};

pub fn i32_3tuple_cmp1(s1: &(i32,f32,usize),s2: &(i32,f32,usize)) -> std::cmp::Ordering {
    if (*s1).1 <= (*s2).1 {
        return Ordering::Less;
    }
    Ordering::Greater
}

/*
a skew batch corrector
*/
pub struct GBCMem {

    // specs for GMem
    sample_cap:usize,
    candidate_cap:usize,
    bc: btchcorrctr::GBatchCorrector
}

pub fn build_GBCMem(sc:usize,cc:usize,k:usize) -> GBCMem {
    GBCMem{sample_cap:sc,candidate_cap:cc,bc: btchcorrctr::empty_GBatchCorrector(k)}
}

impl GBCMem {

    pub fn load_next_batch(&mut self,sb: Vec<skewf32::SkewF32>,refn: Vec<Array1<f32>>) -> bool {
        if sb.len() + self.bc.sample_size() > self.sample_cap {
            return false;
        }
        self.bc.load_next_batch(sb,refn);
        self.filter_candidates();
        true
    }

    pub fn filter_candidates(&mut self) {

        if self.bc.candidate_size() < self.candidate_cap {
            return;
        }

        // collect all candidates into Vec<(i32 candidate,f32 score,is adder usize)>
        let mut cs: Vec<(i32,f32,usize)> = Vec::new();
        for (x,x2) in self.bc.m_candidate_scores.clone().into_iter() {
            cs.push((x,x2,0));
        }

        for (x,x2) in self.bc.a_candidate_scores.clone().into_iter() {
            cs.push((x,x2,1));
        }

        cs.sort_by(i32_3tuple_cmp1);
        self.bc.m_candidate_scores = HashMap::new();
        self.bc.a_candidate_scores = HashMap::new();
        for i in 0..self.candidate_cap {
            if cs[i].2 == 0 {
                self.bc.m_candidate_scores.insert(cs[i].0,cs[i].1);
            } else {
                self.bc.a_candidate_scores.insert(cs[i].0,cs[i].1);
            }
        }
    }


}
