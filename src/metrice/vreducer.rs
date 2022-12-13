use ndarray::{arr1,Array1};
use crate::metrice::gorillasf;
use crate::enci::skew;
use crate::enci::skewf32;
use crate::setti::dessi;
use std::fmt;

/// cast struct for function f, which takes an arr1<f32>
/// as input and outputs another arr1<f32>. 
#[derive(Clone)]
pub struct FCast {
    pub f: fn(Array1<f32>) -> Array1<f32>
}

/// cast struct for function f, which takes an arr1<f32>
/// as input and produces another f32 x, and adding `ai`
/// to x is the output from struct main function <apply>.    
#[derive(Clone)]
pub struct FCastF32 {
    pub f: fn(Array1<f32>) -> f32,
    pub ai: f32 // spare adder
}

/// struct main function is <apply> on an arr1<f32> to produce
/// another arr1<f32>. 
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

/// structure that acts as a chained-function (a sequence of functions).
/// Each function in the sequence can be of type <vreducer::FCast> or
/// type <skewf32::SkewF32>. Main function is <VRed::apply>. 
#[derive(Clone)]
pub struct VRed  {
    /// sequence of <vreducer::FCast>.
    pub fvec: Vec<FCast>,
    /// sequence of <skewf32::SkewF32>. 
    pub svec: Vec<skewf32::SkewF32>,
    /// binary sequence with value at each index signifying if fvec function (0) or svec function (1) is used.
    pub directions: Vec<usize>,
    /// boolean used to switch between fvec and svec
    pub switch_f: usize,
    /// index of fvec function during <vreducer::VRed> apply function
    pub fi:usize,
    /// index of svec function during <vreducer::VRed> apply function
    pub si:usize,
    /// last function in apply, produces a f32 singleton.
    pub tail1: Option<FCastF32>,
    /// last function in apply, produces a f32 vector.
    pub tailn: Option<skewf32::SkewF32>,
    /// vector of adder skews for tail-1
    pub tail1_skew: Vec<f32>
}


pub fn build_VRed(fv:Vec<FCast>,sv:Vec<skewf32::SkewF32>,
    direction:Vec<usize>,switch_f:usize,
    tail1:Option<FCastF32>, tailn: Option<skewf32::SkewF32>) -> VRed {
    VRed{fvec:fv,svec:sv,directions:direction,switch_f:switch_f,
    fi:0,si:0,tail1:tail1,tailn:tailn,tail1_skew:Vec::new()}
}


impl fmt::Display for VRed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("fsz {} ssz {}",self.fvec.len(),self.svec.len());//String::from("skalos {}",self.s);
        s += &format!("tail1 {} tailn {}",!self.tail1.is_none(),!self.tailn.is_none());
        write!(f, "{}", s)
    }
}

impl VRed {

    /// applies body function (fvec|svec functions) on argument `a`.
    /// If `tail_type` is 0, applies tail1, o.w. applies tailn. 
    pub fn apply(&mut self,a:Array1<f32>,tail_type:usize) -> (Option<f32>,Option<Array1<f32>>) {
        let a2 = self.apply_body(a);
        
        // tail1
        if tail_type == 0 {
            if self.tail1.is_none() {
                println!("[!] vred:: unintended output");
                return (None,Some(a2));
            }
            let mut a3 = (self.tail1.clone().unwrap()).apply(a2);
            a3 += self.tail1_skew_sum();
            return (Some(a3),None);
        }

        // tailn
        if self.tailn.is_none() {
            return (None,Some(a2));
        }

        let a3 = (self.tailn.clone().unwrap()).skew_value(a2);
        (None,Some(a3))
    }

    /// # description
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

    /// checks validity of `directions` with respect to function vectors
    /// `fvec` and `svec`. 
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

    /// index-reset function. 
    pub fn reset_i(&mut self) {
        self.fi = 0;
        self.si = 0;
    }

    /// outputs the number of functions in body function. 
    pub fn size_fs(&mut self) -> usize {
        self.fvec.len() + self.svec.len()
    }

    pub fn current_switch(&mut self) -> usize {
        let q = self.switch_f.clone();
        (q + self.directions.len() - 1) % 2
    }

    /// adds a <vreducer::FCast> function to `fvec`  and modifies directions. 
    pub fn add_f(&mut self,a: FCast) {
        let cs = self.current_switch();

        if cs != 0 {
            let sz = self.size_fs();
            self.directions.push(sz);
        }
        self.fvec.push(a);
    }

