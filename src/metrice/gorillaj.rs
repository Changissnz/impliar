use crate::metrice::gorillains;
use crate::metrice::vreducer;
use crate::metrice::vcsv;
use crate::metrice::btchcorrctr;

use crate::enci::skew;
use crate::enci::skewf32;
use crate::enci::mat2sort;

use ndarray::{Array1,arr1};
use std::collections::HashSet;
use std::sync::{Arc, Mutex, Once};

/// <gorillaj::GorillaJudge> uses <gorillains::GorillaIns> to
/// learn each sample it processes. Samples are loaded into this
/// data structure by its x-data <vcsv::BatchReader> and ?y-data?
/// <vcsv::BatchReader>. 
///
/// Two metrics are used to gauge the "goodness-of-fit" of the 
/// mapping function `base_vr`:
///         - summation of skew
///         - summation of mis-classification
///
/// Each sample `s` goes through the function `base_vr` and its skew is the vector
///             `I - base_vr(s)`, 
/// `I` is the vector of `f32's in [0,1]` such that each i'th value of `base_vr(s)` should
/// equal the i'th value in `I` for a labelling of `s` with no mis-classification error.
/// 
/// Summation of mis-classification is calculated by adding the score of
/// each <gorillains::GorillaIns> solution (a <fs::FSelect> instance). The
/// `.score` attribute of the <fs::FSelect> instance is its misclassification
/// for the sample.
pub struct GorillaJudge {
    /// x-data reader
    pub brx: vcsv::BatchReader,
    /// y-data reader (for supervised learning)
    pub bry: Option<vcsv::BatchReader>,
    /// is output a vector?
    pub is_tailn: bool,
    /// batch data from x-data reader
    pub data_load: Vec<Array1<f32>>,
    /// batch data from y-data reader (tail-n)
    pub label_loadn:Option<Vec<Array1<f32>>>,
    /// batch data from y-data reader (tail-1)
    pub label_load1:Option<Vec<f32>>,
    /// base VReducer used to map each x sample into its y label
    pub base_vr: vreducer::VRed,
    /// decimal length for skew values; used by <gorillains::GorillaIns>
    k:usize,
    /// range-space size for `BFGSelectionRule`
    reducer_szt:usize,
    /// batch size
    bs:usize,
    /// size of last batch read
    lbs: usize,
    /// tail-n case: skew values for each `base_vr(s)` of `s in x-data`.  
    tail1_skew:Vec<f32>,
    /// tail-n case: skew values for each `base_vr(s)` of `s in x-data`.  
    pub tailn_skew:Vec<skewf32::SkewF32>,
    /// batch corrector for refactoring skews
    pub bc: btchcorrctr::GBatchCorrector,
    /// tail-n case: output from `base_vr` to value before y-label
    pub vr_outputn: Vec<Array1<f32>>,
    /// tail-1 case: output from `base_vr` to value before y-label
    vr_output1: Vec<f32>,
    /// mis-classification score metric
    misclass_mtr: f32,
    /// skew summation score metric
    pub skew_mtr: f32,
    // sequence of adders or multers used in refactoring; these values are used to get the baseline skew of the instance's <vreducer:VRed>.
    //pub skew_values: Vec<f32>
}


pub fn build_GorillaJudge(fp:String,fp2:Option<String>,
    is_tailn: bool, base_vr: vreducer::VRed,k:usize,rs: usize,bs:usize) -> GorillaJudge {

    let mut brx = vcsv::build_BatchReader(fp,bs,false,'_');
    let mut brx2: Option<vcsv::BatchReader> = if fp2.is_none() {None} else 
        {Some(vcsv::build_BatchReader(fp2.unwrap(),bs,!is_tailn,'_'))};
    let mut gbc = btchcorrctr::empty_GBatchCorrector(k);

    GorillaJudge{brx:brx, bry:brx2,is_tailn:is_tailn,data_load:Vec::new(),
        label_loadn:Some(Vec::new()),label_load1:Some(Vec::new()),
        base_vr:base_vr, k:k,reducer_szt:rs,bs:bs,lbs: 0,tail1_skew:Vec::new(),tailn_skew:Vec::new(),
        bc:gbc,vr_outputn:Vec::new(),vr_output1:Vec::new(),misclass_mtr: 0.,skew_mtr:0.}
}

impl GorillaJudge {

    /*
    /// # description
    /// declare a <gorillains::GorillaIns> instance; tail-1 variant has non-null `single` argument
    /// and tail-n has a non-null `multi`.
    pub fn make_GorillaIns(&mut self,single:Option<(Array1<f32>,usize)>,multi:Option<(Array1<f32>,Array1<usize>)>) -> gorillains::GorillaIns {
    }
    */

    ////////
    
