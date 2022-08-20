//! numerical seeds for types i32,f32 using `std_rng`
use crate::enci::std_rng;

pub trait RN<T>
where T: Into<f64> {
    fn r_next(&mut self) -> T;
}

pub struct Si32 {
    pub r: (i32,i32)
}

pub struct Sf32 {
    pub r: (f32,f32)
}

impl RN<i32> for Si32 {

    fn r_next(&mut self) -> i32 {
        std_rng::random_i32_in_range(self.r.clone())
    }
}

impl RN<f32> for Sf32 {

    fn r_next(&mut self) -> f32 {
        std_rng::random_f32_in_range(self.r.clone())
    }
}
