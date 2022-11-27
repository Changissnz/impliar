//! Contains a "normal"-detection algorithm. 
use crate::metrice::brp;
use crate::metrice::arp;
use crate::metrice::vreducer;
use crate::metrice::skewcorrctr;
use crate::metrice::vcsv;
use crate::setti::fs;
use crate::enci::skew;
use crate::enci::skewf32;
#[doc(hidden)]
use ndarray::{arr1,Array1};

/// # description
/// standard reducer for tail-1
pub fn f9(x:Array1<f32>) -> f32 {
    let l = x.len() as f32;
    x.into_iter().sum::<f32>() / l
}

/// Gorilla instructor GorillaIns is a "normal"-detection algorithm that determines
/// if attribute "sequence" (arr1\<f32\>) is normal. 
/// 
/// For a sequence S, applies the approach A, a <vreducer::VRed>  on S by the following
///                A: S --> {0,1}^|S| OR (0|1),
/// based on user arg. (vector of boolean values denoting normal).
/// .....
/// GorillaIns can proceed by one of the following:
/// pre-labelled data (normal values) for sequence S using data struct RangePartitionGF2
/// non-labelled data, hypothesis computed by ArbitraryRangePartition
/// .....
/// GorillaIns can calculate a correction vector (if tail-n) or correction float (if tail-1).
pub struct GorillaIns {
    /// target of "normal"-analysis
    sequence: Array1<f32>,
    /// number of decimal places considered
    k:usize,
    /// the chained function used to "translate" sequence
    approach: vreducer::VRed,
    /// tail-1 case output 
    app_out1: Option<f32>,
    /// tail-n case output 
    pub app_outn: Option<Array1<f32>>,
    /// tail-n case wanted normal 
    wanted_normaln:Option<Array1<usize>>,
    /// tail-1 case wanted normal 
    wanted_normal1:Option<usize>,
    /// solution for manual approach (provided "wanted")
    pub man_sol: Option<brp::RangePartitionGF2>,
    /// solution for automatic approach (not provided "wanted") 
    auto_sol: Option<arp::ArbitraryRangePartition>,
    /// 0 for tai1-1 (output is f32) or 1 for tail-n (output is n-vector)
    tail_mode:usize,
    /// partition variable 
    szt:usize,
    /// soln from manual or automatic approach
    pub soln:Option<fs::FSelect>,
    /// corrector vector for tail-n mode 
    pub corr:Option<Array1<f32>>,
    /// corrector vector for tail-1 mode 
    pub corr2: Option<f32>
}

/// # caution
/// wanted normal1 intended for gf2.
pub fn build_GorillaIns(sequence:Array1<f32>,k:usize,approach:vreducer::VRed,wanted_normaln:Option<Array1<usize>>,
    wanted_normal1:Option<usize>,tail_mode:usize,szt:usize) -> GorillaIns {

    if !wanted_normal1.is_none() {
        assert!(wanted_normal1.clone().unwrap() < 2);
    }

    GorillaIns{sequence:sequence,k:k,approach:approach,app_out1:None,app_outn:None,
    wanted_normaln:wanted_normaln, wanted_normal1:wanted_normal1,man_sol:None,
    auto_sol:None,tail_mode:tail_mode,szt:szt,soln:None,corr:None,corr2:None}
}

impl GorillaIns {

    /// # description
    /// Predicts the normality of a sequence v by one of the tail
    /// approaches, singleton or tail-n (vector). 
    ///
    /// return: 
    /// If tail_mode == 0: \[0\] is solution 
    /// If tail_mode == 1: \[1\] is solution
    pub fn predict_sequence(&mut self,v: Array1<f32>) -> (Option<usize>,Option<Array1<usize>>)  {
        if self.tail_mode == 1 {
            assert!(!self.soln.is_none(), "DESTRAUUUUUUUGHT");
            let mut x1 = self.approach.apply(v.clone(),self.tail_mode).1.unwrap();
            let mut fsa = self.soln.clone().unwrap();

            // iterate through and get index and label
            let mut ls : Vec<usize> = Vec::new();
            for v_ in x1.into_iter() {
                let l = fsa.label_of_f32(v_);
                ls.push(if l.is_none() {0} else {l.unwrap()});
            }
            return (None,Some(ls.into_iter().collect()));
        }

        let mut x1 = self.approach.apply(v.clone(),self.tail_mode).0.unwrap();
        if !self.corr2.is_none() {
            x1 = x1 + self.corr2.unwrap();
        }

        if x1 < 0.5 {(Some(0),None)} else {(Some(1),None)}
    }

