use crate::metrice::gorillains;
use crate::metrice::vreducer;
use crate::metrice::vcsv;
use crate::metrice::btchcorrctr;

use crate::enci::skew;
use crate::enci::skewf32;
use crate::enci::mat2sort;

use ndarray::{Array1,arr1};

use std::sync::{Arc, Mutex, Once};

/*
struct streams data from files X and Y (option for labelled)

CAUTION: not tested for large files.
*/
pub struct GorillaJudge {
    filepath:String,
    filepath2:Option<(String,bool)>, // filepath,true if label load n

    // stream of data
    pub data_load: Option<Vec<Array1<f32>>>,
    pub label_loadn:Option<Vec<Array1<f32>>>,
    pub label_load1:Option<Array1<f32>>,

    // base VReducer
    pub base_vr: vreducer::VRed,
    vr_outputn: Option<Vec<Array1<f32>>>,
    vr_output1: Option<Vec<f32>>,

    k:usize,
    reducer_szt:usize,

    tailn_skew:Vec<skewf32::SkewF32>,
    tail1_skew:Vec<f32>,
    ////tail1_skew_storage_index:usize,

    // score metrics of Gorilla Judge instance
        // metrics used for brute force skew
    pub skew_score:Option<f32>,
    pub skew_score0:Option<f32>,
    pub gi_score:Option<f32>
}

pub fn build_GorillaJudge(fp:String,fp2:Option<(String,bool)>,
    base_vr: vreducer::VRed,k:usize,rs: usize) -> GorillaJudge {

    GorillaJudge{filepath:fp,filepath2: fp2,data_load:None,label_loadn:None,
            label_load1:None,base_vr:base_vr, vr_outputn: Some(Vec::new()),vr_output1:Some(Vec::new()),
            k:k,reducer_szt:rs,tailn_skew:Vec::new(),tail1_skew:Vec::new(),skew_score:None,
            skew_score0:None,gi_score:None}
}

/*
implement factorization for batch correction
*/
impl GorillaJudge {

    pub fn new_batch(&mut self) {

    }

    pub fn load_data(&mut self) {
        self.data_load = Some(vcsv::csv_to_arr1_seq(&self.filepath).unwrap());

        if self.filepath2.is_none() {
            return;
        }

        let (h1,h2) = self.filepath2.clone().unwrap();
        if h2 {
            self.label_loadn = Some(vcsv::csv_to_arr1_seq(&h1).unwrap());
            assert_eq!(self.data_load.as_ref().unwrap().len(),self.label_loadn.as_ref().unwrap().len());
        } else {
            self.label_load1 = Some(vcsv::csv_to_arr1(&h1).unwrap());
            assert_eq!(self.data_load.as_ref().unwrap().len(),self.label_load1.as_ref().unwrap().len());
        }
    }

    pub fn vred_on_data(&mut self) -> (Option<Vec<f32>>,Option<Vec<Array1<f32>>>) {
        let (mut x1,mut x2): (Vec<f32>,Vec<Array1<f32>>) = (Vec::new(),Vec::new());

        let l = self.data_load.as_ref().unwrap().len();
        let t:usize = if self.filepath2.as_ref().unwrap().1 {1} else {0};
        for i in 0..l {
            let x = self.data_load.as_ref().unwrap()[i].clone();
            let (y1,y2) = self.base_vr.apply(x,t);
            if !y1.is_none() {
                x1.push(y1.unwrap());
            } else {
                x2.push(y2.unwrap());
            }
        }

        if x1.len() != 0 {
            return (Some(x1),None);
        }

        (None,Some(x2))
    }

    /*
    */
    pub fn make_GorillaIns(&mut self,single:Option<(Array1<f32>,usize)>,multi:Option<(Array1<f32>,Array1<usize>)>) -> gorillains::GorillaIns {
        assert!(single.is_none() || multi.is_none());

        if !single.is_none() {
            return gorillains::build_GorillaIns(single.as_ref().unwrap().0.clone(),self.k,
                self.base_vr.clone(),None,Some(single.as_ref().unwrap().1.clone()),0,self.reducer_szt);
        }

        gorillains::build_GorillaIns(multi.as_ref().unwrap().0.clone(),self.k,
            self.base_vr.clone(),Some(multi.as_ref().unwrap().1.clone()),None,1,self.reducer_szt)
    }

    /*
    return:
    - f32 skew | SkewF32 & gi score
    */
    pub fn gorilla_at_index(&mut self,i:usize) -> (Option<f32>,Option<skewf32::SkewF32>,f32) {
        let (mut x1,mut x2): (Option<(Array1<f32>,usize)>,Option<(Array1<f32>,Array1<usize>)>) = (None,None);
        let mut s1 = self.data_load.as_ref().unwrap()[i].clone();
        if self.filepath2.as_ref().unwrap().1 {
            let mut s2:Array1<usize> = self.label_loadn.as_ref().unwrap()[i].clone().into_iter().map(|x| x as usize).collect();
            x2 = Some((s1,s2));
        } else {
            let mut s3 = self.label_load1.as_ref().unwrap()[i].clone();
            x1 = Some((s1,s3 as usize));
        }

        let mut gi = self.make_GorillaIns(x1,x2);
        let mut giscore: f32 = 0.;
        if !gi.man_sol.is_none() {
            giscore = gi.man_sol.as_ref().unwrap().score as f32;
        }

        if self.filepath2.as_ref().unwrap().1 { gi.brute_process_tailn();} else {gi.process_tail1();};

        let (x1,x2) = gi.improve_approach__labels(self.filepath2.as_ref().unwrap().1);
        if !self.filepath2.as_ref().unwrap().1 {
            return (x1,None,giscore);
        }

        (None,Some(vreducer::sample_vred_adder_skew(x2.unwrap(),self.k)),giscore)
    }

