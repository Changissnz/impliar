/*
measures on proper bounds b = (start,end); start < end
*/
extern crate round;

use ndarray::{Array1,arr1};
use round::round;
use crate::enci::mat2sort;

pub fn is_proper_bounds(b:(f32,f32)) -> bool {
    b.0 <= b.1
}

/*
*/
pub fn in_bounds(b:(f32,f32),f:f32) -> bool {
    assert!(is_proper_bounds(b.clone()));
    f >= b.0 && f <= b.1
}

/*

*/
pub fn is_subbound(b:(f32,f32),b2:(f32,f32)) -> bool {
    assert!(is_proper_bounds(b.clone()));
    assert!(is_proper_bounds(b.clone()));

    in_bounds(b.clone(),b2.0) && in_bounds(b.clone(),b2.1)
}

/*
f,b2 in b1.
*/
pub fn closest_distance_to_subbound(b1:(f32,f32),b2:(f32,f32),f:f32) -> f32 {
    assert!(in_bounds(b1.clone(),f), "bounds {:?} for {}",b1,f);
    assert!(in_bounds(b1.clone(),b2.0) && in_bounds(b1.clone(),b2.1));

    if in_bounds(b2.clone(),f) {
        return 0.;
    }

    // try b2.0
    let bd0 = bdistance_of_f32pair((f.clone(),b2.0.clone()),b1);
    let bd1 = bdistance_of_f32pair((f.clone(),b2.1.clone()),b1);

    // get
    if bd0.abs() < bd1.abs() {
        return bd0;
    }

    bd1
}

/*

*/
pub fn bounds_intersect(b1:(f32,f32),b2:(f32,f32)) -> bool {
    in_bounds(b2.clone(),b1.0) || in_bounds(b2.clone(),b1.1)
}

/*
if f in b: output f
otherwise: output the modulo value v of f based on its under or over bounds; v in b.
*/
pub fn calibrate_in_bounds(b:(f32,f32),f:f32) -> f32 {
    assert!(is_proper_bounds(b.clone()));

    // case: f in b
    if f >= b.0 && f <= b.1 {
        return f;
    }

    // case: f <= b.0, calibrate by -1 direction
    let bl = b.1 - b.0;

    // get diff
    let mut diff = if f < b.0 {f - b.0} else {f - b.1};
    let a = if diff < 0. {bl.clone()} else {-1. * bl.clone()};

    let mut sol = f.clone();
    while diff != 0. {

        if diff.abs() < bl {
            if diff < 0. {
                sol = b.1 + diff;
            } else {
                sol = b.0 + diff;
            }
            diff = 0.;
            continue;
        }

        diff = diff + a;
    }

    sol
}

/*
bounded distance, minumum absolute distance of f32 pair on bounds
*/
pub fn bdistance_of_f32pair(p:(f32,f32),b:(f32,f32)) -> f32 {
    assert!(is_proper_bounds(b.clone()));
    assert!(is_proper_bounds(b.clone()));
    assert!(in_bounds(b.clone(),p.0.clone()) && in_bounds(b.clone(),p.1.clone()));

    // get the positive distance
    let d = p.1 - p.0;

    // get the negative distance
    let d2 = (b.1 - p.1) + (p.0 - b.0);

    if d < d2 {
        return d;
    }

    -d2
}

/*
conducts f32 addition of a to f in the bounds b
*/
pub fn additive_in_bounds(b:(f32,f32),f:f32,a:f32) -> f32 {
    assert!(is_proper_bounds(b.clone()));
    assert!(in_bounds(b.clone(),f.clone()), "have bounds {:?} value {}",b.clone(),f);

    // && in_bounds(b.clone(),a.clone()));

    // check for remainder
    calibrate_in_bounds(b.clone(),f + a)
}



/*
vec. with elements that are all non-intersecting proper bounds?
*/
pub fn is_proper_bounds_vec(bv: Vec<(f32,f32)>) -> bool {
    let l = bv.len();
    for i in 0..l - 1 {
        let mut q = bv[i].clone();

        for j in i + 1..l {
            if bounds_intersect(q.clone(),bv[j].clone()) {
                return false;
            }
        }
    }
    true
}

/*
outputs the subvector of indices of bv that intersect with v
*/
pub fn intersecting_bounds_to_bound(bv: Vec<(f32,f32)>, v: (f32,f32)) -> Vec<usize> {
    let mut sol: Vec<usize> = Vec::new();

    for (i,b) in bv.into_iter().enumerate() {
        if bounds_intersect(v.clone(),b.clone()) {
            sol.push(i);
        }
    }
    sol
}

/*
merges the vector of proper bounds bv into one bound
*/
pub fn merge_bounds(bv: Vec<(f32,f32)>) -> (f32,f32) {
    let maximum = bv.clone().into_iter().fold(f32::MIN, |acc,s| if s.1 > acc {s.1.clone()} else {acc});
    let minimum = bv.clone().into_iter().fold(f32::MAX, |acc,s| if s.0 < acc {s.0.clone()} else {acc});
    (minimum,maximum)
}