    /// # description
    /// runs <vred::VReducer> by `tail_mode` on `sequence` 
    /// 
    /// # return
    /// tail-1 output or tail-n output
    pub fn approach_on_sequence(&mut self) -> (Option<f32>,Option<Array1<f32>>) {
        self.approach.apply(self.sequence.clone(),self.tail_mode)
    }

    /// # description
    /// runs `approach_on_sequence` and calculates the error-score
    ///
    /// # return
    /// (label 0|1 of sample,error-cost)
    pub fn process_tail1(&mut self) -> (usize,Option<f32>) {
        assert!(!(self.tail_mode == 1));

        let x = self.approach_on_sequence().0.unwrap();
        assert!(x >= 0. && x <= 1.);

        let y:usize = if x < 0.5 {0} else {1};
        if self.wanted_normal1.is_none() {
            return (y,None);
        }

        let y2 = self.wanted_normal1.clone().unwrap();

        if y2 != y {
            self.corr2 = if y2 == 1 {Some(0.5 - x)} else {Some(0.49 - x)};
        }
        (y2,self.corr2.clone())
    }

    /// # description
    /// processes tail-n approach by brute-force approach using `arp::ArbitraryRangePartition`
    pub fn brute_process_tailn(&mut self) {
        if self.tail_mode == 0 {
            return;
        }
        let (_, x1) = self.approach_on_sequence();
        self.app_outn = x1.clone();

        if self.wanted_normaln.is_none() {
            let mut arp1 = arp::build_ArbitraryRangePartition(x1.clone().unwrap(),self.szt);
            arp1.brute_force_search__decision();
            self.soln = arp1.fselect.clone();
            self.auto_sol =  Some(arp1);

            return;
        }

        let mut rpgf2 = brp::build_range_partition_gf2(x1.clone().unwrap(),self.wanted_normaln.clone().unwrap(),
            self.szt,"basic".to_string());
        rpgf2.brute_force_search__decision();

        self.soln = Some(rpgf2.fselect.clone());
        self.man_sol = Some(rpgf2);
    }

    /// # description
    /// improves solution by the same size threshold t. uses struct<Skew> to
    /// modify values.
    pub fn improve_approach__labels(&mut self,is_multi:bool) -> (Option<f32>,Option<Array1<f32>>) {

        if !is_multi {
            // get label
            let (_,q2) = self.process_tail1();
            if q2.is_none() {
                return (Some(0.),None);
            }
            return (q2,None);
        }

        // convert fselect to bfgselect
        let bfgsr = skewcorrctr::gorilla_improve_approach_tailn__labels(self.app_outn.clone().unwrap(),self.wanted_normaln.clone().unwrap());
        let corr = skewcorrctr::correction_for_bfgrule_approach_tailn__labels(bfgsr,self.app_outn.clone().unwrap(),self.wanted_normaln.clone().unwrap());

        // get skew
        (None,Some(corr))
    }

