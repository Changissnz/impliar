//! batch correcting algorithm used for factorization of skews
use crate::metrice::{btchcorrctrc,btchcorrctr_tc,vreducer};
use crate::enci::{skew,skewf32,mat2sort};
use crate::setti::dessi;
use ndarray::{arr1,Array1};
use std::collections::{HashMap,HashSet};

/// # return
/// 
pub fn apply_skewf32_type_a_variable_size(mut s: skewf32::SkewF32,v:Array1<f32>) -> Array1<f32> {
    let l1 = s.sk.addit.clone().unwrap().len();
    let l2 = v.len();
    let q:Array1<f32> = Array1::zeros(l1);
    let s2:Array1<f32> = s.skew_value(q);
    if l1 < l2 {
        // first chunk
        let mut v2:Array1<f32> = (0..l1).into_iter().map(|x| v[x].clone()).collect();
        let mut v3:Vec<f32> = (s2.clone() + v2.clone()).into_iter().collect(); 
        
        // remainder
        for i in (l1..l2) {
            v3.push(v[i].clone());
        }
        return v3.into_iter().collect();
    } else if l2 < l1 {
        let mut v2:Array1<f32> = (0..l2).into_iter().map(|x| s2[x].clone()).collect();
        return v + v2;
    }

    s.skew_value(v)
}

/// <btchcorrctr::GBatchCorrector> is a data structure used
/// to refactor batches of skew data given their corresponding references.
/// For each batch, structure saves all a-candidate (adders) and m-candidate (multers)  
/// scores into its memory, and updates its memory of existing candidates and new candidates
/// for every next batch.
///
/// - To load a new batch, call `.load_next_batch(...)`.
/// 
/// - To find the best factor candidate from the beginning, call `.refactor(...)`.
///
/// - To keep track of the best factor candidate after every batch, call `.process_batch_(...)`.
#[derive(Clone)]
pub struct GBatchCorrector {
    /// all addit skews
    sb: Vec<skewf32::SkewF32>,
    /// addit skews in batch
    pub b: Vec<skewf32::SkewF32>,
    /// all reference vectors, operand priori
    pub refn: Vec<Array1<f32>>,
    /// batch reference vectors, operand priori
    pub refn1: Vec<Array1<f32>>,
    /// (best factor, score after, factor is adder?)
    pub best_refactor: (Option<i32>,Option<f32>,bool),
    /// multer candidate scores
    pub m_candidate_scores:HashMap<i32,f32>,
    /// adder candidate scores
    pub a_candidate_scores:HashMap<i32,f32>,
    /// decimal places
    k:usize,
    /// vreducer
    pub vr: vreducer::VRed,
    pub skew_mtr:f32,
    /// stat on if refactor has been updated; used for case of `.load_next_batch(...)`.
    pub refactor_update:bool
}

pub fn empty_GBatchCorrector(vr: vreducer::VRed,k:usize) -> GBatchCorrector {
    GBatchCorrector{sb:Vec::new(),b:Vec::new(),refn:Vec::new(),refn1:Vec::new(),
        best_refactor:(None,None,true),m_candidate_scores:HashMap::new(),
        a_candidate_scores:HashMap::new(),k:k,vr:vr,skew_mtr:0.,refactor_update:false}
}

impl GBatchCorrector {

    /// # return
    /// number of samples that have been processed
    pub fn sample_size(&mut self) -> usize {
            self.refn.len()
    }

    /// # return
    /// number of candidates
    pub fn candidate_size(&mut self) -> usize {
        self.m_candidate_scores.len() + self.a_candidate_scores.len()
    }

    /// # description
    /// loads the next batch into 
    pub fn load_next_batch(&mut self,sb: Vec<skewf32::SkewF32>,refn: Vec<Array1<f32>>) {
        self.b = sb;
        self.refn1 = refn;
        assert!(self.is_proper_batch());
    }

    /// # return 
    /// all .s is equal for new batch `b`?
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

    /// # description
    /// converts all <skewf32::SkewF32> into <skew::Skew> over the
    /// batch specified by `is_batch`
    ///
    /// # arguments
    /// is_batch := bool, true b false sb
    pub fn bare_skew(&mut self,is_batch:bool) -> Vec<skew::Skew> {

        let mut q: Vec<skewf32::SkewF32> = if is_batch {self.b.clone()} else {self.sb.clone()};
        q.clone().into_iter().map(|x| x.sk).collect()
    }

