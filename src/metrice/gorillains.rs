use crate::metrice::brp;
use crate::metrice::arp;
use crate::metrice::vreducer;
use crate::setti::fs;
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
    pub soln:Option<fs::FSelect>
}

/*
*/
pub fn build_GorillaIns(sequence:Array1<f32>,approach:vreducer::VRed,wanted_normaln:Option<Array1<usize>>,
    wanted_normal1:Option<usize>,tail_mode:usize,szt:usize) -> GorillaIns {

    GorillaIns{sequence:sequence,approach:approach,wanted_normaln:wanted_normaln,
    wanted_normal1:wanted_normal1,man_sol:None,auto_sol:None,
    tail_mode:tail_mode,szt:szt,soln:None}
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
            self.soln = arp1.fselect.clone();
            return;
        }

        let mut rpgf2 = brp::build_range_partition_gf2(x1.clone().unwrap(),self.wanted_normaln.clone().unwrap(),
            self.szt,"basic".to_string());
        rpgf2.brute_force_search__decision();
        self.soln = Some(rpgf2.fselect);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__GorillaIns__brute_process_tailn() {
        // case 1:
        let q:Array1<f32> = arr1(&[14.,18.,81131222.,75121.]);
        let fx: Vec<fn(Array1<f32>) -> Array1<f32>> = Vec::new();
        let mut r: vreducer::VRed = vreducer::build_VRed(fx,None,None);
        r.mod_tailn(vreducer::std_euclids_reducer);
        let normal:Array1<usize> = arr1(&[1,1,0,0]);
        let mut gi = build_GorillaIns(q,r,Some(normal),None,1,3);
        gi.brute_process_tailn();
        assert!(gi.soln.unwrap().score.unwrap() == 2.);


    }

}
