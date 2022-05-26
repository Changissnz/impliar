/*
implementation of vector reducer: a sequence of functions (arr1->arr1),and the last is
(arr1->f32).


*/
use ndarray::Array1;

pub struct VRed {
    s: Vec<fn(Array1<i32>) -> Array1<i32>>,
    tail: fn(Array1<i32>) -> f32
}

// for Euclid's, output of s' is boolean vec for normal


impl VRed {

    pub fn apply(&mut self,a:Array1<i32>) -> f32 {
        let mut sol = a.clone();

        for s_ in self.s.clone().into_iter() {
            sol = s_(sol);
        }

        (self.tail)(sol)
    }

    pub fn add_one_body(&mut self,a:fn(Array1<i32>) -> Array1<i32>) {
        self.s.push(a);
    }

    pub fn mod_tail(&mut self,nt:fn(Array1<i32>) -> f32) {
        self.tail = nt;
    }
}

// euclid's reducer
