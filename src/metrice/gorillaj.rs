use crate::metrice::gorillains;
use crate::metrice::vreducer;
use crate::metrice::vcsv;
use crate::metrice::{skewcorrctr,btchcorrctr};

use crate::enci::skew;
use crate::enci::skewf32;
use crate::enci::mat2sort;

use ndarray::{Array1,arr1};
use std::collections::HashSet;

pub fn basic_binary_function(f:f32) -> usize {
    if f < 0.5 {return 0;}
    return 1;
}

/// memory container for tail-1 case
pub struct Tail1Mem {
    pub tail1_skew:Vec<f32>,
    pub vr_output1: Vec<f32>,
    pub misclass_mtr: f32,
    pub skew_mtr: f32
}

impl Tail1Mem {

    pub fn refactor_batch_tail1_(&mut self) -> (f32,f32,f32) {

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
        let rsm = rs.clone().into_iter().enumerate().fold((0,x1), |min, val| if val.1 < min.1 { val } else{ min });

        // update tail1_skew and vr_output1
        let q = arr1(&self.tail1_skew);
        self.tail1_skew = (q - rs[rsm.0].clone()).into_iter().collect();
        self.vr_output1 = (arr1(&self.vr_output1) + rs[rsm.0].clone()).into_iter().collect();

        // fix skew meter before return
        if rsm.0 == 0 {
            return (x1,x1,0.);
        } else if rsm.0 == 1 {
            self.skew_mtr = x2;
            return (x1,x2,mn);
        } else if rsm.0 == 2 {
            self.skew_mtr = x3;
            return (x1,x3,minu);
        }
        self.skew_mtr = x4;
        (x1,x4,maxu)
    }
}


pub fn empty_Tail1Mem() -> Tail1Mem {
    Tail1Mem{tail1_skew:Vec::new(),vr_output1:Vec::new(),misclass_mtr:0.,skew_mtr:0.}
}

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
    
    /// batch size
    bs:usize,
    /// size of last batch read
    lbs: usize,
    
    /// batch corrector for refactoring skews, interval ordering \[0.25,0.75\]
    pub bc: btchcorrctr::GBatchCorrector,
    /// batch corrector for refactoring skews, interval ordering \[0.75,0.25\]
    pub bc2: btchcorrctr::GBatchCorrector,    

    pub tm: Tail1Mem,

    pub misclass_mtr:f32


}

pub fn build_GorillaJudge(fp:String,fp2:Option<String>,
    is_tailn: bool, base_vr: vreducer::VRed,k:usize,bs:usize) -> GorillaJudge {

    let mut brx = vcsv::build_BatchReader(fp,bs,false,'_');
    let mut brx2: Option<vcsv::BatchReader> = if fp2.is_none() {None} else 
        {Some(vcsv::build_BatchReader(fp2.unwrap(),bs,!is_tailn,'_'))};
    let mut gbc = btchcorrctr::empty_GBatchCorrector(base_vr.clone(),k);
    let mut gbc2 = btchcorrctr::empty_GBatchCorrector(base_vr.clone(),k);

    let tm = empty_Tail1Mem(); 

    GorillaJudge{brx:brx, bry:brx2,is_tailn:is_tailn,data_load:Vec::new(),
        label_loadn:Some(Vec::new()),label_load1:Some(Vec::new()),
        base_vr:base_vr, k:k,bs:bs,lbs: 0,bc:gbc,bc2:gbc2,tm:tm,misclass_mtr:0.}
}

impl GorillaJudge {


    pub fn process_next(&mut self,refactor:bool) {
        self.lbs += self.load_next_batch();
        if self.lbs > 0 {
            self.gorilla_on_batch();
            if self.is_tailn {
                self.bc.process_batch_(true);
                self.bc2.process_batch_(true);
            }
        }

        if refactor {
            self.refactor();
            self.lbs = 0;
        }
    }

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

    pub fn gorilla_on_batch(&mut self) {
        let l = self.data_load.len();

        for i in 0..l {
            self.process_gorilla_at_index(i);
        }
    }

    pub fn process_gorilla_at_index(&mut self, i: usize) {// -> (Option<f32>,Option<skewf32::SkewF32>,Option<f32>,Option<Array1<f32>>,f32) {
        let mut gi = self.gorilla_at_index(i);
        self.add_sample_to_data(i,&mut gi);
    }

