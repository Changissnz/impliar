/*
f32 version of Skew; uses a scale s for 10^s
scaling of values from Skew
*/
use ndarray::{arr1,Array1};
use crate::enci::skew;

#[derive(Clone)]
pub struct SkewF32 {
    pub sk: skew::Skew,
    pub s: usize
}

impl SkewF32 {

    pub fn skew_value(&mut self, mut v : Array1<f32>) -> Array1<f32> {
        //
        let mut v_:Array1<i32> = v.into_iter().map(|x| (x * f32::powf(10.,self.s as f32)) as i32).collect();
        let ans:Array1<i32> = self.sk.skew_value(v_);
        ans.into_iter().map(|x| (x as f32) / f32::powf(10.,self.s as f32)).collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_SkewF32_skew_value() {
        let a:Array1<i32> = arr1(&[1,5,15,50,60]);

        let sk = skew::build_skew(None,None,Some(a),None,vec![2],None);
        let mut s = SkewF32{sk:sk,s:4};
        let q:Array1<f32> = arr1(&[1.5,2.5,43.2,10.1,20.1]);
        assert_eq!(s.skew_value(q),arr1(&[1.5001, 2.5005, 43.2015, 10.105, 20.106]));
    }

}
