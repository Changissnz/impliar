use crate::metrice::brp;
use crate::metrice::arp;
use crate::metrice::vreducer;
use crate::metrice::skewcorrctr;
use crate::setti::fs;
use crate::enci::skew;
use crate::enci::skewf32;
use ndarray::{arr1,Array1};

/*
Gorilla instructor GorillaIns is a "normal"-detection algorithm
that is given a sequence S of f32, and determines a mapping
                f: s in S --> {0,1}^|S| OR (0|1),
based on user arg. (vector of boolean values denoting normal).

GorillaIns can proceed by one of the following:
- pre-labelled data (normal values) for sequence S using data struct RangePartitionGF2
- non-labelled data, hypothesis computed by ArbitraryRangePartition
*/
pub struct GorillaIns {
    sequence: Array1<f32>,
    approach: vreducer::VRed,
    app_out1: Option<f32>,
    pub app_outn: Option<Array1<f32>>,

    // indices from S that are labelled "normal"
    wanted_normaln:Option<Array1<usize>>,
    wanted_normal1:Option<usize>,

    /// two approaches to getting soln for normal: manual | auto
    man_sol: Option<brp::RangePartitionGF2>,
    auto_sol: Option<arp::ArbitraryRangePartition>,

    // two recognition modes:
    /*
    (0) f32 for sequence
    (1) f32 for each value of sequence

    recog_mode_seq := true -> (0)
    */
    tail_mode:usize,
    szt:usize,
    pub soln:Option<fs::FSelect>,
    pub corr:Option<Array1<f32>>
}

/*
*/
pub fn build_GorillaIns(sequence:Array1<f32>,approach:vreducer::VRed,wanted_normaln:Option<Array1<usize>>,
    wanted_normal1:Option<usize>,tail_mode:usize,szt:usize) -> GorillaIns {
    GorillaIns{sequence:sequence,approach:approach,app_out1:None,app_outn:None,
    wanted_normaln:wanted_normaln, wanted_normal1:wanted_normal1,man_sol:None,
    auto_sol:None,tail_mode:tail_mode,szt:szt,soln:None,corr:None}
}

impl GorillaIns {

    pub fn approach_on_sequence(&mut self) -> (Option<f32>,Option<Array1<f32>>) {
        self.approach.apply(self.sequence.clone(),self.tail_mode)
    }

    pub fn brute_process_tailn(&mut self) {
        let (_,mut x1) = self.approach_on_sequence();

        if self.wanted_normaln.is_none() {
            let mut arp1 = arp::build_ArbitraryRangePartition(x1.clone().unwrap(),self.szt);
            arp1.brute_force_search__decision();
            self.auto_sol =  Some(arp1);
            return;
        }

        let mut rpgf2 = brp::build_range_partition_gf2(x1.clone().unwrap(),self.wanted_normaln.clone().unwrap(),
            self.szt,"basic".to_string());
        rpgf2.brute_force_search__decision();

        self.soln = Some(rpgf2.fselect.clone());
        self.man_sol = Some(rpgf2);
        self.app_outn = x1.clone();
    }

    /*
    improves solution by the same size threshold t. uses struct<Skew> to
    modify values.
    */
    pub fn improve_approach__labels(&mut self,is_multi:bool) -> (Option<f32>,Option<Array1<f32>>) {

        if !is_multi {
            return (None,None);
        }

        // convert fselect to bfgselect
        let bfgsr = skewcorrctr::gorilla_improve_approach_tailn__labels(self.app_outn.clone().unwrap(),self.wanted_normaln.clone().unwrap());
        let corr = skewcorrctr::correction_for_bfgrule_approach_tailn__labels(bfgsr,self.app_outn.clone().unwrap(),self.wanted_normaln.clone().unwrap());

        // get skew
        (None,Some(corr))
    }

    /*
    improves approach by VRed on tailn
    */
    pub fn improve_vred__tailn(&mut self,v:Array1<f32>) {
        self.corr = Some(v.clone());

        // tail as last body
        if !self.approach.tailn.is_none() {
            let x = self.approach.tailn.clone().unwrap();//.unwrap();
            self.approach.add_s(x);
        }

        // make skew
        let sk = vreducer::sample_vred_adder_skew(v);
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
        let mut gi = build_GorillaIns(q,vr21,Some(normal),None,0,3);

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
        let mut gi = build_GorillaIns(q,vr21,Some(normal),None,0,3);

        // before improvement
        gi.brute_process_tailn();
        assert!(gi.soln.clone().unwrap().score.clone().unwrap() <= 2.);

        // after improvement
        let (_,qrx) = gi.improve_approach__labels(true);
        gi.improve_vred__tailn(qrx.unwrap());
        gi.brute_process_tailn();

        assert_eq!(gi.app_outn,Some(arr1(&[0.75, 0.75, 0.25, 0.25001])));
        assert_eq!(Some(0.), gi.soln.clone().unwrap().score);
    }

}