/*
outputs a subbound f32 of refb (a proper bound of f in (real numbers)^2)
based on b of [0.,1.]^2.
*/
pub fn bound_01_to_subbound_f32(refb:(f32,f32),b:(f32,f32)) -> (f32,f32) {
    assert!(is_proper_bounds(refb.clone()));
    assert!(is_proper_bounds(b.clone()));
    let d = refb.1.clone() - refb.0.clone();
    let s = refb.0 + b.0 * d;
    let e = refb.0 + b.1 * d;
    (s,e)
}

/*
outputs a bound f2 in [0.,1.]^2 for f in (real numbers)^2 based on refb (a proper bound
 of f in (real numbers)^2).
*/
////

/*
b2 is subbound of b1
*/
pub fn subbound_f32_to_bound_01(b1:(f32,f32),b2:(f32,f32)) -> (f32,f32) {

    assert!(is_proper_bounds(b2.clone()));
    assert!(is_proper_bounds(b1.clone()));

    assert!(in_bounds(b1.clone(),b2.0));
    assert!(in_bounds(b1.clone(),b2.1));

    let d = b1.1.clone() - b1.0.clone();
    if d == 0. {
        return (0.,0.);
    }


    let f1 = (b2.0.clone() - b1.0.clone()) / d;
    let f2 = (b2.1.clone() - b1.0.clone()) / d;
    (f1,f2)
}

/*
*/
pub fn bounds_of_bv(bv: Vec<(f32,f32)>) -> (f32,f32) {
    assert!(bv.len() != 0);

    // collect all into 1-d
    let mut a: Vec<f32> = Vec::new();//bv.into_iter();

    for b in bv.into_iter() {
        a.push(b.0.clone());
        a.push(b.1.clone());
    }

    let mut c: Array1<f32> = a.into_iter().collect();
    c = mat2sort::sort_arr1(c,mat2sort::f32_cmp1);
    let l = c.len();

    (c[0],c[l -1])
}

/*
maps vector of bounds of (real numbers)^2 to bounds of [0,1]^2 by
[min(fv as arr1),max(fv as arr1)]
*/
pub fn bvec_f32_to_bvec_01(fv:Vec<(f32,f32)>) -> Vec<(f32,f32)> {
    let bvv = merge_bounds(fv.clone());
    fv.clone().into_iter().map(|x| subbound_f32_to_bound_01(bvv.clone(),x)).collect()
}

pub fn bvec_01_to_bvec_f32(bv:Vec<(f32,f32)>,refb:(f32,f32)) -> Vec<(f32,f32)> {
    bv.clone().into_iter().map(|x| bound_01_to_subbound_f32(refb.clone(),x)).collect()
}

    //////////////////////// specialized functions for i32

/*
special case of cheapest add:

soln is i32
*/
pub fn pos_neg_add_vecs_target_i32(v1:Array1<i32>,b:(i32,i32),li:i32) -> (Array1<i32>,Array1<i32>) {
    // get + & - values for v1 to li
    let mut pv: Vec<i32> = Vec::new();
    let mut nv: Vec<i32> = Vec::new();
    for v in v1.into_iter() {
        if v > li {
            pv.push(b.1 - v + li - b.0);
            nv.push(li - v);
        } else {
            pv.push(li - v);
            nv.push(-(v - b.0 + b.1 - li));
        }
    }

    (pv.into_iter().collect(),nv.into_iter().collect())
}

pub fn bounded_cheapest_add_target_i32_(v1:Array1<i32>,b:(i32,i32),li:i32) -> Array1<i32> {
    let (x1,x2) = pos_neg_add_vecs_target_i32(v1.clone(),b.clone(),li);
    let mut sol:Vec<i32> = Vec::new();
    let l = x1.len();
    for i in 0..l {
        if x1[i].abs() < x2[i].abs() {
            sol.push(x1[i]);
        } else {
            sol.push(x2[i]);
        }
    }
    sol.into_iter().collect()
}

/*
*/
pub fn bounded_cheapest_add_target_i32(v1:Array1<i32>,b:(i32,i32),li:i32) -> i32 {
    let (pv,nv) = pos_neg_add_vecs_target_i32(v1.clone(),b.clone(),li);

    // get average of each
    let (a1,a2) = (pv.mean().unwrap(),nv.mean().unwrap());

    // add each average to v1
    let mut v2 = v1.clone() + round(a1 as f64,0) as i32;
    let mut v3 = v1.clone() + round(a2 as f64,0) as i32;
    let l = v1.len();

    // calibrate each value in bounds
    // determine cumulative absolute bdistance for each of v2,v3
    let (mut c2,mut c3):(i32,i32) = (0,0);
    for i in 0..l {
        // calibrate
        let v2_ = calibrate_in_bounds((b.0 as f32,b.1 as f32),v2[i] as f32);
        let v3_ = calibrate_in_bounds((b.0 as f32,b.1 as f32),v3[i] as f32);

        // abs bdistance
        let d2 = bdistance_of_f32pair((v2_,li as f32),(b.0 as f32,b.1 as f32));
        let d3 = bdistance_of_f32pair((v3_,li as f32),(b.0 as f32,b.1 as f32));

        c2 = c2 + (round(d2 as f64,0) as i32).abs();
        c3 = c3 + (round(d3 as f64,0) as i32).abs();
    }

    // determine which value results in the least distance
    if c2 < c3 {
        return a1 as i32;
    }
    a2 as i32
}

