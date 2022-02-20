/*
file contains functions for factors.
*/
use crate::setti::strng_srt;
use crate::setti::setf;
use crate::setti::setf::Count;

use ndarray::{Array,Array1,arr1};
use std::collections::HashSet;
use std::hash::Hash;
use std::collections::HashMap;
use std::str::FromStr;


///////////////////////////////// start: factors

pub fn factors_of_usize(v:usize) -> HashSet<usize> {
    let mut cap = v / 2;
    let mut sol: HashSet<usize> = HashSet::new();
    for i in 1..cap + 1 {
        if v % i == 0 {
            if sol.contains(&i) {
                break;
            } else {
                sol.insert(i);
                sol.insert(v / i);
            }
        }
    }
    sol
}


pub fn factors_of_i32(v:i32) -> HashSet<i32> {

    if v >= 0 {
        let sol:HashSet<i32> = factors_of_usize(v as usize).into_iter().map(|x| x as i32).collect();
        return sol;
    }

    let v_:usize = (v * -1) as usize;
    let sol_:HashSet<usize> = factors_of_usize(v_);//.into_iter().map(|x| x as i32).collect();
    let mut sol:HashSet<i32> = HashSet::new();

    for s in sol_.iter() {
        let s_:i32 = *s as i32;
        sol.insert(s_);
        sol.insert(s_ * -1);
    }
    sol
}


/*
*/
pub fn factors_for_vec(v1:Vec<i32>) -> Vec<HashSet<i32>> {
    let mut sol: Vec<HashSet<i32>> = Vec::new();
    for v in v1.iter() {
        sol.push(factors_of_i32(*v));
    }
    sol
}