    /// # description
    /// improves approach by making a <vreducer::VRed> adder skew out of `v` 
    /// and updating its tailn w/ it.
    pub fn improve_vred__tailn(&mut self,v:Array1<f32>) {
        self.corr = Some(v.clone());

        // tail as last body
        if !self.approach.tailn.is_none() {
            let x = self.approach.tailn.clone().unwrap();
            self.approach.add_s(x);
        }

        // make skew
        let sk = vreducer::sample_vred_adder_skew(v,self.k);
        self.approach.mod_tailn(sk);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__GorillaIns__brute_process_tailn() {
        // case 1:
        let q:Array1<f32> = arr1(&[14.,18.,81131222.,75121.]);
        let normal:Array1<usize> = arr1(&[0,1,1,0]);
        let sv1: Vec<vreducer::FCast> = vec![vreducer::FCast{f:vreducer::std_euclids_reducer}];
        let vr21 = vreducer::build_VRed(sv1,Vec::new(),vec![0],
                    0,None,None);
        let mut gi = build_GorillaIns(q,5,vr21,Some(normal),None,1,3);

        gi.brute_process_tailn();
        assert_eq!(gi.app_outn,Some(arr1(&[0.47728175, 0.75453043, 0.75, 0.86111116])));
    }

    #[test]
    pub fn test__GorillaIns__improve_vred__tailn() {
        let q:Array1<f32> = arr1(&[14.,18.,81131222.,75121.]);
        let normal:Array1<usize> = arr1(&[1,1,0,0]);
        let sv1: Vec<vreducer::FCast> = vec![vreducer::FCast{f:vreducer::std_euclids_reducer}];
        let vr21 = vreducer::build_VRed(sv1,Vec::new(),vec![0],
                    0,None,None);
        let mut gi = build_GorillaIns(q,5,vr21,Some(normal),None,1,3);

        // before improvement
        gi.brute_process_tailn();
        assert!(gi.soln.clone().unwrap().score.clone().unwrap() <= 2.);

        // after improvement
        let (_,qrx) = gi.improve_approach__labels(true);
        gi.improve_vred__tailn(qrx.unwrap());
        gi.brute_process_tailn();

        assert_eq!(gi.app_outn,Some(arr1(&[0.75, 0.75, 0.25, 0.25])));
        assert_eq!(Some(0.), gi.soln.clone().unwrap().score);
    }

    /*
    man-sol
    */
    #[test]
    pub fn test__GorillaIns__predict_sequence() {
        let x0 = vcsv::csv_to_arr1_seq("src/data/f3_x.csv").unwrap();
        let x = vcsv::csv_to_arr1("src/data/f3_y.csv").unwrap();

        // case 1
        let t0 = x0[0].clone();
        let t1 = x[0].clone() as usize;

        let sv1: Vec<vreducer::FCast> = vec![vreducer::FCast{f:vreducer::std_euclids_reducer}];

        let vr21 = vreducer::build_VRed(sv1.clone(),Vec::new(),vec![0,1],
                    0,Some(vreducer::FCastF32{f:f9,ai:0.}),None);

        let mut gi = build_GorillaIns(t0.clone(),5,vr21.clone(),None,Some(t1),0,3);

        let (u,o) = gi.process_tail1();

        for t in x0.clone().into_iter() {
            let (u2,_) = gi.predict_sequence(t.clone());
            assert_eq!(u2.unwrap(),1);
        }
    }

    /*
    auto_sol
    */
    #[test]
    pub fn test__GorillaIns__predict_sequence__2() {
        let x0 = vcsv::csv_to_arr1_seq("src/data/f2_x.csv").unwrap();

        // get the first
        let t0 = x0[0].clone();
        let sv1: Vec<vreducer::FCast> = vec![vreducer::FCast{f:vreducer::std_euclids_reducer}];
        let vr21 = vreducer::build_VRed(sv1,Vec::new(),vec![0],
                    0,None,None);

        let mut gi = build_GorillaIns(t0.clone(),5,vr21,None,None,1,5);
        gi.brute_process_tailn();
        let (_,u2) = gi.predict_sequence(x0[0].clone());
        assert_eq!(Some(arr1(&[0, 1, 2, 3, 0])),u2);
    }

    /*
    man-sol
    */
    #[test]
    pub fn test__GorillaIns__predict_sequence__3() {

        let x0 = vcsv::csv_to_arr1_seq("src/data/f3_x.csv").unwrap();
        let x = vcsv::csv_to_arr1("src/data/f3_y.csv").unwrap();

        // get the first
        let t0 = x0[1].clone();
        let t1 : usize = x[1].clone() as usize;

        let sv1: Vec<vreducer::FCast> = vec![vreducer::FCast{f:vreducer::std_euclids_reducer}];
        let vr21 = vreducer::build_VRed(sv1,Vec::new(),vec![0],
                    0,Some(vreducer::FCastF32{f:f9,ai:0.}),None);

        let mut gi = build_GorillaIns(t0.clone(),5,vr21,None,Some(t1),0,5);

        let (u,o) = gi.process_tail1();
        let (u2,_) = gi.predict_sequence(x0[4].clone());
        assert_eq!(u2,Some(1));

        let (u2,_) = gi.predict_sequence(x0[9].clone());
        assert_eq!(u2,Some(1));
    }

}
