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

pub struct GorillaPred {
    pub hyp_n1: Option<(Array1<usize>,f32)>,
    pub hyp_n2: Option<(Array1<usize>,f32)>,
    pub hyp_1: Option<usize>,
    pub hyp_n: Option<Array1<usize>>
}

pub fn build_GorillaPred(hyp_n1:Option<(Array1<usize>,f32)>, hyp_n2:Option<(Array1<usize>,f32)>,
    hyp_1:Option<usize>,hyp_n:Option<Array1<usize>>) -> GorillaPred {

    let x:bool = (!hyp_n1.is_none() && !hyp_n2.is_none() && hyp_1.is_none() && hyp_n.is_none()) ||
                (hyp_1.is_none() && !hyp_n.is_none()) ||
                (!hyp_1.is_none() && hyp_n.is_none());

    assert!(x, "invalid arguments");
    GorillaPred{hyp_n1:hyp_n1,hyp_n2:hyp_n2,hyp_1:hyp_1,hyp_n:hyp_n}
}