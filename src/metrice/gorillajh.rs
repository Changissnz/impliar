//! GorillaJudge hypothesis container
use ndarray::{Array1,arr1};

pub struct GorillaHyp {
    pub yn:Vec<Array1<usize>>,
    pub y1:Vec<usize>
}

pub fn empty_GorillaHyp() -> GorillaHyp {
    GorillaHyp{yn: Vec::new(),y1: Vec::new()}
}

impl GorillaHyp {

    pub fn add_sample(&mut self,yn_:Option<Array1<usize>>,y1_:Option<usize>) {
        if yn_.is_none() {
            self.y1.push(y1_.unwrap());
        } else if y1_.is_none() {
            self.yn.push(yn_.unwrap());
        } else {
            assert!(false, "cannot have a label-array and a label");
        }

    }

}