    /// adds a <skewf32::SkewF32> to `svec` and modifies directions. 
    pub fn add_s(&mut self,a: skewf32::SkewF32) {
        let cs = self.current_switch();

        if cs != 0 {
            let sz = self.size_fs();
            self.directions.push(sz);
        }

        self.svec.push(a);
    }

    /// replaces `tailn` with argument `nt`. 
    pub fn mod_tailn(&mut self,nt:skewf32::SkewF32) {
        self.tailn = Some(nt);
    }

    /// # description
    /// adds an existing `tailn` to the end of `svec` and replaces
    /// `tailn` with `nt`. 
    pub fn mod_tailn_(&mut self,nt:skewf32::SkewF32) {

        if !self.tailn.is_none() {
            self.add_s(self.tailn.clone().unwrap());
        }

        self.mod_tailn(nt);
    }

    pub fn mod_tail1(&mut self,nt:FCastF32) {
        self.tail1 = Some(nt);
    }

    /// # description
    pub fn add_tail1_skew(&mut self,f:f32) {
        self.tail1_skew.push(f);
    }

    pub fn tail1_skew_sum(&mut self) -> f32 {
        let x: Array1<f32> = self.tail1_skew.clone().into_iter().collect();
        x.sum()
    }

}

pub fn sample_vred_euclids_reducer() -> VRed {
    let sv1: Vec<FCast> = vec![FCast{f:std_euclids_reducer}];
    build_VRed(sv1,Vec::new(),vec![0],0,None,None)
}



/// constructs addit <skewf32::SkewF32>.
pub fn sample_vred_addit_skew(a:Array1<f32>,t:usize) -> skewf32::SkewF32 {
    // get size
    let v_:Array1<i32> = a.into_iter().map(|x1| ((x1 * f32::powf(10.,t as f32))).round() as i32).collect();
    let sk = skew::build_skew(None,None,Some(v_),None,vec![2],None);
    skewf32::SkewF32{sk:sk,s:t}
}

/// constructs adder <skewf32::SkewF32>.
pub fn sample_vred_adder_skew(a:i32,t:usize) -> skewf32::SkewF32 {
    // get size
    let sk = skew::build_skew(Some(a),None,None,None,vec![0],None);
    skewf32::SkewF32{sk:sk,s:t}
}

/// constructs multer <skewf32::SkewF32>.
pub fn sample_vred_multer_skew(m:i32) -> skewf32::SkewF32 {
    // get size
    let sk = skew::build_skew(None,Some(m),None,None,vec![1],None);
    skewf32::SkewF32{sk:sk,s:1}
}



/// sample `fvec`, `svec` used for testing.
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
    sv.push(sample_vred_addit_skew(m1,5));
    sv.push(sample_vred_addit_skew(m2,5));

    (fv,sv)
}

/// # description
/// average for gorilla euclid additive vector A and coefficient vector C
pub fn std_euclids_reducer(s:Array1<f32>) -> Array1<f32> {
    let s1: Array1<i32> = s.into_iter().map(|x| x as i32).collect();
    let (g1,g2) = gorillasf::gorilla_touch_arr1_basic(s1,0.5);
    (g1 + g2) / 2.0
}

/// # description
/// uses greatest common denominator to reduce
pub fn std_gcd_reducer(s:Array1<f32>) -> Array1<f32> {
    let s1: Array1<i32> = s.into_iter().map(|x| x as i32).collect();
    gorillasf::gorilla_touch_arr1_gcd(s1,0.5)
}

/// # descrption
/// one-percent reducer; outputs a \<0.01\> vector of equal
/// length to `s`;
/// A <gorillains::GorillaIns> that uses a <vreducer::VRed> 
/// with only this function is able to determine the "baseline"
/// a-factors and m-factors necessary to correct the skews for
/// the samples given their labelling.
pub fn one_reducer(s:Array1<f32>) -> Array1<f32> {
    s.into_iter().map(|x| 0.01).collect()
}



/// # description
/// mainly used for purpose of testing skews.
pub fn identity_reducer(s: Array1<f32>) -> Array1<f32> {
    s
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
        let rxx = sample_vred_addit_skew(yx,5);
        vr.mod_tailn(rxx);


        let s1:Array1<f32> = arr1(&[-5.,6.3,15.0,-20.34,2.31313]);
        let x2 = vr.apply(s1.clone(),2);
        assert_eq!(x2.1.unwrap(),arr1(&[7.0, 18.0, 16.0, 224.0, 4.0]));
    }

}
