/*
measures on proper bounds b = (start,end); start < end
*/
extern crate round;

use round::round;

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
f,b2 in b1.
*/
pub fn closest_distance_to_subbound(b1:(f32,f32),b2:(f32,f32),f:f32) -> f32 {
    assert!(in_bounds(b1.clone(),f));
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
    assert!(in_bounds(b.clone(),f.clone()));

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

/////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn test_sample_bmeas_info() -> ((f32,f32),(f32,f32),(f32,f32)) {
    let p1: (f32,f32)  = (0.15,0.31);
    let p2: (f32,f32)  = (0.3,1.1);
    let f: (f32,f32) = (0.05,1.2);
    (p1,p2,f)
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

}