    /// # description
    /// scales reference `refn1` if argument `ref_1 == True`, otherwise 
    /// scales reference `refn`. Output is a sequence of <arr1\<i32\>> 
    /// that is the sequence of <arr1\<f32\>> * `k`. 
    ///
    /// # return
    /// the scaled reference 
    pub fn scale_ref(&mut self,ref_1:bool) -> Vec<Array1<i32>> {
        let x = if ref_1 {self.refn1.clone()} else {self.refn.clone()};
        let mut sol: Vec<Array1<i32>> = Vec::new();
        for x_ in x.into_iter() {
            sol.push(dessi::scale_arr1_f32_to_arr1_i32(x_,self.k));
        }
        sol
    }

    /// # return
    /// ((best factor),(candidate score),candidate is adder)
    pub fn process_batch(&mut self,b:bool,verbose:bool) -> (Option<i32>,Option<f32>,bool) {
        if b && self.b.len() == 0 {
            return (None,None,true);
        }

        if !b && self.sb.len() == 0 {
            return (None,None,true);
        }
        
        let (x1,x2) = self.afactor_on_batch(b,verbose);
        let (y1,y2) = self.mfactor_on_batch(b,verbose);
        if x2.unwrap() < y2.unwrap() {(x1,x2,true)} else {(y1,y2,false)}
    }

    /// # description
    /// pushes the batch data `b` into `sb` and `refn1` into `refn`
    pub fn push_batch(&mut self) {
        let l = self.b.len();
        for i in 0..l {
            self.sb.push(self.b[0].clone());
            self.refn.push(self.refn1[0].clone());

            // update skew_mtr
            self.skew_mtr += self.b[0].clone().skew_size();

            self.b = self.b[1..].to_vec();
            self.refn1 = self.refn1[1..].to_vec();
        }
    }

    /// # description
    /// calculates the best (m|a)-factor for all batch samples and then calls the
    /// `push_batch` function.
    pub fn process_batch_(&mut self,verbose:bool) {
        if self.b.len() == 0 {
            return;
        }
        
        self.best_refactor = self.process_batch(true,verbose);
        self.refactor_update = true;
        self.push_batch();
    }

    /// # description
    /// updates `a_candidate_score` map with adder `c`
    /// 
    /// # arguments
    /// v1_ := hashmap of adder to score
    /// c := adder 
    ///
    /// # return
    /// score of a-factor `c` on all samples (sb + b) 
    pub fn process_candidate_adder(&mut self,v1_: HashMap<i32,f32>,c:i32) -> f32 {

        if self.a_candidate_scores.contains_key(&c) {
            // process c on self.b
            let (s1,s2,s3) = btchcorrctrc::a_refactor_skewf32_batch_type_a(self.bare_skew(true),self.k,c);

            // add score minus (c / (10 ** k))
            let c_: f32 = (c as f32) / f32::powf(10.,self.k as f32);
            *self.a_candidate_scores.get_mut(&c).unwrap() += s3;// - c_;
        }  else {
            // do c on all self.sb
            let (s1,s2,s3) = btchcorrctrc::a_refactor_skewf32_batch_type_a(self.bare_skew(false),self.k,c);
            self.a_candidate_scores.insert(c,v1_.get(&c).unwrap() + s3);
        }
        *(self.a_candidate_scores.get(&c).clone().unwrap())
    }

