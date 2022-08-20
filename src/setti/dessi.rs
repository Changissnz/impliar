//! decimal to integer conversion functions
use round::round;
use ndarray::{arr1,Array1};

pub fn f32_is_integer(f:f32) -> bool {
    f == round(f as f64,0) as f32
}

/// # description
/// number of decimal places of f32
/// 
/// # arguments
/// `f` := target f32
/// t := max decimal places
///
/// # return
/// decimal places `d` of `f` or MIN<`d`,`t`> if `t` is not None
pub fn f32_decimal_length(f:f32,t:Option<usize>) -> usize {
    let t_:usize = if t.is_none() {usize::MAX} else {t.unwrap()};
    let mut c:usize = 0;
    let mut f_ = f.clone();
    while !f32_is_integer(f_) && c < t_ {
        c += 1;
        f_ *= 10.;
    }
    c
}

/// # description
pub fn f32_to_i32(f:f32,t:Option<usize>) -> i32 {
    let u = f32_decimal_length(f,t);
    let f2 = f32::powf(10.,u as f32);
    (f * f2) as i32
}

/// # return
/// converted f32 sequence to i32 sequence based on `v`'s decimal length.
pub fn arr1_f32_to_arr1_i32(v:Array1<f32>) -> Array1<i32> {
    if v.len() == 0 {
        return Array1::zeros(0);
    }

    let x:Array1<usize> = v.clone().into_iter().map(|x| f32_decimal_length(x,None)).collect();
    let xm: usize = x.into_iter().max().unwrap();
    v.into_iter().map(|x| (x * f32::powf(10.,xm as f32)) as i32).collect()
}

/// # return
/// converted f32 sequence to i32 sequence based on decimal length `s`.
pub fn scale_arr1_f32_to_arr1_i32(v:Array1<f32>,s:usize) -> Array1<i32> {
    if v.len() == 0 {
        return Array1::zeros(0);
    }

    v.into_iter().map(|x| ((x * f32::powf(10.,s as f32))).round() as i32).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_decimal_length() {
        let d:f32 = 4.11313;
        let u = f32_decimal_length(d,Some(50));
        assert!(f32_is_integer(d * f32::powf(10.,u as f32)));
    }

    #[test]
    fn test_f32_to_i32() {
        let d:f32 = 4.11313;
        let u = f32_to_i32(d,Some(3));
        assert_eq!(u,4113);
        let u = f32_to_i32(d,Some(5));
        assert_eq!(u,411313);
    }

    #[test]
    fn test_arr1_f32_to_arr1_i32() {
        let x1:Array1<f32> = arr1(&[1.,3.4567242,34.432]);
        let x2 = arr1_f32_to_arr1_i32(x1);
        let sol:Array1<i32> = arr1(&[10000000, 34567240, 344320000]);
        assert_eq!(x2,sol);
    }
}
