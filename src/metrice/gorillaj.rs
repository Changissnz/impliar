use crate::metrice::gorillains;
use crate::metrice::vreducer;
use crate::metrice::vcsv;
use crate::metrice::gorillajh;
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
#[derive(Clone)]
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
        let rs = vec![(x1,0.),(x2,mn),(x3,minu),(x4,maxu)];

        // get (index, score) w/ lowest score
        let rsm = rs.clone().into_iter().fold((x1,0.), |min, val| if val.0 < min.0 { val } else{ min });

        // update tail1_skew and vr_output1
        let q = arr1(&self.tail1_skew);
        self.tail1_skew = (q - rsm.1).into_iter().collect();
        self.vr_output1 = (arr1(&self.vr_output1) + rsm.1).into_iter().collect();
        self.skew_mtr = rsm.0.clone();
        (x1,rsm.0,rsm.1)       
    }

    pub fn closest_value_index(&mut self,f:f32) -> usize {
        assert!(self.vr_output1.len() > 0);
        let x = self.vr_output1.iter().enumerate().fold((0,&f32::MAX),|x1,x2| if (*x1.1 - f).abs() < (*x2.1 - f).abs() {x1} else {x2});
        x.0
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
    /// tail-n case, batch corrector for refactoring skews, interval ordering \[0.25,0.75\]
    pub bc: btchcorrctr::GBatchCorrector,
    /// tail-n case, batch corrector for refactoring skews, interval ordering \[0.75,0.25\]
    pub bc2: btchcorrctr::GBatchCorrector,    
    /// tail-1 case corrector
    pub tm: Tail1Mem,
    /// memory container for label-less
    pub gh: gorillajh::GorillaHyp,
    /// misclassification score
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
        label_loadn:None,label_load1:None,base_vr:base_vr, k:k,bs:bs,lbs: 0,
        bc:gbc,bc2:gbc2,tm:tm,gh:gorillajh::empty_GorillaHyp(),misclass_mtr:0.}
}

impl GorillaJudge {