    pub fn add_sample_to_data(&mut self,i:usize,gi: &mut gorillains::GorillaIns) {
        
        let (x1,x2) = (*gi).improve_approach__labels(self.is_tailn);

        // case: add to batch corrector
        if self.is_tailn {
            self.misclass_mtr += self.misclass_of_gorillains(gi);
            self.add_sample_to_batch_corrector(i,(*gi).interval_ordering.clone().unwrap());
            return;
        }

        // case: add to Tail1Mem
        self.tm.vr_output1.push((*gi).app_out1.clone().unwrap());
        self.tm.tail1_skew.push(x1.clone().unwrap());
        self.tm.skew_mtr += x1.clone().unwrap();

        let u:usize = basic_binary_function((*gi).app_out1.clone().unwrap());        
        let mut y = self.label_load1.as_ref().unwrap()[i].clone() as usize;
        let l:f32 = if u != y {1.} else {0.};
        self.tm.misclass_mtr += l; 
    }
    

    pub fn add_sample_to_batch_corrector(&mut self,i:usize,ordering:Vec<usize>) -> f32 {
        let mut x = self.data_load[i].clone();
        let mut y:Array1<usize> = self.label_loadn.as_ref().unwrap()[i].clone().into_iter().map(|x| x as usize).collect();
        let mut x2:Array1<f32> = x.clone();
        if ordering == vec![0,1] {
            // get the outputn
            let (_,q) = self.bc.vr.apply(x.clone(),1);
            x2 = q.unwrap();
            let (c,_) = skewcorrctr::correction_for_bfgrule_approach_tailn__labels(ordering,x2.clone(),y.clone());
            let mut sk = vreducer::sample_vred_addit_skew(c.clone(),self.k);
            self.bc.refn1.push(x2.clone());
            self.bc.b.push(sk.clone());


            return sk.skew_size();
        } 
        // get the outputn
        let (_,q) = self.bc2.vr.apply(x.clone(),1);
        x2 = q.unwrap();
        let (c,_) = skewcorrctr::correction_for_bfgrule_approach_tailn__labels(ordering,x2.clone(),y.clone());
        let mut sk = vreducer::sample_vred_addit_skew(c.clone(),self.k);
        self.bc2.refn1.push(x2.clone());
        self.bc2.b.push(sk.clone());
        sk.skew_size()
    }

    /// # description
    /// Constructs a <gorillains::GorillaIns> at index `i` and processes it by tail-n or tail-1 mode.
    pub fn gorilla_at_index(&mut self,i:usize) -> gorillains::GorillaIns {
        let mut x: Array1<f32> = self.data_load[i].clone();
        let (mut y1,mut yn): (Option<usize>,Option<Array1<usize>>) = (None,None);

        if !self.is_tailn {

            if !self.label_load1.is_none() {
                y1 = Some(self.label_load1.as_ref().unwrap()[i].clone() as usize);
            }
        } else {
            if !self.label_loadn.is_none() {
                yn = Some(self.label_loadn.as_ref().unwrap()[i].clone().into_iter().map(|x| x as usize).collect());
            }
        }

        let mut gi = gorillains::build_GorillaIns(x,self.k,2,self.base_vr.clone(), yn,y1,
            if !self.is_tailn {0} else {1},2); //self.reducer_szt);

        if self.is_tailn { gi.brute_process_tailn();} else {gi.process_tail1();};
        gi
    }

    pub fn misclass_of_gorillains(&mut self,g: &mut gorillains::GorillaIns) -> f32 {
        // case: no y-data, no misclass
        if self.bry.is_none() {
            return 0.;
        }

        // case: y-data, calculate misclass from FSelect score
        let l:f32  = (*g).soln.as_ref().unwrap().data.len() as f32;
        (*g).soln.as_ref().unwrap().score.unwrap() / l
    }

    pub fn refactor(&mut self) {

        if !self.is_tailn {
            self.refactor_batch_tail1();
            return;
        }

        self.bc.refactor_();
        self.bc2.refactor_();
    }


    /// # description
    /// uses min,max,mean as points of interest. Refactorization by tail-1 mode
    /// only considers adders (no multers).
    pub fn refactor_batch_tail1(&mut self) {
        let (_,_,x3) = self.tm.refactor_batch_tail1_();
        // case: no adder
        if x3 == 0. {
            return;
        }

        self.base_vr.add_tail1_skew(x3);
    }
}