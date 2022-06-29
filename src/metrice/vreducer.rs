use ndarray::{arr1,Array1};
use crate::metrice::gorillasf;
use crate::enci::skew;
use crate::enci::skewf32;
use crate::setti::dessi;
use std::fmt;

/*
cast for function
*/
#[derive(Clone)]
pub struct FCast {
    pub f: fn(Array1<f32>) -> Array1<f32>
}

#[derive(Clone)]
pub struct FCastF32 {
    pub f: fn(Array1<f32>) -> f32,
    pub ai: f32 // spare adder
}

impl FCast {

    pub fn apply(&mut self,a:Array1<f32>) -> Array1<f32> {
        (self.f)(a)
    }
}

impl FCastF32 {

    pub fn apply(&mut self,a:Array1<f32>) -> f32 {
        (self.f)(a) + self.ai
    }
}

#[derive(Clone)]
pub struct VRed  {
    pub fvec: Vec<FCast>,
    pub svec: Vec<skewf32::SkewF32>,

    // binary vec
    pub directions: Vec<usize>,
    pub switch_f: usize,

    pub fi:usize,
    pub si:usize,

    // tail is the skew
    pub tail1: Option<FCastF32>,
    pub tailn: Option<skewf32::SkewF32>
}


pub fn build_VRed(fv:Vec<FCast>,sv:Vec<skewf32::SkewF32>,
    direction:Vec<usize>,switch_f:usize,
    tail1:Option<FCastF32>, tailn: Option<skewf32::SkewF32>) -> VRed {
    VRed{fvec:fv,svec:sv,directions:direction,switch_f:switch_f,
    fi:0,si:0,tail1:tail1,tailn:tailn}
}


impl fmt::Display for VRed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("fsz {} ssz {}",self.fvec.len(),self.svec.len());//String::from("skalos {}",self.s);
        s += &format!("tail1 {} tailn {}",!self.tail1.is_none(),!self.tailn.is_none());
        write!(f, "{}", s)
    }

}

impl VRed {

    pub fn apply(&mut self,a:Array1<f32>,tail_type:usize) -> (Option<f32>,Option<Array1<f32>>) {
        let a2 = self.apply_body(a);
        // tail1
        if tail_type == 0 {
            if self.tail1.is_none() {
                println!("[!] vred:: unintended output");
                return (None,Some(a2));
            }
            let a3 = (self.tail1.clone().unwrap()).apply(a2);
            return (Some(a3),None);
        }

        // tailn
        if self.tailn.is_none() {
            return (None,Some(a2));
        }

        let a3 = (self.tailn.clone().unwrap()).skew_value(a2);
        (None,Some(a3))
    }

    pub fn apply_body(&mut self,a:Array1<f32>) -> Array1<f32> {
        self.reset_i();

        let mut q = self.switch_f.clone();
        let l = self.directions.len();

        let mut sol = a.clone();

        for i in 1..l {
            // check off all indices prior: (prior,i)
            let (x1,x2) = (self.directions[i - 1].clone(),self.directions[i].clone());
            let d = x2 - x1;

            // f
            if q == 0 {
                for j in 0..d {
                    sol = self.fvec[self.fi].apply(sol);
                    self.fi += 1;
                }
            } else {
            // s
                for j in 0..d {
                    sol = self.svec[self.si].skew_value(sol);
                    self.si += 1;
                }
            }

            // switch
            q = (q + 1) % 2;
        }

        // iterate through the end of svec or fvec
        if q == 0 {
            let l = self.fvec.len();
            for i in self.fi..l {
                sol = self.fvec[self.fi].apply(sol);
                self.fi += 1;
            }
        } else {
            let l = self.svec.len();
            for i in self.si..l {
                sol = self.svec[self.si].skew_value(sol);
                self.si += 1;
            }
        }
        sol
    }

    pub fn check_directions(&mut self) -> bool {
        let mut q = self.switch_f.clone();
        let l = self.directions.len();

        let (lf,ls) = (self.fvec.len(),self.svec.len());
        for i in 1..l {
            // check off all indices prior: (prior,i)
            let x2 = self.directions[i].clone();
            let qr:usize = if q == 1 {ls.clone()} else {lf.clone()};

            if x2 >= qr {
                return false;
            }

            // switch
            q = (q + 1) % 2;
        }
        true
    }

    pub fn reset_i(&mut self) {
        self.fi = 0;
        self.si = 0;
    }

    pub fn size_fs(&mut self) -> usize {
        self.fvec.len() + self.svec.len()
    }

    pub fn current_switch(&mut self) -> usize {
        let q = self.switch_f.clone();
        (q + self.directions.len() - 1) % 2
    }