    pub fn process_next(&mut self,refactor:bool) {
        self.lbs += self.load_next_batch();
        if self.lbs > 0 {
            self.gorilla_on_batch();
            if self.is_tailn {
                self.bc.process_batch_(false);
                self.bc2.process_batch_(false);
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
        // case: no labels
        if self.bry.is_none() {
            self.add_sample_to_ghmem(gi);
            return;
        }

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
        self.tm.skew_mtr += x1.clone().unwrap().abs();

        let u:usize = basic_binary_function((*gi).app_out1.clone().unwrap());        
        let mut y = self.label_load1.as_ref().unwrap()[i].clone() as usize;
        let l:f32 = if u != y {1.} else {0.};
        self.tm.misclass_mtr += l; 
    }

    pub fn add_sample_to_ghmem(&mut self,gi:&mut gorillains::GorillaIns) {
        let (x1,x2) = (*gi).predict_sequence((*gi).sequence.clone());
        self.gh.add_sample(x2,x1);
    }
    
    pub fn add_sample_to_batch_corrector(&mut self,i:usize,ordering:Vec<usize>) -> f32 {
        let mut x = self.data_load[i].clone();
        let mut y:Array1<usize> = self.label_loadn.as_ref().unwrap()[i].clone().into_iter().map(|x| x as usize).collect();
        let mut x2:Array1<f32> = x.clone();
        if ordering == vec![0,1] {
            ////println!("adding sample {} to bc#1",i);
            // get the outputn
            let (_,q) = self.bc.vr.apply(x.clone(),1);
            x2 = q.unwrap();
            let (c,_) = skewcorrctr::correction_for_bfgrule_approach_tailn__labels(ordering,x2.clone(),y.clone());
            let mut sk = vreducer::sample_vred_addit_skew(c.clone(),self.k);
            self.bc.refn1.push(x2.clone());
            self.bc.b.push(sk.clone());
            return sk.skew_size();
        } 

        ////println!("adding sample {} to bc#2",i);
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

        if self.bry.is_none() {
            return;
        }

        if !self.is_tailn {
            self.refactor_batch_tail1();
            return;
        }
        println!("** corrector refactor");
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

    /// # description
    /// 
    pub fn predict_sequence(&mut self,x:Array1<f32>) -> gorillajh::GorillaPred {
        // case: unlabelled
        if self.bry.is_none() {
            let mut gi = gorillains::build_GorillaIns(x.clone(),self.k,2,self.base_vr.clone(), None,None,
            if !self.is_tailn {0} else {1},2);
            if self.is_tailn { gi.brute_process_tailn();} else {gi.process_tail1();};
            let (x1,x2) = gi.predict_sequence(x.clone());
            return gorillajh::build_GorillaPred(None,None,x1,x2);
        }

        if self.is_tailn {
            let (x11,x12) = self.predictn_(x.clone());
            return gorillajh::build_GorillaPred(Some(x11),Some(x12),None,None)
        }

        let x13 = self.predict1_(x.clone());
        return gorillajh::build_GorillaPred(None,None,Some(x13),None)
    }

    pub fn predictn_(&mut self,x:Array1<f32>) -> ((Array1<usize>,f32),(Array1<usize>,f32)) {
        // predict by corrector #1
            //let (_,q) = self.bc.vr.apply(x.clone(),1);
            //let mut q_ = q.unwrap();
        let mut q_ = self.bc.map_sample(x.clone());
    
        let li = skewcorrctr::label_intervals_by_ordering(vec![0,1]);
        let p1:Array1<usize> = q_.clone().into_iter().map(|y| gorillains::label_of_f32(y,li.clone())).collect();
        let w1 = skewcorrctr::wanted_normaln_to_interval_values(p1.clone(),vec![0,1]);
        let s1:Array1<f32> = (w1.clone() -q_.clone()).into_iter().map(|y| y.abs()).collect();

        // predict by corrector #2
            //let (_,q2) = self.bc2.vr.apply(x.clone(),1);
            //let mut q2_ = q2.unwrap();
        let mut q2_ = self.bc2.map_sample(x.clone());    
        let li2 = skewcorrctr::label_intervals_by_ordering(vec![1,0]);
        let p2:Array1<usize> = q2_.clone().into_iter().map(|y| gorillains::label_of_f32(y,li2.clone())).collect();
        let w2 = skewcorrctr::wanted_normaln_to_interval_values(p2.clone(),vec![1,0]);
        let s2:Array1<f32> = (w2.clone() -q2_.clone()).into_iter().map(|y| y.abs()).collect();

        ((p1,s1.sum()),(p2,s2.sum()))
    }

    pub fn predict1_(&mut self,x:Array1<f32>) -> usize {
        let(q,_) = self.base_vr.apply(x.clone(),0);

        let u = self.tm.closest_value_index(q.clone().unwrap());

        basic_binary_function(self.tm.tail1_skew[u].clone() + q.unwrap())
    }

    /// # description
    pub fn skew_meter(&mut self) {

        if self.bry.is_none() {
            println!("no skew metric for unlabelled data");
            return;
        }

        if self.is_tailn {
            println!("corrector #1");
            self.bc.info();
            println!("corrector #2");
            self.bc2.info();
        } else {
            println!("corrector: {}",self.tm.skew_mtr);
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
    /// Checks that refactors result in lower skew values
    #[test]
    pub fn test__GorillaJudge__process_next___case_1() {
        let vr = vreducer::sample_vred_euclids_reducer();
        let mut gj = build_GorillaJudge("src/data/f3_x.csv".to_string(),Some("src/data/f3_y2.csv".to_string()),
            true,vr.clone(),2,20); 
    
        let mut gj2 = build_GorillaJudge("src/data/f3_x.csv".to_string(),Some("src/data/f3_y2.csv".to_string()),
            true,vr,2,20); 
    
        gj.process_next(false);
        let sm:f32 = gj.bc.skew_mtr + gj.bc2.skew_mtr;
    
        gj.refactor();
        let sm2:f32 = gj.bc.skew_mtr + gj.bc2.skew_mtr;
        assert!(sm2 < sm, "refactor results in >= skew metric");
    }

    #[test]
    pub fn test__GorillaJudge__process_next___case_2() {

        let vr = vreducer::sample_vred_euclids_reducer_tail1();
        let mut gj = build_GorillaJudge("src/data/f3_x.csv".to_string(),Some("src/data/f3_y.csv".to_string()),
            false,vr.clone(),2,20); 
        gj.process_next(false);
        let sm:f32 = gj.tm.skew_mtr.clone(); 
        gj.refactor();
        let sm2:f32 = gj.tm.skew_mtr.clone();     
        assert!(sm2 < sm, "refactor results in >= skew metric");
    }

    /// # description
    /// tests accuracy of predicting samples that a GorillaJudge
    /// instance has already trained on.
    /// 
    /// * test description:
    /// - tail-n, labelled, 11 samples, tests for accuracy at least 7. 
    #[test]
    pub fn test__GorillaJudge__predict_sequence__case_1() {

        let vr = vreducer::sample_vred_euclids_reducer();
        let mut gj = build_GorillaJudge("src/data/f3_x.csv".to_string(),Some("src/data/f3_y2.csv".to_string()),
            true,vr.clone(),2,20); 
        gj.process_next(true);
        
        let x = vcsv::csv_to_arr1_seq("src/data/f3_x.csv").unwrap();
        let y = vcsv::csv_to_arr1_seq("src/data/f3_y2.csv").unwrap();
    
        let l = x.len();
        let mut c = 0;
        for i in 0..l {
            let mut p = gj.predict_sequence(x[i].clone());
            let y2:Array1<usize> = y[i].clone().into_iter().map(|y_| y_ as usize).collect();    
            if p.best().1.unwrap() == y2 {
                c += 1;
            }
        }
        
        assert!(c >= 7); 
    }

 /// # description
    /// tests accuracy of predicting samples that a GorillaJudge
    /// instance has already trained on.
    /// 
    /// * test description:
    /// - tail-1, labelled, 11 samples, tests for accuracy of all. 
    #[test]
    pub fn test__GorillaJudge__predict_sequence__case_2() {

        let vr = vreducer::sample_vred_euclids_reducer_tail1();
        let mut gj = build_GorillaJudge("src/data/f3_x.csv".to_string(),Some("src/data/f3_y5.csv".to_string()),
            false,vr.clone(),2,20); 
        gj.process_next(true);
    
        let x = vcsv::csv_to_arr1_seq("src/data/f3_x.csv").unwrap();
        let y = vcsv::csv_to_arr1("src/data/f3_y5.csv").unwrap();
    
        let l = x.len();
        let mut c = 0;
        for i in 0..l {
            let mut p = gj.predict_sequence(x[i].clone());
            
            let y2:usize = y[i].clone() as usize;    
            if p.best().0.unwrap() == y2 {
                c += 1;
            }
            
        }
        assert_eq!(c,11);
    }
}