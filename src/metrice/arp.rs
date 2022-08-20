//! range partition for sequences of unlabelled real numbers
use crate::setti::fs;
use crate::metrice::brp;
use ndarray::{Array1,arr1};
use crate::metrice::bmeas;
use crate::enci::mat2sort;

/// # description
/// converts an <arr1<f32>> into a scaled <arr1<f32>> (values in \[0.,1.\])
pub fn arr1_f32_to_arr1_01(f:Array1<f32>) -> Array1<f32> {
    let c = mat2sort::sort_arr1(f.clone(),mat2sort::f32_cmp1);
    let l = c.len();
    let d = c[l - 1] - c[0];
    if d == 0. {
        return Array1::zeros(l);
    }

    let mut sol: Vec<f32> = Vec::new();
    for f_ in f.into_iter() {
        sol.push((f_.clone() - c[0]) / d);
    }
    sol.into_iter().collect()
}

/// # description
/// converts a scaled <arr1<f32>> (values in \[0.,1.\]) into an
/// <arr1<f32>> based on the bounds `b`. 
pub fn arr1_01_to_arr1_f32(f:Array1<f32>, b:(f32,f32)) -> Array1<f32> {
    assert!(bmeas::is_proper_bounds(b.clone()));
    let mut sol: Vec<f32> = Vec::new();

    for f_ in f.into_iter() {
        sol.push(b.0 + f_ * (b.1 - b.0));
    }

    sol.into_iter().collect()
}

pub fn arr1_minmax(f:Array1<f32>) -> (f32,f32) {
    let ans:f32 = f[0].clone();
    let minumum = f.clone().into_iter().fold(ans.clone(),|acc,s| if s > acc {acc} else {s});
    let maximum = f.clone().into_iter().fold(ans.clone(),|acc,s| if s < acc {acc} else {s});
    (minumum,maximum)
}

/// struct labels a sequence of values similar to <brp::RangePartitionGF2>
/// difference w/ RangePartitionGF2 rests on auto-labelling
///
/// struct solution search process is essentially k-means on 1 dimension.
/// for each FSelect to include an arbitrary f32, uses choices calculated from
/// RangePartitionGF2 and modifies their score.
/// RangePartitionGF2 is used by mapping its \[0,1\] space to ArbitraryRangePartition
/// as solution.
pub struct ArbitraryRangePartition {
    /// vec over real numbers
    f32_vec: Array1<f32>,
    /// bounds for `f32_vec` values
    xstream: (f32,f32),
    /// used to collect partitions over\[0,1\] range
    rpgf2: brp::RangePartitionGF2,
    /// solution
    pub fselect:Option<fs::FSelect>, // used to keep track of distance score: mean and frequency for each range
    /// maximum number of partitions allowed
    t:usize,
    /// used for determining best <fs::FSelect> solution
    cache:Vec<(fs::FSelect,f32)> // FSelect, arp score
}

pub fn build_ArbitraryRangePartition(f32_vec: Array1<f32>,szt:usize) -> ArbitraryRangePartition {
    // default binary label is all 0
    let bl:Array1<usize> = Array1::zeros(f32_vec.len());

    // get 01 version
    let xstream = arr1_minmax(f32_vec.clone());
    let tmp_f32_vec: Array1<f32> = arr1_f32_to_arr1_01(f32_vec.clone());
    let rs = brp::build_range_partition_gf2(tmp_f32_vec.clone(),bl,szt,"fm".to_string());

    // fselect:
    ArbitraryRangePartition{f32_vec:f32_vec,xstream:xstream,rpgf2:rs,fselect:None,t:szt,cache:Vec::new()}
}

impl ArbitraryRangePartition {

    pub fn brute_force_search__decision(&mut self) {
        let f_ = self.rpgf2.brute_force_search__decision();
        if f_.is_none() {
            println!("ERROR: no search resut for ARP");
            return;
        }
        let mut f = f_.unwrap();
        self.mod_fselect_fm(&mut f);
        self.fselect = Some(f.clone());
    }

    pub fn mod_fselect_fm(&mut self, f: &mut fs::FSelect) {
        // scale vec(bounds)
        let data = f.data.clone();
        let amm = arr1_minmax(self.f32_vec.clone());
        let bv = bmeas::bvec_01_to_bvec_f32(data,self.xstream.clone());//amm.clone());
        f.data = bv;

        // scale fm meens
        let mut fm = f.fm.clone().unwrap();
        let l = fm.len();

        for i in 0..l {
            fm[i].meen = self.xstream.0 + fm[i].meen * (amm.1 - amm.0)
        }
        f.fm = Some(fm);

        f.data_labels = (0..f.data.len()).into_iter().collect();
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__ArbitraryRangePartition__brute_force_search__decision() {

        let f1: Array1<f32> = arr1(&[40.0,20.,10.0,-34.0,60.]);

        let szt:usize = 2;
        let mut arp1 = build_ArbitraryRangePartition(f1,szt);
        arp1.brute_force_search__decision();
        let d2 = arp1.fselect.clone().unwrap().data;
        assert_eq!(d2,vec![(40.0, 60.0), (-34.0, 20.0)]);

        let sol1: Vec<(f32,usize)> = vec![(50.,2),(-17.666666,3)];
        let q = arp1.fselect.clone().unwrap().fm.unwrap();

        for (i,f) in q.into_iter().enumerate() {
            assert_eq!(f.meen,sol1[i].0);
            assert_eq!(f.frequency,sol1[i].1);
        }
    }
}