    /*
    Apply VRed on data load to collect
    VRed output and improvement.

    outputs (f32 correction vec, skew f32 correction vec)
    */
    pub fn gorilla_apply_vred(&mut self) ->
        (Option<Vec<f32>>,Option<Vec<skewf32::SkewF32>>) {
        let l = self.data_load.as_ref().unwrap().len();
        let (mut x1,mut x2) : (Vec<f32>,Vec<skewf32::SkewF32>) = (Vec::new(),Vec::new());
        self.gi_score = Some(0.);

        for i in 0..l {
            let (y1,y2,y3) = self.gorilla_at_index(i);

            if !y1.is_none() {
                x1.push(y1.unwrap());
            }

            if !y2.is_none() {
                x2.push(y2.unwrap());
            }

            self.gi_score = Some(self.gi_score.as_ref().unwrap() + y3);
        }

        if x1.len() == 0 {(None,Some(x2))} else {(Some(x1),None)}
    }

    //// TODO: make class variable for GorillaJudge
    pub fn refactor_greedy(&mut self) -> bool {
        // tail-n
        if self.filepath2.as_ref().unwrap().1 {
            let (x1,x2,x3) = self.refactor_batch_tailn();

            if x3.is_none() {
                return false;
            }
            self.skew_score0 = Some(x1);
            self.skew_score = Some(x2);
            self.base_vr.mod_tailn_(x3.unwrap());
            return true;
        }
        false
        /*
        let (x1,x2,x3) = self.refactor_batch_tail1();

        if x3 == 0. {
            return false;
        }
        self.skew_score0 = Some(x1);
        self.skew_score = Some(x2);

        //// TODO here
        println!("SKEW");

        true
        */
    }

    /*
    return:
    - current skew score, skew score after update, skew factor
    */
    pub fn refactor_batch_tailn(&mut self) -> (f32,f32,Option<skewf32::SkewF32>) {
        let (_,r) = self.vred_on_data();
        let (_,x) = self.gorilla_apply_vred();
        self.tailn_skew = x.clone().unwrap();
        let ps:f32 = self.tailn_skew.clone().into_iter().map(|mut x| x.skew_size()).into_iter().sum::<f32>();

        // get the previous cost (tail1n_skew)
        let mut gbc = btchcorrctr::empty_GBatchCorrector(self.k);
        gbc.load_next_batch(x.unwrap(),r.unwrap());
        gbc.process_batch_(true);

        // make the refactor
        let (x1,x2,x3) = gbc.best_refactor.clone();
        if x1.is_none() {
            return (0.,0.,None);
        }

        if *(x1.as_ref().unwrap()) == 0 {
            return (0.,0.,None);
        }

        if x3 {
            let sk = skew::build_skew(Some(x1.clone().unwrap()),None,None,None,vec![0],None);
            return (ps,x2.unwrap(),Some(skewf32::SkewF32{sk:sk,s:self.k}));
        }

        let sk = skew::build_skew(None,Some(x1.clone().unwrap()),None,None,vec![1],None);
        (ps,x2.unwrap(),Some(skewf32::SkewF32{sk:sk,s:self.k}))
    }

    /*
    uses min,max,mean as points of interest

    return:
    - current score, score after adder, adder
    */
    pub fn refactor_batch_tail1(&mut self) -> (f32,f32,f32) {
        let (r,_) = self.vred_on_data();
        let (x,_) = self.gorilla_apply_vred();
        self.tail1_skew = x.clone().unwrap();

        // check
        let q = arr1(&self.tail1_skew);
        let mn_ = q.mean();

        if mn_.is_none() {
            return (0.,0.,0.);
        }
        let mn = mn_.unwrap();

        let minu = self.tail1_skew.iter().fold(f32::MAX, |min, &val| if val < min{ val } else{ min });
        let maxu = self.tail1_skew.iter().fold(f32::MIN, |max, &val| if val > max { val } else{ max });

        let x1 = mat2sort::abs_sum_arr1_f32(q.clone());
        let x2 = mat2sort::abs_sum_arr1_f32(q.clone() - mn);
        let x3 = mat2sort::abs_sum_arr1_f32(q.clone() - minu);
        let x4 = mat2sort::abs_sum_arr1_f32(q.clone() - maxu);

        let rs = vec![x1,x2,x3,x4];
        let rsm = rs.iter().fold(x1, |min, &val| if val < min{ val } else{ min });

        if (x1 - rsm).abs() < 0.0001 {
            return (x1,x1,0.);
        }

        (x1,rsm,0.)
    }

    pub fn cohesion(&mut self) {

    }

}