    /// # description
    /// loads next batch (x-data, ?y-data?) of size <= `bs` into memory
    /// # return
    /// size of batch
    pub fn load_next_batch(&mut self) -> usize {
        let (_,b1) = self.brx.read_batch_numerical();
        self.data_load = b1.unwrap();

        // case: supervised
        if !self.bry.is_none() {
            let b2 = self.bry.as_mut().unwrap().read_batch_numerical();

            if self.is_tailn {
                self.label_loadn = Some(b2.1.unwrap());
            } else {
                self.label_load1 = Some(b2.0.unwrap());
            }
        }

        self.data_load.len()
    }
    
    /// # description
    pub fn gorilla_on_batch(&mut self) -> usize {
        let l = self.data_load.len();

        for i in 0..l {
            let (x1,x2,x3,x4,x5) = self.process_gorilla_at_index(i);

            // 
            if self.is_tailn {
                let s = x2.clone().unwrap().skew_size();
                self.vr_outputn.push(x4.unwrap());
                self.tailn_skew.push(x2.unwrap());
                self.skew_mtr += s;
            } else {
                self.vr_output1.push(x3.unwrap());
                self.tail1_skew.push(x1.unwrap());
                self.skew_mtr += x1.unwrap();
            }
            
            self.misclass_mtr += x5;
        }
        l

    }

    /// # description
    /// runs a <gorillains::GorillaIns> instance on the i'th sample in data
    ///
    /// # return
    /// (f32 skew for tail-1 case, vector skew for tail-n case, `vred` output-1, `vred` output-n,misclassification score)
    pub fn process_gorilla_at_index(&mut self, i: usize) -> (Option<f32>,Option<skewf32::SkewF32>,Option<f32>,Option<Array1<f32>>,f32) {
        let mut gi = self.gorilla_at_index(i);
        
        let giscore = gi.soln.as_ref().unwrap().score;

        let (x1,x2) = gi.improve_approach__labels(self.is_tailn);
        if !self.is_tailn {
            return (x1,None,gi.app_out1,gi.app_outn,giscore.unwrap());
        }

        (None,Some(vreducer::sample_vred_addit_skew(x2.unwrap(),self.k)),gi.app_out1,gi.app_outn,giscore.unwrap())

    }

    pub fn gorilla_at_index(&mut self,i:usize) -> gorillains::GorillaIns {
        let mut x: Array1<f32> = self.data_load[i].clone();
        let (mut y1,mut yn): (Option<usize>,Option<Array1<usize>>) = (None,None);

        if !self.is_tailn {
            y1 = Some(self.label_load1.as_ref().unwrap()[i].clone() as usize);
        } else {
            yn = Some(self.label_loadn.as_ref().unwrap()[i].clone().into_iter().map(|x| x as usize).collect());
        }

        let mut gi = gorillains::build_GorillaIns(x,self.k,self.base_vr.clone(), yn,y1,
            if !self.is_tailn {0} else {1}, self.reducer_szt);

        if self.is_tailn { gi.brute_process_tailn();} else {gi.process_tail1();};

        gi
    }

    /// # description
    /// 
    /// # return
    /// tail-1 skew if `!is_tailn`
    pub fn refactor(&mut self) -> Option<f32> {
        if self.is_tailn {
            let (_,_,x3,x4) = self.refactor_batch_tailn();
            let q = if x4 {vreducer::sample_vred_adder_skew(x3.unwrap(),self.k)} else {vreducer::sample_vred_multer_skew(x3.unwrap())};
            self.base_vr.add_s(q);
            return None;
        } 
        let (_,_,x3) = self.refactor_batch_tail1();
        Some(x3)
    }

    

    /// # description
    /// 
    /// # return
    /// current skew score, skew score after update, skew factor, is adder?
    pub fn refactor_batch_tailn(&mut self) -> (f32,f32,Option<i32>,bool) {

        // fetch the refactor
        let (x1,x2,x3) = self.bc.best_refactor.clone();

        // case: no best refactor
        if x1.is_none() {
            return (self.skew_mtr.clone(),self.skew_mtr.clone(),None,false);
        }

        let mut q: Vec<Array1<f32>> = self.vr_outputn.clone();
        // wanted values
        let mut x: Vec<Array1<f32>> = q.clone().into_iter().enumerate().map(|(i,x2)| self.tailn_skew[i].skew_value(x2)).collect();
        
        if x3 {

            // calibrate adder
            let x11 = x1.unwrap() as f32 / (i32::pow(10,self.k as u32) as f32);

            // new vr_outputn
            q = q.into_iter().map(|y| y + x11).collect();
        } else {

            // get multer
            let x11 = x1.unwrap() as f32;

            // new vr_outputn
            q = q.into_iter().map(|y| y * x11).collect();
        }

        self.vr_outputn = q.clone();

        // new skew
        self.tailn_skew = x.into_iter().enumerate().map(|(i,y)| vreducer::sample_vred_addit_skew(y - q[i].clone(),self.k)).collect();

        // new score
        let sm = self.skew_mtr.clone();
        self.skew_mtr = x2.unwrap(); 

        (sm,self.skew_mtr.clone(),x1,x3)
    }

