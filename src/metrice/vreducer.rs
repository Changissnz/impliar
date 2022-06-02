/*
implementation of vector reducer: a sequence of functions (arr1->arr1),and the last is
(arr1->f32).


*/
use ndarray::Array1;
use crate::metrice::gorillasf;

pub struct VRed {
    s: Vec<fn(Array1<f32>) -> Array1<f32>>,
    tail1: Option<fn(Array1<f32>) -> f32>,
    tailn: Option<fn(Array1<f32>) -> Array1<f32>>,
}

pub fn build_VRed(s:Vec<fn(Array1<f32>) -> Array1<f32>>,
    tail1:Option<fn(Array1<f32>) -> f32>, tailn: Option<fn(Array1<f32>) -> Array1<f32>>) -> VRed {
    VRed{s:s,tail1:tail1,tailn:tailn}
}

// for Euclid's, output of s' is boolean vec for normal
impl VRed {

    pub fn apply(&mut self,a:Array1<f32>,tail_type:usize) -> (Option<f32>,Option<Array1<f32>>) {
        let mut sol = a.clone();

        for s_ in self.s.clone().into_iter() {
            sol = s_(sol);
        }

        if tail_type == 0 {
            return (Some((self.tail1.unwrap())(sol)),None);
        }

        (None,Some( (self.tailn.unwrap())(sol)))
    }

    pub fn add_one_body(&mut self,a:fn(Array1<f32>) -> Array1<f32>) {
        self.s.push(a);
    }

    pub fn mod_tailn(&mut self,a:fn(Array1<f32>) -> Array1<f32>) {
        self.s.push(a);
    }

    pub fn mod_tail1(&mut self,nt:fn(Array1<f32>) -> f32) {
        self.tail1 = Some(nt);
    }
}

/*
average for gorilla euclid additive vector A and coefficient vector C
*/
pub fn std_euclids_reducer(s:Array1<f32>) -> Array1<f32> {
    let s1: Array1<i32> = s.into_iter().map(|x| x as i32).collect();
    let (mut g1,mut g2) = gorillasf::gorilla_touch_arr1_basic(s1,0.5);
    (g1 + g2) / 2.0
}
