use crate::metrice::btchcorrctrc;
use crate::enci::{skew,skewf32};
use crate::setti::dessi;

use ndarray::{arr1,Array1};
use std::collections::{HashMap,HashSet};

/*
sb is skewf32 batch type a,
ref is the operand priori.
*/
#[derive(Clone)]
pub struct GBatchCorrector {
    sb: Vec<skewf32::SkewF32>,
    b: Vec<skewf32::SkewF32>,

    refn: Vec<Array1<f32>>,
    refn1: Vec<Array1<f32>>,
    pub best_refactor: (Option<i32>,Option<f32>,bool),

    pub m_candidate_scores:HashMap<i32,f32>,
    pub a_candidate_scores:HashMap<i32,f32>,
    k:usize
}

pub fn empty_GBatchCorrector(k:usize) -> GBatchCorrector {
    GBatchCorrector{sb:Vec::new(),b:Vec::new(),refn:Vec::new(),refn1:Vec::new(),
        best_refactor:(None,None,true),m_candidate_scores:HashMap::new(),a_candidate_scores:HashMap::new(),k:k}
}

impl GBatchCorrector {

    pub fn sample_size(&mut self) -> usize {
            self.refn.len()
    }

    pub fn candidate_size(&mut self) -> usize {
        self.m_candidate_scores.len() + self.a_candidate_scores.len()
    }

    pub fn load_next_batch(&mut self,sb: Vec<skewf32::SkewF32>,refn: Vec<Array1<f32>>) {
        self.b = sb;
        self.refn1 = refn;
        assert!(self.is_proper_batch());
    }

    // check that all .s is equal for sb
    pub fn is_proper_batch(&mut self) -> bool {
        for mut b_ in self.b.clone().into_iter() {
            if b_.s != self.k {
                return false;
            }
            // check only active is addit
            if b_.sk.active() != HashSet::from_iter([2]) {
                return false;
            }
        }

        // scale all ref to k
        true
    }

    /*
    is_batch := bool, true b false sb
    */
    pub fn bare_skew(&mut self,is_batch:bool) -> Vec<skew::Skew> {

        let mut q: Vec<skewf32::SkewF32> = if is_batch {self.b.clone()} else {self.sb.clone()};
        q.clone().into_iter().map(|x| x.sk).collect()
    }

    pub fn scale_ref(&mut self,ref_1:bool) -> Vec<Array1<i32>> {
        let x = if ref_1 {self.refn1.clone()} else {self.refn.clone()};
        let mut sol: Vec<Array1<i32>> = Vec::new();
        for x_ in x.into_iter() {
            sol.push(dessi::scale_arr1_f32_to_arr1_i32(x_,self.k));
        }
        sol
    }

    /*
    return:
    - (best candidate),(candidate score),candidate is adder
    */
    pub fn process_batch(&mut self,verbose:bool) -> (Option<i32>,Option<f32>,bool) {
        let (x1,x2) = self.afactor_on_batch(verbose);
        let (y1,y2) = self.mfactor_on_batch(verbose);
        if x2.unwrap() < y2.unwrap() {(x1,x2,true)} else {(y1,y2,false)}
    }

    pub fn push_batch(&mut self) {
        let l = self.b.len();
        for i in 0..l {
            self.sb.push(self.b[0].clone());
            self.refn.push(self.refn1[0].clone());
            self.b = self.b[1..].to_vec();
            self.refn1 = self.refn1[1..].to_vec();
        }
    }

    pub fn process_batch_(&mut self,verbose:bool) {
        self.best_refactor = self.process_batch(verbose);
        self.push_batch();
    }

    /*
    outputs score for adder candidate
    */
    pub fn process_candidate_adder(&mut self,v1_: HashMap<i32,f32>,c:i32) -> f32 {

        if self.a_candidate_scores.contains_key(&c) {
            // process c on self.b
            let (s1,s2,s3) = btchcorrctrc::a_refactor_skewf32_batch_type_a(self.bare_skew(true),self.k,c);

            // add score
            *self.a_candidate_scores.get_mut(&c).unwrap() += s3;
        }  else {
            // do c on all self.sb
            let (s1,s2,s3) = btchcorrctrc::a_refactor_skewf32_batch_type_a(self.bare_skew(false),self.k,c);
            self.a_candidate_scores.insert(c,v1_.get(&c).unwrap() + s3);
        }
        *(self.a_candidate_scores.get(&c).clone().unwrap())
    }

