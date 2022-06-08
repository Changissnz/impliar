use round::round;

pub fn f32_is_integer(f:f32) -> bool {
    f == round(f as f64,0) as f32
}

/*
function to determine number of decimal places of f32
*/
pub fn f32_decimal_length(f:f32,t:Option<usize>) -> usize {
    let t_:usize = if t.is_none() {usize::MAX} else {t.unwrap()};
    let mut c:usize = 0;
    let mut f_ = f.clone();
    while !f32_is_integer(f_) && c < t_ {
        c += 1;
        println!("NEW: {}",f_);
        f_ *= 10.;
        //println!("NEW: {}",f_);
    }
    c
}

pub fn f32_to_i32(f:f32,t:Option<usize>) -> i32 {
    let u = f32_decimal_length(f,t);
    let f2 = f32::powf(10.,u as f32);
    (f * f2) as i32
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
}
