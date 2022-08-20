/*
frequency collector
*/
use crate::enci::mat2sort::f32_cmp1;
use ndarray::Array1;

extern crate round;
use round::round;

pub fn frequency_intervals(l:usize) -> Array1<f32> {
    if l == 0 {
        return Array1::zeros(0);
    }

    let mut sol: Vec<f32> = Vec::new();
    let k = 1. / l as f32;
    let mut s:f32 = 0.;

    for _ in 0..l {
        s += k;
        sol.push(s);
    }
    sol.into_iter().collect()
}

/// frequency collector of values v over a range r by partition p;
/// output counts in pc
pub struct FrqncCl {
    v: Vec<f32>,
    r: (f32,f32),
    p:usize, // partition
    pub pc: Vec<usize>, // count
    pub pci: Vec<f32>, // partition label for count
    pub analysis: f32
}

pub fn build_FrqncCl(v:Vec<f32>,r:(f32,f32),p:usize) -> FrqncCl {
    assert!(v.len() > 0);
    assert!(r.1 > r.0);
    assert!(p > 0);

    let mn = v.iter().fold(f32::MAX,|x,&q| if x > q {q} else {x});
    let mx = v.iter().fold(f32::MIN,|x,&q| if x < q {q} else {x});
    assert!(r.0 <= mn && r.1 >= mx);

    // sort v
    let mut v2 = v.clone();
    v2.sort_by(f32_cmp1);
    FrqncCl{v:v2,r:r,p:p,pc:Vec::new(),pci:Vec::new(),analysis:f32::MAX}
}

impl FrqncCl {

    pub fn adjust_p(&mut self,p2:usize) {
        self.p = p2;
        self.count_pc();
    }

    pub fn frequency_intervals(&mut self,l:usize) -> Array1<f32> {
        let mut x = frequency_intervals(l);
        for i in 0..l {
            let x2 = self.r.0 + x[i] * (self.r.1 - self.r.0);
            x[i] = x2;
        }
        x
    }

    pub fn count_pc(&mut self) {
        self.pc = (0..self.p).into_iter().map(|x| 0).collect();
        self.pci = self.frequency_intervals(self.p).into_iter().collect();

        let mut s:f32 = self.r.0.clone();
        let mut s2:usize = 0;
        let l = self.v.len();
        let mut i = 0;
        while i < l && s2 < self.p {
            if self.v[i] >= s && self.v[i] <= self.pci[s2] {
                self.pc[s2] += 1;
                i += 1;
            } else {
                s = self.pci[s2].clone();
                s2 += 1;
            }
        }
    }

    pub fn analyze(&mut self) {
        let t = self.t();
        let mut s:f32 = 0.;
        for &x_ in self.pc.iter() {
            s += (x_ as f32 - t).abs();
        }
        self.analysis = round(s as f64,5) as f32 / self.max_diff();

    }

    pub fn t(&mut self) -> f32 {
        round((self.v.len() as f32 / self.p as f32) as f64,5) as f32
    }

    pub fn max_diff(&mut self) -> f32 {
        let t = self.t();
        // 0 all except one
        let x1 = (self.p - 1) as f32 * t;

        // all for one
        let x2 = self.v.len() as f32 - t;
        round((x1 + x2) as f64,5) as f32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__FrqncCl__count_pc() {
        let q: Vec<f32> = vec![4.5,5.2,6.3,13.];
        let r:(f32,f32) = (0.,20.);
        let s:usize = 5;

        let mut c = build_FrqncCl(q,r,s);
        c.count_pc();
        assert_eq!(vec![0, 3, 0, 1, 0],c.pc);
    }

    #[test]
    pub fn test__FrqncCl__analyze() {
        let q: Vec<f32> = vec![4.5,5.2,6.3,13.];
        let q2:Vec<f32> = vec![4.5,4.5,4.5,4.5];
        let q3:Vec<f32> = vec![1.,5.,9.,13.,18.];

        let r:(f32,f32) = (0.,20.);
        let s:usize = 5;

        // case 1
        let mut c = build_FrqncCl(q,r,s);
        c.count_pc();
        c.analyze();
        assert_eq!(c.analysis,0.75);

        // case 2
        c = build_FrqncCl(q2,r,s);
        c.count_pc();
        c.analyze();
        assert_eq!(c.analysis,1.);

        // case 3
        c = build_FrqncCl(q3,r,s);
        c.count_pc();
        c.analyze();
        assert_eq!(c.analysis,0.);
    }

}