    /// # description
    /// uses min,max,mean as points of interest. Refactorization by tail-1 mode
    /// only considers adders (no multers).
    /// 
    /// # return:
    /// current score, score after adder, adder
    pub fn refactor_batch_tail1(&mut self) -> (f32,f32,f32) {
        let (x1,x2,x3) = self.get_refactor_batch_tail1();

        if x3 == 0. {
            return (x1,x2,x3);
        }

        // refactor tail1-skew
        let q = arr1(&self.tail1_skew);
        self.tail1_skew = (q - x3).into_iter().collect();
        self.vr_output1 = (arr1(&self.vr_output1) + x3).into_iter().collect();
        self.skew_mtr = x2;

        (x1,x2,x3)
    }

    
    /// # description
    /// uses min,max,mean as points of interest. Refactorization by tail-1 mode
    /// only considers adders (no multers).
    /// 
    /// # return:
    /// current score, score after adder, adder
    pub fn get_refactor_batch_tail1(&mut self) -> (f32,f32,f32) {

        let q = arr1(&self.tail1_skew);
        let mn_ = q.mean();

        if mn_.is_none() {
            return (0.,0.,0.);
        }
        let mn = mn_.unwrap();

        let minu = self.tail1_skew.iter().fold(f32::MAX, |min, &val| if val < min{ val } else{ min });
        let maxu = self.tail1_skew.iter().fold(f32::MIN, |max, &val| if val > max { val } else{ max });

        /// get new skew scores
        let x1 = mat2sort::abs_sum_arr1_f32(q.clone());
        let x2 = mat2sort::abs_sum_arr1_f32(q.clone() - mn);
        let x3 = mat2sort::abs_sum_arr1_f32(q.clone() - minu);
        let x4 = mat2sort::abs_sum_arr1_f32(q.clone() - maxu);

        let rs = vec![x1,x2,x3,x4];

        // get (index, score) w/ lowest score
        let rsm = rs.into_iter().enumerate().fold((0,x1), |min, val| if val.1 < min.1 { val } else{ min });

        if rsm.0 == 0 {
            return (x1,x1,0.);
        } else if rsm.0 == 1 {
            return (x1,x2,mn);
        } else if rsm.0 == 2 {
            return (x1,x3,minu);
        }

        (x1,x4,maxu)
    }

    /// # description
    /// method called after refactor modification to `base_vr`. 
    pub fn reload_batchcorrctr(&mut self) {
        self.bc = btchcorrctr::empty_GBatchCorrector(self.k);
        self.bc.load_next_batch(self.tailn_skew.clone(),self.vr_outputn.clone());
    }

    /// # description
    /// 
    pub fn update_batchcorrctr(&mut self) {
        // add the last batch of size lbs
        let l = self.vr_outputn.len();
        println!("L: {} BS: {}",l,self.lbs);
        self.bc.load_next_batch(self.tailn_skew[l - self.lbs..l].to_vec().clone(),
            self.vr_outputn[l - self.lbs..l].to_vec().clone());

    }

    /// # description
    /// main function. 
    pub fn process_next(&mut self,refactor:bool) {
        self.lbs += self.load_next_batch();
        if self.lbs > 0 {
            self.gorilla_on_batch();
            if self.is_tailn {
                self.update_batchcorrctr();
                self.bc.process_batch_(true);
            }
        }

        if refactor {
            self.refactor();

            if self.is_tailn {
                self.reload_batchcorrctr();
            }

            self.lbs = 0;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// # description
    /// The <vreducer::VRed> function used by this <gorillaj::GorillaJudge> relies
    /// only on the <vreducer::one_reducer> function.
    /// Calls `process_next` with argument `refactor=False`. Dataset is 
    ///             (x -> `f3_x.csv`,y -> `f3_y2.csv`).
    ///
    /// Checks for appropriate m-factors and a-factors by <btchcorrctr::BatchCorrector>.
    #[test]
    pub fn test__GorillaJudge__process_next___case_1() {
        let sv1: Vec<vreducer::FCast> = vec![vreducer::FCast{f:vreducer::one_reducer}];

        let vr21 = vreducer::build_VRed(sv1,Vec::new(),vec![0],
        0,None,None);
        let mut gj = build_GorillaJudge("src/data/f3_x.csv".to_string(),Some("src/data/f3_y2.csv".to_string()),
            true, vr21,2,2,20);
    
        // second task: add adder_skew vec to VRed and do tail-1
        gj.process_next(false);
                
        let mk:HashSet<i32> = gj.bc.m_candidate_scores.into_keys().collect();
        
        let mut mk_sol:HashSet<i32> = HashSet::from_iter([24,74,49]);
        assert_eq!(mk,mk_sol);
    
        let ak:HashSet<i32> = gj.bc.a_candidate_scores.into_keys().collect();
        let mut ak_sol:HashSet<i32> = HashSet::from_iter([39,74,0]);
        assert_eq!(ak,ak_sol);
    }

}