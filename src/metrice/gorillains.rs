use crate::metrice::brp;
use crate::metrice::vreducer;

/*
Gorilla instructor GorillaIns is a "normal"-detection algorithm
that is given a sequence S of i32, and determines a mapping
                f: s in S --> 0|1,
based on user arg. (vector of boolean values denoting normal).

GorillaIns can proceed by computation by one of the following:
- pre-labelled data (normal values) for sequence S using data struct RangePartitionGF2
- non-labelled data, hypothesis computed by ArbitraryRangePartition
*/

///////////////////////////////////

pub struct GorillaIns {
    S: Vec<f32>,
    approach: vreducer::VRed,
    wanted_normal:Option<Vec<usize>>,
    rpgf2_sol: Option<brp::RangePartitionGF2>,
    // two recognition modes:
    /*
    (1) f32 for sequence
    (2) f32 for each value of sequence

    recog_mode_seq := true -> (1)
    */
    recog_mode_seq:bool
}

impl GorillaIns {

    pub fn process_one() {

    }

}


// build_vred

/*
pub struct VRed {
    s: Vec<fn(Array1<i32>) -> Array1<i32>>,
    tail: fn(Array1<i32>) -> f32
    */
