use ndarray::{Array,Array1,arr1};
use crate::enci::skew::Skew;
use crate::enci::skew;

// find cheapest IndexFractionNotation formation to replicate a sequence

/*
calculates a skew that can transform v1 to v2
*/

pub fn cheapest_skew_cost_function(s:Skew) -> f32 {
    let mut c: f32 = 0.0;

    if !s.adder.is_none() {
        let mut sm: i32 = s.adder.unwrap();
        c += sm as f32;
    }

    if !s.multer.is_none() {
        let mut sm: i32 = s.multer.unwrap();
        c += sm as f32;
    }

    if !s.addit.is_none() {
        let mut sm2: Array1<i32> = s.addit.unwrap();
        c += sm2.sum() as f32;
    }

    if !s.multit.is_none() {
        let mut sm2: Array1<i32> = s.multit.unwrap();
        c += sm2.sum() as f32;
    }

    c
}

pub fn cheapest_multiple(v1:Array1<i32>,v2:Array1<i32>) -> i32 {
    let mut cmv = cheapest_multiple_vec(v1.clone(),v2.clone());
    let mut f1:f32 = v1.sum() as f32;
    let mut f2:f32 = v2.sum() as f32;


    (f2 / f1).round() as i32



}

pub fn mean_multiple(v1:Array1<i32>,v2:Array1<i32>) ->i32 {
    let mut cmv = cheapest_multiple_vec(v1.clone(),v2.clone());
    if cmv.len() == 0 {
        return 0;
    }

    let mean: f32 = (cmv.sum() as f32) / (cmv.len() as f32);

    // find the nearest multiple to the mean
    let mut nearest:i32 = cmv[0];
    let mut nearestDiff:f32 = (mean - nearest  as f32).abs();

    for i in 1..cmv.len() {
        let mut nd2:f32 = (mean - cmv[i] as f32).abs();
        if nd2 < nearestDiff {
            nearestDiff = nd2;
            nearest = cmv[i];
        }
    }
    nearest
}

pub fn cheapest_multiple_vec(v1:Array1<i32>,v2:Array1<i32>) ->Array1<i32> {
    let v1x:Array1<f32> = v1.into_iter().map(|x| x as f32).collect();
    let v2x:Array1<f32> = v2.into_iter().map(|x| x as f32).collect();
    let v3x:Array1<f32> = arr1_safe_divide(v2x,v1x,0.0);
    let v3x:Array1<i32> = v3x.into_iter().map(|x| x.round() as i32).collect();
    v3x
}

/*
is also the mean
*/
pub fn cheapest_add(v1:Array1<i32>,v2:Array1<i32>) ->i32 {
    if v2.len() == 0 {
        return 0;
    }

    (((v2.clone() - v1.clone()).sum() as f32) / (v2.len() as f32).round()) as i32

}

pub fn cheapest_add_vec(v1:Array1<i32>,v2:Array1<i32>) ->Array1<i32> {
    v2 - v1
}

pub fn arr1_safe_divide(v1:Array1<f32>,v2:Array1<f32>,n:f32) -> Array1<f32> {
    assert_eq!(v1.len(),v2.len());
    let mut v:Vec<f32> = Vec::new();
    for (i,x) in v1.iter().enumerate() {
        if v2[i] == 0.0 {
            v.push(n);
        } else {
            v.push(x / v2[i]);
        }
    }
    arr1(&v)
}

pub struct SkewEncoder {
    pub skewChain: Vec<Skew>,
}

pub fn sample_arr1_pair_1() -> (Array1<i32>,Array1<i32>) {
    let mut v1:Array1<i32> = arr1(&[2,4,15,19]);
    let mut v2:Array1<i32> = arr1(&[8,12,55,190]);
    (v1,v2)
}

pub fn sample_arr1_pair_2() -> (Array1<i32>,Array1<i32>) {
    let mut v1:Array1<i32> = arr1(&[2,4,2,4,20]);
    let mut v2:Array1<i32> = arr1(&[4,8,4,8,200]);
    (v1,v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cheapest_multiple_vec() {
        let (v1,v2):(Array1<i32>,Array1<i32>) = sample_arr1_pair_1();
        let mut cmv = cheapest_multiple_vec(v1.clone(),v2.clone());
        assert_eq!(cmv,arr1(&[4,3,4,10]));
    }

    #[test]
    fn test_mean_multiple() {
        let (v1,v2):(Array1<i32>,Array1<i32>) = sample_arr1_pair_1();
        let mut cmv = mean_multiple(v1.clone(),v2.clone());
        assert_eq!(cmv,4);
    }

    #[test]
    fn test_cheapest_multiple() {
        let (v1,v2):(Array1<i32>,Array1<i32>) = sample_arr1_pair_1();
        let mut cmv = cheapest_multiple(v1.clone(),v2.clone());
        assert_eq!(cmv,7);
    }

    #[test]
    fn test_cheapest_add_vec() {
        let (v1,v2):(Array1<i32>,Array1<i32>) = sample_arr1_pair_1();
        let mut cmv = cheapest_add_vec(v1.clone(),v2.clone());
        assert_eq!(cmv,arr1(&[6, 8, 40, 171]));
    }

    #[test]
    fn test_cheapest_add() {
        let (v1,v2):(Array1<i32>,Array1<i32>) = sample_arr1_pair_2();
        let mut cmv = cheapest_add(v1.clone(),v2.clone());
        assert_eq!(cmv,38);
    }

}
