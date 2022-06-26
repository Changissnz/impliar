use crate::metrice::gorillains;
use crate::metrice::vreducer;
use crate::metrice::vcsv;
use ndarray::{Array1,arr1};

/*
struct streams data from files X and Y (option for labelled)

CAUTION: not tested for large files.
*/
pub struct GorillaJudge {
    filepath:String,
    filepath2:Option<(String,bool)>, // filepath,true if label load n

    // stream of data
    data_load: Option<Vec<Array1<f32>>>,
    label_loadn:Option<Vec<Array1<f32>>>,
    label_load1:Option<Array1<f32>>,

    // base VReducer
    base_vr: vreducer::VRed,
    vr_outputn: Option<Vec<Array1<f32>>>,
    vr_output1: Option<Array1<f32>>,

    k:usize,
    reducer_szt:usize
}

pub fn build_GorillaJudge(fp:String,fp2:Option<(String,bool)>,base_vr: vreducer::VRed,
    rs: usize) -> GorillaJudge {
    GorillaJudge{filepath:fp,filepath2: fp2,data_load:None,label_loadn:None,
            label_load1:None,base_vr:base_vr,reducer_szt:rs}
}

/*
implement factorization for batch correction
*/
impl GorillaJudge {

    pub fn load_data(&mut self) {
        self.data_load = Some(vcsv::csv_to_arr1_seq(self.filepath));

        if self.filepath2.is_none() {
            return;
        }

        let (h1,h2) = self.filepath2.clone().unwrap();
        if h2 {
            self.label_loadn = Some(vcsv::csv_to_arr1_seq(h1));
            assert_eq!(self.data_load.unwrap().len(),self.label_loadn.unwrap().len());
        } else {
            self.label_load1 = Some(vcsv::csv_to_arr1(h1));
            assert_eq!(self.data_load.unwrap().len(),self.label_load1.unwrap().len());
        }

    }

    /*
    Apply VRed on data load to collect
    VRed output and improvement.

    outputs (score(GI),improvement_vec)
    */
    pub fn gorilla_apply_vred(&mut self) -> (f32,Option<Vec<f32>>,Option<Vec<skewf32::SkewF32>) {
        let l = self.data_load.unwrap().len();
        let mut soln:Vec<Array1<f32>> = Vec::new();
        let tm:usize = if self.filepath2.unwrap().1 {1} else {0};
        let mut sc:f32 = 0.;

        let mut s1 : Vec<f32> = Vec::new();
        //let s2 : Vec<Array1<f32>> = Vec::new();
        let mut s2 : Vec<skewf32::SkewF32> = Vec::new();


        for i in 0..l {
            let s = self.data_load[i].clone();
            let wn: Option<Array1<usize>> = if self.filepath2.unwrap().1 {None} else {self.label_loadn.unwrap()[i].clone().into_iter().collect()};
            let wn1: Option<usize> = if self.filepath2.unwrap().1 {Some(self.label_load1.unwrap()[i] as usize)} else {None};

            // collect VRed results
            let mut ts = gorillains::build_GorillaIns(s,base_vr.clone(),wn,wn1,tm,self.reducer_szt);
            let (x1,x2) = ts.approach_on_sequence();

            if !self.filepath2.unwrap().1 {
                let mut l1: Vec<f32> = self.vr_output1.clone().unwrap().into_iter().collect();
                l1.push(x1.unwrap());
                self.vr_output1 = Some(l1.into_iter().collect());
            } else {
                let mut vo = self.vr_output.clone().unwrap();
                vo.push(x2.unwrap());
                self.vr_output = Some(vo);
            }

            // get cost of GorillaIns soln
            if self.filepath2.unwrap().1  {
                gi.brute_process_tailn();
                sc += gi.soln.clone().unwrap().score;
            }

            // calculate improvement
            let (x1,x2) = ts.improve_approach__labels(self.filepath2.unwrap().1);
            if !x2.is_none() {
                //s2.push(x2.unwrap());
                s2.push(vreducer::sample_vred_adder_skew(x2.unwrap()));
            } else {
                s1.push(x1.unwrap());
                sc += x1.unwrap();
            }
        }

        if self.filepath2.unwrap().1 {(sc,None,Some(s2))} else {(ax,Some(s1),None)}
    }

    pub fn refactor_batch(&mut self) {
        
    }
}