pub fn calibrate_arr1_i32_in_bounds(v1:Array1<i32>,b:(i32,i32)) -> Array1<i32> {
    let mut f:Vec<i32> = Vec::new();

    for v in v1.into_iter() {
        let x = calibrate_in_bounds((b.0 as f32,b.1 as f32),v as f32) as i32;
        f.push(x);
    }
    f.into_iter().collect()
}

pub fn abs_arr1_bdistance(v1:Array1<i32>, f:i32, b:(i32,i32)) -> usize {

    let x:Array1<usize> = v1.into_iter().map(|x| bdistance_of_f32pair((x as f32,f as f32),(b.0 as f32,b.1 as f32)).abs() as usize ).collect();
    x.sum()

}

/////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn test_sample_bmeas_info() -> ((f32,f32),(f32,f32),(f32,f32)) {
    let p1: (f32,f32)  = (0.15,0.31);
    let p2: (f32,f32)  = (0.3,1.1);
    let f: (f32,f32) = (0.05,1.2);
    (p1,p2,f)
}

pub fn test_sample_bmeas_info_2() -> (Array1<i32>,(i32,i32),i32) {
    let x:Array1<i32> = arr1(&[1,100,112,314]);
    let b:(i32,i32) = (-20,400);
    let l: i32 = 60;
    (x,b,l)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_bdistance_of_f32pair() {

        let (p1,p2,f) = test_sample_bmeas_info();
        let b1 = bdistance_of_f32pair(p1,f.clone());
        assert!(b1 == 0.16);


        let b2 = bdistance_of_f32pair(p2,f.clone());
        let b2_ = round(b2 as f64,2);
        assert!(b2_ == -0.35);
    }

    // additive_in_bounds
    #[test]
    pub fn test_additive_in_bounds() {
        let (p1,p2,f) = test_sample_bmeas_info();
        let b1 = bdistance_of_f32pair(p1,f.clone());
        let b2 = bdistance_of_f32pair(p2,f.clone());
        let b2_ = round(b2 as f64,2) as f32;

        assert!(additive_in_bounds(f.clone(),p1.0,b1) == p1.1);
        assert!(additive_in_bounds(f.clone(),p2.0,b2_ as f32) == p2.1);
    }

    #[test]
    pub fn test_bound_01_to_subbound_f32() {
        assert_eq!(bound_01_to_subbound_f32(
                (-100.,110.),(0.2,0.5)),(-58.,5.));
    }

    #[test]
    pub fn test_subbound_f32_to_bound_01() {
        assert_eq!(subbound_f32_to_bound_01(
                (-10.,10.),(0.0,5.0)),(0.5,0.75));
    }

    #[test]
    pub fn test_bvec_f32_to_bvec_01() {
        let fv: Vec<(f32,f32)> = vec![(-13.0,2.),(4.,24.), (-36.,-16.0)];
        let bv = bvec_f32_to_bvec_01(fv);

        let sol: Vec<(f32,f32)> = vec![(0.38333333, 0.6333333), (0.6666667, 1.0), (0.0, 0.33333334)];
        assert_eq!(sol,bv);
    }

    #[test]
    pub fn test_bvec_01_to_bvec_f32() {
        let fv: Vec<(f32,f32)> = vec![(-13.0,2.),(4.,24.), (-36.,-16.0)];
        let bv = bvec_f32_to_bvec_01(fv.clone());

        let sol: Vec<(f32,f32)> = vec![(0.38333333, 0.6333333), (0.6666667, 1.0), (0.0, 0.33333334)];
        let bl = merge_bounds(fv.clone());
        let bv2 = bvec_01_to_bvec_f32(sol,bl);
        assert_eq!(bv2,fv);
    }

    #[test]
    pub fn test_pos_neg_add_vecs_target_i32() {
        let (x,b,l) = test_sample_bmeas_info_2();

        let (pv,nv) = pos_neg_add_vecs_target_i32(x.clone(),b.clone(),l);
        assert_eq!(pv,arr1(&[59, 380, 368, 166]));
        assert_eq!(nv,arr1(&[-361, -40, -52, -254]));
    }

    #[test]
    pub fn test_bounded_cheapest_add_target_i32_() {
        let (x,b,l) = test_sample_bmeas_info_2();
        let c2 = bounded_cheapest_add_target_i32_(x.clone(),b.clone(),l);
        let x4 = calibrate_arr1_i32_in_bounds(x.clone() + c2,b.clone());
        assert_eq!(0,abs_arr1_bdistance(x4,l,b.clone()));
    }


}