    /// # description
    /// updates `m_candidate_score` map with adder `c`
    /// 
    /// # arguments
    /// v1_ := hashmap of multer to score
    /// c := multer 
    ///
    /// # return
    /// score of m-factor `c` on all samples (sb + b) 
    pub fn process_candidate_multer(&mut self,v1_:HashMap<i32,f32>,c:i32) -> f32 {
        let c_:f32 = c as f32 / f32::powf(10.,self.k as f32);
        if self.m_candidate_scores.contains_key(&c) {
            // process c on self.b
            let (skv,av) = self.scale_data(Some(self.k),true);
            let (h1,sb1) = btchcorrctrc::m_refactor_skew_batch_type_a(skv,av,c);
            let s12:i32 = sb1.into_iter().map(|x| x.skew_size as i32).into_iter().sum::<i32>();//h1.skew_size as i32;

            // add score
            *self.m_candidate_scores.get_mut(&c).unwrap() += s12 as f32 / f32::powf(10.,self.k as f32);
        }  else {
            // do c on all self.sb
            let (skv,av) = self.scale_data(Some(self.k),false);
            let (h1,sb1) = btchcorrctrc::m_refactor_skew_batch_type_a(skv,av,c);
            let s12:i32 = sb1.into_iter().map(|x| x.skew_size as i32).into_iter().sum::<i32>();//h1.skew_size as i32;
            self.m_candidate_scores.insert(c,v1_.get(&c).unwrap() + s12 as f32 / f32::powf(10.,self.k as f32));
            //*self.m_candidate_scores.get_mut(&c).unwrap() += c as f32;
        }
        *(self.m_candidate_scores.get(&c).clone().unwrap())
    }

    
    /// # description
    /// performs an m-factor search on batch data, and fetches the m-factor with the least score
    /// 
    /// # return
    /// (best m-factor, m-factor score)
    pub fn mfactor_on_batch(&mut self,b:bool,verbose:bool) -> (Option<i32>,Option<f32>) {
        let (mut y,mut y2): (Option<i32>,Option<f32>) = (None,Some(f32::MAX));
        if verbose {println!("\tm-factor on batch")};

        // get scores on batch
        let (skv,av) = self.scale_data(Some(self.k),true);

            //let mx:i32 = i32::pow(10,self.k as u32);
        let csvec = btchcorrctrc::multiple_score_pair_vec_on_skew_batch_type_a(skv,av,None);
        let mut v1:HashMap<i32,f32> = HashMap::from_iter(csvec.into_iter().map(|x| (x.0,x.1 as f32 / f32::powf(10.,self.k as f32))).into_iter());

        // remove 1
        v1.remove(&0);

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

    /// # description
    /// performs an a-factor search on batch data, and fetches the a-factor with the least score
    /// 
    /// # return
    /// (best a-factor, a-factor score)
    pub fn afactor_on_batch(&mut self,r:bool,verbose:bool) -> (Option<i32>,Option<f32>) {
        let (mut y,mut y2): (Option<i32>,Option<f32>) = (None,Some(f32::MAX));

        // collect all candidates
        let mut candidates:HashSet<i32> = self.a_candidate_scores.clone().into_keys().collect();
        if verbose {println!("\ta-factor on batch")};

        let mut v1: HashMap<i32,f32> = HashMap::new();        
        if r {
            v1 = HashMap::from_iter(btchcorrctrc::adder_score_pair_vec_on_skew_batch_type_a(self.b.clone()).0.into_iter());
        } else {
            v1 = HashMap::from_iter(btchcorrctrc::adder_score_pair_vec_on_skew_batch_type_a(self.sb.clone()).0.into_iter());
        }
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

    /// # description
    /// searches for the a-factor with the lowest score and the m-factor
    /// with the lowest score. Outputs the refactorization that produces the lowest
    /// score 
    ///
    /// # return
    /// lower-scoring refactorization of the batch data
    pub fn refactor(&mut self) -> (Option<skewf32::SkewF32>,Vec<skewf32::SkewF32>,f32) {
        let (s11,s12,s13) = self.best_a();
        let (s21,s22,s23) = self.best_m();
        if s13 < s23 {
            return (s11,s12,s13);
        }
        (s21,s22,s23)
    }

    /// # description
    /// refactors using the variable `best_refactor` and outputs
    pub fn refactor_(&mut self) {
        if !self.refactor_update {
            self.best_refactor = self.process_batch(false,false);
        }

        let (mut bf1, mut bf2, mut bf3) = self.best_refactor.clone();  

        // case: no best refactor
        if bf1.is_none() {
            return;
        } 

        // update data
        self.update_data(bf1.clone().unwrap(),bf3);

        // add skew to VReducer
        let q = if bf3 {vreducer::sample_vred_adder_skew(bf1.unwrap(),self.k)} else {vreducer::sample_vred_multer_skew(bf1.unwrap())};
        self.vr.mod_tailn_(q);
        //self.vr.add_s(q);

        // update skew_mtr
        self.skew_mtr = bf2.unwrap();

        // clear
        self.clear_best_refactor();
    }

    pub fn best_candidate_for_refactor(&mut self) -> (i32,f32,bool) {
        
        // best multer
        let a1:(i32,f32) = self.a_candidate_scores.clone().into_iter().fold((0,f32::MAX),|x,x2| if x.1 < x2.1 {x} else {x2});

        // best adder        
        let m1:(i32,f32) = self.m_candidate_scores.clone().into_iter().fold((0,f32::MAX),|x,x2| if x.1 < x2.1 {x} else {x2});

        if a1.1 < m1.1 { return (a1.0,a1.1,true);}
        (m1.0,m1.1,false)

    }

    pub fn clear_best_refactor(&mut self) {
        if self.best_refactor.0.is_none() {
            return;
        }

        self.a_candidate_scores = HashMap::new();
        self.m_candidate_scores = HashMap::new();
        self.refactor_update = false;
    }

    /// # description
    /// updates skews `sb` and reference values `refn` with adder|multer x.
    pub fn update_data(&mut self,x:i32,a:bool) {

        // get the actual value of x
        // case: adder -> scale x by 10^-5
        // case: multer -> literal x
        let x2: f32 = if a {x as f32 / (i32::pow(10,self.k as u32) as f32)} else {x as f32};

        // get the wanted values: `refn` + `sb`
        let mut wanted_values: Vec<Array1<f32>> = self.refn.clone().into_iter().enumerate().map(|(i,y)| self.sb[i].skew_value(y)).collect();

        // update reference with value x
        // case: add x
        if a {
            self.refn = self.refn.clone().into_iter().map(|y| y + x2).collect();
        } else {
        // case: multiply x
            self.refn = self.refn.clone().into_iter().map(|y| y * x2).collect();
        }

        // update skews
        self.sb = wanted_values.into_iter().enumerate().map(|(i,y)| vreducer::sample_vred_addit_skew(y - self.refn[i].clone(),self.k)).collect();
    }

    /// # description
    /// best a-factor for batch `sb`
    ///
    /// # return
    /// (a-factor skew,refactored skew, score of refactored solution)
    pub fn best_a(&mut self) -> (Option<skewf32::SkewF32>,Vec<skewf32::SkewF32>,f32) {
        btchcorrctrc::best_afactor_for_skewf32_batch_type_a(self.sb.clone())
    }

    /// # description
    /// best m-factor for batch `sb`
    ///
    /// # return
    /// (m-factor skew,refactored skew, score of refactored solution)
    pub fn best_m(&mut self) -> (Option<skewf32::SkewF32>,Vec<skewf32::SkewF32>,f32) {
        let (skv,av) = self.scale_data(Some(self.k),false);
        let (bs,ms) = btchcorrctrc::best_multiple_for_skew_batch_type_a(skv.clone(),av.clone());
        let sc1 = ms as f32 / f32::powf(10.,self.k as f32);

        if bs == 0 {
            return (None,self.sb.clone(),sc1);
        }

        let (sk1,sk2) = btchcorrctrc::m_refactor_skew_batch_type_a(skv,av,bs);
        let h1 = skewf32::SkewF32{sk:sk1,s:1};
        let sk3 = sk2.into_iter().map(|x| skewf32::SkewF32{sk:x,s:self.k}).collect();
        (Some(h1),sk3,sc1)
    }

    /// # description
    /// scales <skewf32::SkewF32> data into <skew::Skew> data. Data is
    /// either `b` or `sb` based on `is_batch`. 
    pub fn scale_data(&mut self,scale:Option<usize>,is_batch:bool) -> (Vec<skew::Skew>,Vec<Array1<i32>>) {//(Vec<skew::Skew>,Vec<Array1<i32>>,usize) {
        (self.bare_skew(is_batch),self.scale_ref(is_batch))
    }

    pub fn info(&mut self) {
        println!("# of samples: {}",self.sb.len());
        println!("skew meter: {}",self.skew_mtr);
        println!("best refactor: {:?}",self.best_refactor);
    }

    /// # description
    /// iterates through all references and chooses the reference
    /// `r` with the minimum distance to `vr(s)`. Then applies the skew of
    /// `r` to `vr(s)`, and 
    pub fn map_sample(&mut self,s:Array1<f32>) -> Array1<f32> {
        let (_,yn) = self.vr.apply(s.clone(),1);
        let l = self.refn.len();
        if l == 0 {
            return yn.unwrap();
        }

        let mut default:(usize,&Array1<f32>) = (0,&self.refn[0].clone()); 
        let (x,x2):(usize,&Array1<f32>) = self.refn.iter().enumerate().fold(default,
            |z1:(usize,&Array1<f32>),zn| 
            if mat2sort::euclidean_distance_variable_size(yn.clone().unwrap(),(*zn.1).clone(),1.) < 
                mat2sort::euclidean_distance_variable_size(yn.clone().unwrap(),(*z1.1).clone(),1.)
            {zn} else {z1});

        let r = apply_skewf32_type_a_variable_size(self.sb[x].clone(),yn.clone().unwrap());
        r
    }

    //-------------------------------------
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__GBatchCorrector__process_batch_AND_refactor() {
        let (b1,b2) = btchcorrctr_tc::batch_1();
        let sb1:f32 = b1.clone().into_iter().map(|mut x| x.skew_size()).into_iter().sum::<f32>();

        let mut gbc = empty_GBatchCorrector(vreducer::sample_vred_euclids_reducer(),5);
        gbc.load_next_batch(b1,b2);
        gbc.process_batch(true,false);
        gbc.push_batch();

        let (b21,b22) = btchcorrctr_tc::batch_2();
        gbc.load_next_batch(b21,b22);
        let (c,s,bo) = gbc.process_batch(true,false);
        gbc.push_batch();

        let (s11,s12,s13) = gbc.refactor();//best_a();
        let s4 = (s13 - s.unwrap()).abs();
        assert!(s4 < sb1,"{}",s4);
    }

    #[test]
    pub fn test__GBatchCorrector__process_batch___case5() {

        let (b1,b2) = btchcorrctr_tc::batch_5();
        let mut gbc = empty_GBatchCorrector(vreducer::sample_vred_euclids_reducer(),5);
        gbc.load_next_batch(b1,b2);
        gbc.process_batch_(true);

        /// check for correct a-factor and m-factor keys
        let q:HashSet<i32> = gbc.a_candidate_scores.into_keys().collect();
        let sol_a:HashSet<i32> = HashSet::from_iter(vec![1000000,1380000,0,3000000]);
        assert_eq!(q,sol_a);

        let q2:HashSet<i32> = gbc.m_candidate_scores.into_keys().collect();
        let sol_m:HashSet<i32> = HashSet::from_iter(vec![7,4,1,9,10,12,5,2,8]);
        assert_eq!(q2,sol_m);
    }

    // checks that scores from processing of batches 5 and 4 separately 
    // equal to processing at once.
    #[test]
    pub fn test__GBatchCorrector__process_batches_4_and_5_equal_soln() {
        // load two batches 5 and 4 into a batch corrector
        let (b1,b2) = btchcorrctr_tc::batch_5();
        let mut gbc = empty_GBatchCorrector(vreducer::sample_vred_euclids_reducer(),5);
        gbc.load_next_batch(b1,b2);
        gbc.process_batch_(false);

        let (b1,b2) = btchcorrctr_tc::batch_4();
        gbc.load_next_batch(b1,b2);
        gbc.process_batch_(false);
        
        // load batches 4 and 5 at once
        let (mut b3,mut b4) = btchcorrctr_tc::batch_5();
        let (mut b5,mut b6) = btchcorrctr_tc::batch_4();
        b3.extend(b5);
        b4.extend(b6);
        let mut gbc2 = empty_GBatchCorrector(vreducer::sample_vred_euclids_reducer(),5);
        gbc2.load_next_batch(b3,b4);
        gbc2.process_batch_(false);

        for (k,v) in gbc.a_candidate_scores.into_iter() {
            if gbc2.a_candidate_scores.contains_key(&k) {
                assert_eq!(v,gbc2.a_candidate_scores[&k]);
            }
        }

        for (k,v) in gbc.m_candidate_scores.into_iter() {
            if gbc2.m_candidate_scores.contains_key(&k) {
                assert_eq!(v,gbc2.m_candidate_scores[&k]);
            }
        }
    }


}