    /*
    outputs score for multer candidate
    */
    pub fn process_candidate_multer(&mut self,v1_:HashMap<i32,f32>,c:i32) -> f32 {
        let c_:f32 = c as f32 / f32::powf(10.,self.k as f32);

        if self.m_candidate_scores.contains_key(&c) {
            // process c on self.b
            let (skv,av) = self.scale_data(Some(self.k),true);
            let (h1,sb1) = btchcorrctrc::m_refactor_skew_batch_type_a(skv,av,c);
            let s12:i32 = sb1.into_iter().map(|x| x.skew_size as i32).into_iter().sum::<i32>() + h1.skew_size as i32;

            // add score
            *self.m_candidate_scores.get_mut(&c).unwrap() += s12 as f32 / f32::powf(10.,self.k as f32);
        }  else {
            // do c on all self.sb
            let (skv,av) = self.scale_data(Some(self.k),false);
            let (h1,sb1) = btchcorrctrc::m_refactor_skew_batch_type_a(skv,av,c);
            let s12:i32 = sb1.into_iter().map(|x| x.skew_size as i32).into_iter().sum::<i32>() + h1.skew_size as i32;
            self.m_candidate_scores.insert(c,v1_.get(&c).unwrap() + s12 as f32 / f32::powf(10.,self.k as f32));
        }
        *self.m_candidate_scores.get_mut(&c).unwrap() += c_;
        *(self.m_candidate_scores.get(&c).clone().unwrap())
    }

    /*
    outputs min multer soln
    */
    pub fn mfactor_on_batch(&mut self,verbose:bool) -> (Option<i32>,Option<f32>) {
        let (mut y,mut y2): (Option<i32>,Option<f32>) = (None,Some(f32::MAX));
        if verbose {println!("\tm-factor on batch")};

        // get scores on batch
        let (skv,av) = self.scale_data(Some(self.k),true);

        let mx:i32 = i32::pow(10,self.k as u32);
        let csvec = btchcorrctrc::multiple_score_pair_vec_on_skew_batch_type_a(skv,av,Some(mx));
        let mut v1:HashMap<i32,f32> = HashMap::from_iter(csvec.into_iter().map(|x| (x.0,x.1 as f32 / f32::powf(10.,self.k as f32))).into_iter());

        // remove 1
        v1.remove(&1);

        let mut candidates:HashSet<i32> = self.m_candidate_scores.clone().into_keys().collect();
        let v1_:HashSet<i32> = v1.clone().into_keys().collect();
        candidates.extend(&v1_);

        // process each candidate
        for c in candidates.into_iter() {
            let x = self.process_candidate_multer(v1.clone(),c);
            if verbose {println!("* {} --> {}",c,x)};

            // check that multiplication does not exceed
            if x < y2.unwrap() {
                y2 = Some(x);
                y = Some(c);
            }
        }
        (y,y2)
    }

