//! GorillaJudge hypothesis container
use ndarray::{Array1,arr1};
use std::fmt;

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
    hyp_n1: Option<(Array1<usize>,f32)>,
    hyp_n2: Option<(Array1<usize>,f32)>,
    hyp_1: Option<usize>,
    hyp_n: Option<Array1<usize>>
}

pub fn build_GorillaPred(hyp_n1:Option<(Array1<usize>,f32)>, hyp_n2:Option<(Array1<usize>,f32)>,
    hyp_1:Option<usize>,hyp_n:Option<Array1<usize>>) -> GorillaPred {

    let x:bool = (!hyp_n1.is_none() && !hyp_n2.is_none() && hyp_1.is_none() && hyp_n.is_none()) ||
                (hyp_1.is_none() && !hyp_n.is_none()) ||
                (!hyp_1.is_none() && hyp_n.is_none());

    assert!(x, "invalid arguments");
    GorillaPred{hyp_n1:hyp_n1,hyp_n2:hyp_n2,hyp_1:hyp_1,hyp_n:hyp_n}
}

impl fmt::Display for GorillaPred {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "* gorilla prediction\n".to_string();
        if !self.hyp_n1.is_none() {
            let q = &format!("  * corrector #1: {:?}\n * corrector #2: {:?}\n",self.hyp_n1.as_ref().unwrap(),self.hyp_n2.as_ref().unwrap());
            s.push_str(&q);
        } else if !self.hyp_1.is_none() {
            let q = &format!("  * 1-hyp: {:?}\n",self.hyp_1.as_ref().unwrap());
            s.push_str(&q);
        } else {
            let q = &format!("  * n-hyp: {:?}\n",self.hyp_n.as_ref().unwrap());
            s.push_str(&q);
        }
        write!(f, "{}", s)
    }
}

impl GorillaPred {

    pub fn best(&mut self) -> (Option<usize>,Option<Array1<usize>>) {
        if !self.hyp_n1.is_none() {
            if self.hyp_n1.as_ref().unwrap().1 < self.hyp_n2.as_ref().unwrap().1 {
                return (None,Some(self.hyp_n1.clone().unwrap().0));
            }  else {
                return (None,Some(self.hyp_n2.clone().unwrap().0));
            }
        }

        if !self.hyp_n.is_none() {
            return (None,self.hyp_n.clone());
        }

        (self.hyp_1.clone(),None) 



    }


}