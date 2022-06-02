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
    wanted_normal:Option<Array1<usize>>,

    /// two approaches to getting soln for normal: manual | auto
    man_sol: Option<brp::RangePartitionGF2>,
    auto_sol: Option<arp::ArbitraryRangePartition>,

    // two recognition modes:
    /*
    (1) f32 for sequence
    (2) f32 for each value of sequence

    recog_mode_seq := true -> (1)
    */
    recog_mode_seq:bool,
    szt:usize,
    soln:Option<fs::FSelect>
}

/*
*/
pub fn build_GorillaIns(sequence:Array1<f32>,approach:vreducer::VRed,wanted_normal:Option<Array1<usize>>,
    recog_mode_seq:bool,szt:usize) -> GorillaIns {
    GorillaIns{sequence:sequence,approach:approach,wanted_normal:wanted_normal,
        man_sol:None,auto_sol:None,recog_mode_seq:recog_mode_seq,szt:szt,soln:None}
}

impl GorillaIns {

    /*
    uses brute-force approach of *RangePartition* to process entire var<sequence>
    */
    pub fn brute_process_one(&mut self) {
        // case: auto-labelling required
        if self.wanted_normal.is_none() {
            let mut arp1 = arp::build_ArbitraryRangePartition(self.sequence.clone(),self.szt);
            arp1.brute_force_search__decision();
            self.soln = arp1.fselect.clone();
            return;
        }

        let mut rpgf2 = brp::build_range_partition_gf2(self.sequence.clone(),self.wanted_normal.clone().unwrap(),
            self.szt,"basic".to_string());
        rpgf2.brute_force_search__decision();
        self.soln = Some(rpgf2.fselect);
    }
}