    pub fn add_f(&mut self,a: FCast) {
        let cs = self.current_switch();

        if cs != 0 {
            let sz = self.size_fs();
            self.directions.push(sz);
        }
        self.fvec.push(a);
    }

    pub fn add_s(&mut self,a: skewf32::SkewF32) {
        let cs = self.current_switch();

        if cs != 0 {
            let sz = self.size_fs();
            self.directions.push(sz);
        }

        self.svec.push(a);
    }

    pub fn mod_tailn(&mut self,nt:skewf32::SkewF32) {
        self.tailn = Some(nt);
    }

    pub fn mod_tailn_(&mut self,nt:skewf32::SkewF32) {

        if !self.tailn.is_none() {
            self.add_s(self.tailn.clone().unwrap());
        }

        self.mod_tailn(nt);

    }

    pub fn mod_tail1(&mut self,nt:FCastF32) {
        self.tail1 = Some(nt);
    }
}

pub fn sample_vred_adder_skew(a:Array1<f32>,t:usize) -> skewf32::SkewF32 {

    // get size
    //let y:usize = a.clone().into_iter().map(|x1| dessi::f32_decimal_length(x1,Some(t))).into_iter().max().unwrap();
    let v_:Array1<i32> = a.into_iter().map(|x1| ((x1 * f32::powf(10.,t as f32))).round() as i32).collect();
    let sk = skew::build_skew(None,None,Some(v_),None,vec![2],None);
    skewf32::SkewF32{sk:sk,s:t}
}

pub fn sample_fsvecs() -> (Vec<FCast>,Vec<skewf32::SkewF32>) {

    fn f1(x:Array1<f32>) -> Array1<f32> {
        x / 2.
    }

    fn f2(x:Array1<f32>) -> Array1<f32> {
        x + 2.
    }

    fn f3(x:Array1<f32>) -> Array1<f32> {
        x * 2.
    }

    let mut fv: Vec<FCast> = Vec::new();
    fv.push(FCast{f:f1});
    fv.push(FCast{f:f2});
    fv.push(FCast{f:f3});

    let mut sv: Vec<skewf32::SkewF32> = Vec::new();
    let m1:Array1<f32> = arr1(&[1.,3.134,54.12,60.11111,-55.2]);
    let m2:Array1<f32> = arr1(&[3.134,1.,-55.2,60.11111,54.12]);
    sv.push(sample_vred_adder_skew(m1,5));
    sv.push(sample_vred_adder_skew(m2,5));

    (fv,sv)
}

/*
average for gorilla euclid additive vector A and coefficient vector C
*/
pub fn std_euclids_reducer(s:Array1<f32>) -> Array1<f32> {
    let s1: Array1<i32> = s.into_iter().map(|x| x as i32).collect();
    let (g1,g2) = gorillasf::gorilla_touch_arr1_basic(s1,0.5);
    (g1 + g2) / 2.0
}

pub fn std_gcd_reducer(s:Array1<f32>) -> Array1<f32> {
    let s1: Array1<i32> = s.into_iter().map(|x| x as i32).collect();
    gorillasf::gorilla_touch_arr1_gcd(s1,0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_VRed__apply_body() {
        let (f,s) = sample_fsvecs();
        let mut vr = build_VRed(f.clone(),s.clone(),vec![0,2,4],0,None,None);

        let s1:Array1<f32> = arr1(&[-5.,6.3,15.0,-20.34,2.31313]);
        let xx = vr.apply_body(s1.clone());
        assert_eq!(xx,arr1(&[7.268, 18.568, 16.84, 224.10445, 4.15312]));

        let mut vr2 = build_VRed(f.clone(),s.clone(),vec![0],0,None,None);
        let xx2 = vr2.apply_body(s1.clone());
        assert_eq!(xx2,arr1(&[-1.0, 10.3, 19.0, -16.34, 6.31313]));

        let mut vr3 = build_VRed(f.clone(),s.clone(),vec![0],1,None,None);
        let xx3 = vr3.apply_body(s1);
        assert_eq!(xx3,arr1(&[-0.866, 10.434, 13.92, 99.88222, 1.23312]));

    }

    #[test]
    fn test_VRed__apply() {
        let (f,s) = sample_fsvecs();
        let mut vr = build_VRed(f,s,vec![0,2,4],0,None,None);

        // add skew
        let yx = arr1(&[-0.268, -0.568, -0.84, -0.10445, -0.15312]);
        let rxx = sample_vred_adder_skew(yx,5);
        vr.mod_tailn(rxx);


        let s1:Array1<f32> = arr1(&[-5.,6.3,15.0,-20.34,2.31313]);
        let x2 = vr.apply(s1.clone(),2);
        assert_eq!(x2.1.unwrap(),arr1(&[7.0, 18.0, 16.0, 224.0, 4.0]));
    }

}