    /*
    outputs min adder soln
    */
    pub fn afactor_on_batch(&mut self,verbose:bool) -> (Option<i32>,Option<f32>) {
        let (mut y,mut y2): (Option<i32>,Option<f32>) = (None,Some(f32::MAX));

        // collect all candidates
        let mut candidates:HashSet<i32> = self.a_candidate_scores.clone().into_keys().collect();
        if verbose {println!("\ta-factor on batch")};

        let v1: HashMap<i32,f32> = HashMap::from_iter(btchcorrctrc::adder_score_pair_vec_on_skew_batch_type_a(self.b.clone()).0.into_iter());
        let v2:HashSet<i32> = v1.clone().into_keys().collect();
        candidates.extend(&v2);
        // process each candidate
        for c in candidates.into_iter() {
            let x = self.process_candidate_adder(v1.clone(),c);
            if verbose {println!("* {} --> {}",c,x)};
            if x < y2.unwrap() {
                y2 = Some(x);
                y = Some(c);
            }
        }
        (y,y2)
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    //// methods on entire sb

    pub fn refactor(&mut self) -> (Option<skewf32::SkewF32>,Vec<skewf32::SkewF32>,f32) {
        let (s11,s12,s13) = self.best_a();
        let (s21,s22,s23) = self.best_m();
        if s13 < s23 {
            return (s11,s12,s13);
        }
        (s21,s22,s23)
    }

    /*
    best a-factor for batch
    */
    pub fn best_a(&mut self) -> (Option<skewf32::SkewF32>,Vec<skewf32::SkewF32>,f32) {
        btchcorrctrc::best_afactor_for_skewf32_batch_type_a(self.sb.clone())
    }

    /*
    best m-factor for batch
    */
    pub fn best_m(&mut self) -> (Option<skewf32::SkewF32>,Vec<skewf32::SkewF32>,f32) {
        let (skv,av) = self.scale_data(Some(self.k),false);
        let (bs,ms) = btchcorrctrc::best_multiple_for_skew_batch_type_a(skv.clone(),av.clone());
        let sc1 = ms as f32 / f32::powf(10.,self.k as f32);

        if bs == 0 {
            return (None,self.sb.clone(),sc1);
        }

        let (sk1,sk2) = btchcorrctrc::m_refactor_skew_batch_type_a(skv,av,bs);
        let h1 = skewf32::SkewF32{sk:sk1,s:self.k};
        let sk3 = sk2.into_iter().map(|x| skewf32::SkewF32{sk:x,s:self.k}).collect();
        (Some(h1),sk3,sc1)
    }

    pub fn scale_data(&mut self,scale:Option<usize>,is_batch:bool) -> (Vec<skew::Skew>,Vec<Array1<i32>>) {//(Vec<skew::Skew>,Vec<Array1<i32>>,usize) {
        (self.bare_skew(is_batch),self.scale_ref(is_batch))
    }
}

pub fn batch_1() -> (Vec<skewf32::SkewF32>,Vec<Array1<f32>>) {

     // scale by k = 5
     let k:usize = 5;

     let r1 = arr1(&[3.513,4.221,5.4646,6.88,20.5]);
     let r2 = arr1(&[1.,2.222,3.001,4.5]);
     let r3 = arr1(&[30.30303,16.5,25.1]);

     let s1 = arr1(&[15.,16.,22.2,7.1,14.]);
     let s2 = arr1(&[5.,6.5,10.5,9.5]);
     let s3 = arr1(&[80.1,50.3,67.5]);
     let sk1 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s1,k)),
                    None,vec![2],None);
    let sk2 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s2,k)),
                   None,vec![2],None);
    let sk3 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s3,k)),
                  None,vec![2],None);
    let s1k = skewf32::SkewF32{sk:sk1,s:k};
    let s2k = skewf32::SkewF32{sk:sk2,s:k};
    let s3k = skewf32::SkewF32{sk:sk3,s:k};

     (vec![s1k,s2k,s3k],vec![r1,r2,r3])
}

pub fn batch_2() -> (Vec<skewf32::SkewF32>,Vec<Array1<f32>>) {
    let k:usize = 5;

    let r1 = arr1(&[2.,4.,2.,4.]);
    let r2 = arr1(&[5.,7.,82.]);
    let r3 = arr1(&[12.,35.,83.]);
    let r4 = arr1(&[1.,21.]);

    let s1 = arr1(&[4.,16.,4.,16.]);
    let s2 = arr1(&[12.,35.,83.]);
    let s3 = arr1(&[70.,150.,20.]);
    let s4 = arr1(&[21.,84.7]);

    let sk1 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s1,k)),
                   None,vec![2],None);
    let sk2 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s2,k)),
                  None,vec![2],None);
    let sk3 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s3,k)),
                 None,vec![2],None);
    let sk4 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s4,k)),
       None,vec![2],None);

    let s1k = skewf32::SkewF32{sk:sk1,s:k};
    let s2k = skewf32::SkewF32{sk:sk2,s:k};
    let s3k = skewf32::SkewF32{sk:sk3,s:k};
    let s4k = skewf32::SkewF32{sk:sk4,s:k};

    (vec![s1k,s2k,s3k,s4k],vec![r1,r2,r3,r4])
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__GBatchCorrector__process_batch_AND_refactor() {
        let (b1,b2) = batch_1();
        let sb1:f32 = b1.clone().into_iter().map(|mut x| x.skew_size()).into_iter().sum::<f32>();

        let mut gbc = empty_GBatchCorrector(5);
        gbc.load_next_batch(b1,b2);
        gbc.process_batch(false);
        gbc.push_batch();
        //println!("---");

        let (b21,b22) = batch_2();
        gbc.load_next_batch(b21,b22);
        let (c,s,bo) = gbc.process_batch(false);
        //println!("{:?} {:?} {}",c,s,bo);
        gbc.push_batch();

        let (s11,s12,s13) = gbc.refactor();//best_a();
        //println!("{} {}",s11.unwrap(),s13);
        let s4 = (s13 - s.unwrap()).abs();
        //println!("{:?}",s4);
        assert!(s4 < sb1,"{}",s4);
    }

}