pub fn is_factor_for_vec(v1:Vec<i32>,f:i32) -> bool {
    for v_ in v1.iter() {
        if *v_ % f != 0 {
            return false;
        }
    }
    true
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

///////////////////////////////// end: factors

//////////////////////////////// start: gcf

pub fn gcf_for_vec(v1:Vec<i32>) -> i32 {
    assert!(v1.len() > 0);

    if v1.len() == 1 {
        let mut f: Vec<i32> = factors_of_i32(v1[0]).into_iter().collect();
        f.sort();
        return f[f.len() - 1].clone();
    }

    // get min of pairwise differences
    let mut v2:Vec<i32> = v1.clone();
    v2.sort();

    let mut minDiff:i32 = v2[v2.len() -1];
    for i in 1..v2.len() {
        let mut d = v2[i] - v2[i - 1];
        if d < minDiff {
            minDiff = d;
        }
    }

    for i in (2..minDiff + 1).rev() {
        if is_factor_for_vec(v2.clone(),i) {
            return i;
        }
    }

    1
}

/*
*/
pub fn gcf_add4mult_vec(v1:Array1<i32>,v2:Array1<i32>) -> Array1<i32> {
    let mut v2_:Vec<i32> = v2.into_iter().collect();
    let m = gcf_for_vec(v2_);
    -1 * (v1 - m)
}

//////////////////////////////// end: gcf


////////////////////////////////////// start: cheapest

pub fn cheapest_multiple_vec(v1:Array1<i32>,v2:Array1<i32>) ->Array1<i32> {
    let v1x:Array1<f32> = v1.into_iter().map(|x| x as f32).collect();
    let v2x:Array1<f32> = v2.into_iter().map(|x| x as f32).collect();
    let v3x:Array1<f32> = arr1_safe_divide(v2x,v1x,0.0);
    let v3x:Array1<i32> = v3x.into_iter().map(|x| x.round() as i32).collect();
    v3x
}


pub fn cheapest_multiple(v1:Array1<i32>,v2:Array1<i32>) -> i32 {
    let mut f1:f32 = v1.sum() as f32;
    let mut f2:f32 = v2.sum() as f32;
    (f2 / f1).round() as i32
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


////////////////////////////////////// end: cheapest

///////////////////////////// start: closest to

pub fn median_of_iterable<T>(v:Vec<T>) -> (T,Option<T>)
where
T: Clone
 {
    if v.len() % 2 == 1 {
        return (v[v.len() / 2].clone(),None);
    } else {
        return (v[v.len() / 2 - 1].clone(), Some(v[v.len() / 2].clone()));
    }
}

/*
pops them out
*/
pub fn median_of_iterable_<T>(v:Vec<T>) -> (usize,Option<usize>)
where
T: Clone
{

    if v.len() % 2 == 1 {
        return (v.len() / 2, None);
    } else {
        return (v.len() / 2 - 1, Some(v.len() / 2));
    }
}

pub fn sort_by_distance_to_median<T>(v:Vec<T>) -> Vec<T>
where
T: Clone
{
    let mut sol:Vec<T> = Vec::new();
    let mut v2:Vec<T> = v.clone();
    let mut stat: bool = v2.len() > 0;
    while stat {
        let (x1,x2):(usize,Option<usize>) = median_of_iterable_(v2.clone());
        let mut v3:Vec<T> = v2[0..x1].to_vec();
        sol.push(v2[x1].clone());
        let l = v2.len();
        if x2.is_none() {
            v3.extend_from_slice(&mut v2[x1 + 1..l]);//collect());
        } else {
            v3.extend_from_slice(&mut v2[x2.unwrap() + 1..l]);
            sol.push(v2[x2.unwrap()].clone());
        }
        v2 = v3;
        stat = v2.len() > 0;
    }

    sol
}

pub fn neg_double_vec(v:Vec<usize>) -> Vec<i32> {
    let mut v2: Vec<i32> = Vec::new();

    for v_ in v.into_iter() {
        v2.push(v_ as i32);
        v2.push(v_ as i32 * -1);
    }
    v2
}

/*
*/
pub fn ranked_mult_additives_for_i32(v:i32,v2:i32) -> Vec<i32> {
    let mut fv: Vec<usize> = factors_of_usize(v as usize).into_iter().collect();
    let mut fv_ = neg_double_vec(fv);
    let mut fv2 = sort_by_distance_to_median(fv_);
    let fv3:Vec<i32> = fv2.into_iter().map(|x| x - v2).collect();
    fv3
}

/*
Outputs a vector v2 in which for each i'th element e in v2,
e is the vector of additives for the factors M of the i'th element of v2,
and its ordering of the additives correspond to that of the distance to
the median of M.
*/
pub fn ranked_mult_additive_for_vec(v:Array1<i32>,v2:Array1<i32>) -> Vec<Vec<i32>> {
    let mut sol: Vec<Vec<i32>> = Vec::new();
    for (i,v_) in v.into_iter().enumerate() {
        sol.push(ranked_mult_additives_for_i32(v_,v2[i]));
    }
    sol
}

/*
locates the i32 add for v to satisfy l members of
v as factors of v2's corresponding members, l the
maximum possible number of members can be satisfied
by adding an i32.
*/
pub fn max_satisfying_mult_additive_for_vec(v:Array1<i32>,v2:Array1<i32>) -> i32 {
    let b: Vec<Vec<i32>> = ranked_mult_additive_for_vec(v,v2);
    let mut vc: setf::VectorCounter = setf::VectorCounter{data: HashMap::new()};

    for b_ in b.into_iter() {
        vc.countv(b_);
    }
    let mut d2:Vec<(String,i32)> = strng_srt::hashmap_to_2vector(vc.data.clone());
    d2.sort_by(strng_srt::str_cmp5);
    i32::from_str(d2[d2.len() - 1].0.as_str()).unwrap()
}

pub fn closest_i32_to_mean(v:Array1<i32>) -> i32 {
    let mut m:i32 = v.clone().into_iter().map(|x| x as f32).collect::<Array1<f32>>().mean().unwrap().round() as i32;
    let mut diff:Array1<usize> = v.clone().into_iter().map(|x| (x - m).abs() as usize).collect();
    let mn = diff.iter().fold(0, |min, &val| if val < min{ val } else{ min });
    let index = diff.iter().position(|&r| r == mn).unwrap();
    v[index]
}

pub fn closest_i32_to_median(v:Array1<i32>) -> i32 {
    let mut v_:Vec<i32> = v.clone().into_iter().collect();
    v_.sort();
    let (s1,_): (i32,Option<i32>) = median_of_iterable(v_);
    let mut diff:Array1<usize> = v.clone().into_iter().map(|x| (x - s1).abs() as usize).collect();
    let mn = diff.iter().fold(0, |min, &val| if val < min{ val } else{ min });

    let index = diff.iter().position(|&r| r == mn).unwrap();
    v[index]
}

///////////////////////////// end: closest to

/*
*/
pub fn intersection_set_for_hashsetvec<T>(v:Vec<HashSet<T>>) ->HashSet<T>
where
T:  Hash + Clone + Eq {
    assert!(v.len() > 0);

    if v.len() == 1 {
        return v[0].clone();
    }

    let mut sol:HashSet<T> = v[1].intersection(&v[0]).map(|x| (*x).clone()).collect();
    let l = v.len();
    for i in 2..l {
        sol = sol.intersection(&v[i]).map(|x| (*x).clone()).collect();
    }

    sol
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

//////////////////////////// start: test samples

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

pub fn sample_arr1_pair_3() -> (Array1<i32>,Array1<i32>) {
    let v1:Array1<i32> = arr1(&[4,2]);
    let v2:Array1<i32> = arr1(&[8,10]);
    (v1,v2)
}

//////////////////////////// end: test samples


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
    fn test_factors_for_vec() {

        let sol:Vec<String> = vec!["1_10_2_20_4_5".to_string(),
        "1_10_15_2_3_30_5_6".to_string(),
        "1_10_2_20_4_40_5_8".to_string(),
        "1_15_25_3_5_75".to_string(),
        "1_11_2_22_4_44_8_88".to_string()];

        let mut x:Vec<i32> = vec![20,30,40,75,88];
        let mut x2:Vec<HashSet<i32>> = factors_for_vec(x.clone());
        for (i,x_) in x2.iter().enumerate() {
            let mut x2_:HashSet<String> = x_.into_iter().map(|x| x.to_string()).collect();
            assert_eq!(strng_srt::stringized_srted_hash(x2_),sol[i]);
        }
    }

    #[test]
    fn test_intersection_set_for_hashsetvec() {
        let mut x:Vec<i32> = vec![20,30,40,75,88];
        let mut x2:Vec<HashSet<i32>> = factors_for_vec(x.clone());
        let mut q: HashSet<i32> = intersection_set_for_hashsetvec(x2);
        let mut y:String = strng_srt::stringized_srted_vec(&mut q.into_iter().map(|x| x.to_string()).collect());
        assert_eq!("1".to_string(),y);
    }

    #[test]
    fn test_gcf_for_vec() {
        let mut x:Vec<i32> = vec![20,30,40,75,88];
        let mut y:i32 = gcf_for_vec(x.clone());
        assert_eq!(1,y);

        let mut x2:Vec<i32> = vec![8,16,40,36,88];
        let mut y2:i32 = gcf_for_vec(x2.clone());
        assert_eq!(4,y2);
    }

    #[test]
    fn test_factors_of_usize() {
        let mut q:HashSet<usize> = factors_of_usize(30);
        let mut q2:HashSet<String> = q.iter().map(|y| y.to_string()).collect();
        let s = strng_srt::stringized_srted_hash(q2);
        let sol1 = "1_10_15_2_3_30_5_6".to_string();
        assert_eq!(sol1,s);

        q = factors_of_usize(60);
        q2 = q.iter().map(|y| y.to_string()).collect();
        let s2 = strng_srt::stringized_srted_hash(q2);
        let sol2 = "1_10_12_15_2_20_3_30_4_5_6_60".to_string();
        assert_eq!(sol2,s2);
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

    #[test]
    fn test_closest_i32_to_mean() {
        let a:Array1<i32> = arr1(&[4,1,4,3,2]);
        let x:i32 = closest_i32_to_mean(a);
        assert_eq!(x,3);
    }

    #[test]
    fn test_closest_i32_to_median() {
        let a:Array1<i32> = arr1(&[-2,-2,1,1,1,4,4,4,4,5,5,6,6]);
        let x:i32 = closest_i32_to_median(a);
        assert_eq!(x,4);
    }

    #[test]
    fn test_sort_by_distance_to_median() {
        let mut v: Vec<usize> = vec![0,0,0,0,2,2,2,3,3,4,4,4,4,4,5,5,5,5,5,5,5];
        let mut vm: Vec<usize> = sort_by_distance_to_median(v);
        assert_eq!(vm,vec![4, 4, 4, 3, 4, 3, 4, 2, 5, 2, 5, 2, 5, 0, 5, 0, 5, 0, 5, 0, 5]);
    }

    #[test]
    fn test_ranked_mult_additive_for_vec() {
        let (a,a2):(Array1<i32>,Array1<i32>) = sample_arr1_pair_3();
        let b:Vec<Vec<i32>> = ranked_mult_additive_for_vec(a.clone(),a2.clone());
        let mut vc: setf::VectorCounter = setf::VectorCounter{data: HashMap::new()};
        for b_ in b.into_iter() {
            vc.countv(b_);
        }

        let m: HashMap<String, i32> = HashMap::from_iter([("-9".to_string(), 2), ("-6".to_string(), 1),
            ("-11".to_string(), 1), ("-10".to_string(),1), ("-7".to_string(),1), ("-4".to_string(),1),
            ("-8".to_string(),1), ("-12".to_string(),2)]);
        assert_eq!(vc.data,m);
    }

    #[test]
    fn test_max_satisfying_mult_additive_for_vec() {
        let (a,a2):(Array1<i32>,Array1<i32>) = sample_arr1_pair_3();
        let i:i32 = max_satisfying_mult_additive_for_vec(a.clone(),a2.clone());
        assert!(i == -12 || i == -9);
    }

}
