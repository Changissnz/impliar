//! functions for \<arr1\> and \<arr2\> sort
use crate::setti::matrixf;
use ndarray::{Array,Array1,Array2,arr1,arr2};
use ndarray::s;
use std::collections::HashMap;
use std::cmp::Ordering;
use ndarray::Dim;
use std::collections::HashSet;

/// # return 
/// shuffled `a` by index vector `s`
pub fn apply_shuffle_map_arr1<T>(a: Array1<T>,s:Array1<usize>) -> Array1<T>
where
T: Clone
 {
    assert_eq!(a.len(),s.len());
    let mut sol:Vec<T> = Vec::new();

    let l = a.len();

    for i in 0..l {
        sol[i] = a[s[i]].clone();
    }
    arr1(&sol)
}

/// # return 
/// row-wise or column-wise shuffled `a` by index vector `s`
pub fn apply_shuffle_map_arr2<T>(a: Array2<T>,s:Array1<usize>,is_row:bool) -> Array2<T>
where
T: Clone + Default
 {
    let mut sol : Vec<Array1<T>> = Vec::new();
    for (i,s_) in s.into_iter().enumerate() {
        let q:Array1<T> = if is_row {a.row(s_).to_owned()} else {a.column(s_).to_owned()};
        sol.push(q);
    }
    vec_to_arr2(sol).unwrap()
}

/// # return
/// sequence of indices of each i'th row of `a` in `a2` 
pub fn arr2_shuffle_map<T>(a:Array2<T>,a2:Array2<T>) -> Array1<usize>
where
T:Eq + Clone
{

    assert_eq!(a.len(),a2.len());

    let mut sol:Array1<usize> = Array1::zeros(a.dim().0);
    let l = a.dim().0;
    for i in 0..l {
        let r1:Array1<T> = a.row(i).to_owned();
        sol[i] = vec_in_arr2(a2.clone(),r1,true).unwrap();
    }
    sol
}

/// # return 
/// index of `a2` in `a` or None
pub fn vec_in_arr2<T>(a:Array2<T>,a2: Array1<T>,is_row:bool) -> Option<usize>
where
T:Eq + Clone
 {
    let l = if is_row {a.dim().0} else {a.dim().1};

    assert_eq!(l,a2.len());

    for i in 0..l {
        if is_row {
            if a.row(i).to_owned() == a2.clone() {
                return Some(i);
            }
        } else {
            if a.column(i).to_owned() == a2.clone() {
                return Some(i);
            }
        }
    }

    None
}

/// # description
/// f32 comparator function `1`; used for sorting
pub fn f32_cmp1(s1: &f32, s2: &f32) -> std::cmp::Ordering {

    if s1 < s2 {
    	return Ordering::Less;
    }
    Ordering::Greater
}

/// # description
/// sorts array1<f32> `a` by comparator function `f` 
pub fn sort_arr1(a: Array1<f32>,f: fn(&f32,&f32) -> std::cmp::Ordering) -> Array1<f32> {
    let mut v: Vec<f32> = a.into_iter().collect();
    v.sort_by(f);
    v.into_iter().collect()
}

/// # description
/// sorts array2<f32> `a` row-wise by comparator function `f`
/// 
/// # return
/// does not perform tie-breakers for comparator function `f`
pub fn sort_arr2(mut a:Array2<f32>,f: fn(&Array1<f32>,&Array1<f32>) -> std::cmp::Ordering) -> Array2<f32> {//f: fn(&Array1<f32>,&Array1<f32>) -> std::cmp::Ordering) -> Array2<f32> {

    let mut a2: Vec<Array1<f32>> = Vec::new();
    for x in a.rows_mut() {
        a2.push(x.to_owned());
    }

    a2.sort_by(f);
    let mut a3:Array2<f32> = Array2::zeros((a2.len(),a.dim().1));

    for (i,ax) in a2.iter().enumerate() {
        matrixf::replace_vec_in_arr2(&mut a3,&mut ax.clone(),i,true);
    }
    a3
}

/// # description
/// Similar to above method, except for cases in which elements result in equal
/// values, uses probability weights `pr` to determine ordering. A higher probability
/// weight results in higher priority.
///
/// # return 
/// (sorted `a`,probability vector)
pub fn sort_arr2_tie_breakers(a:Array2<f32>,ignore_col: Option<HashSet<usize>>, pr:Array1<f32>,
        f: fn(Array1<f32>) -> usize) -> (Array2<f32>,Array1<f32>) {

    let a_: Array2<f32> = a.clone();
    let mut sol:Vec<Array1<f32>> = Vec::new();
    let l = a.dim().0;
    let mut prx : Vec<f32> = Vec::new();
    for i in 0..l {
        let q = a.row(i).to_owned();
        let pr_ = pr[i];
        let prx2 = arr1(&prx);
        let j = sort_insert_in_vec_tie_breakers(&mut sol,q, ignore_col.clone(),prx2,pr_,f);
        prx.insert(j,pr_);
    }

    (vec_to_arr2(sol).unwrap(), Array1::from_vec(prx))
}

/// # return
/// converted vector of <arr1\<T\>> into <arr2\<T\>> or None 
pub fn vec_to_arr2<T>(v: Vec<Array1<T>>) -> Option<Array2<T>>
where
T: Clone + Default
 {
    if v.len() == 0 {
        return None;
    }
    let c = v[0].len();
    let mut sol: Array2<T> = Array2::default((v.len(),c));

    for (i,v_) in v.iter().enumerate() {
        matrixf::replace_vec_in_arr2(&mut sol, &mut (*v_).clone(),i,true);
    }

    Some(sol)
}

/// # description
/// helper method for `sort_arr2_tie_breakers`. If `ignore_col` is not None, `f` scores `a` according to
/// its indices not in `ignored_col`. Determines the minimal index `i` in `v` in which `f(a) <= f(v[i])`.
///             if f(a) == f(v\[i\]) and pra < pr\[i\], insert.
pub fn sort_insert_in_vec_tie_breakers(v: &mut Vec<Array1<f32>>,a:Array1<f32>,ignore_col: Option<HashSet<usize>>, pr:Array1<f32>,
    pra:f32,f: fn(Array1<f32>) -> usize) -> usize {

    assert_eq!((*v).len(),pr.len());
    if pr.len() > 0 {
        assert_eq!(a.len(),(*v)[0].len());
    }
    let l = (*v).len();
    let mut j:usize = 0;
    let q:usize = if ignore_col.is_none() {f(a.clone())} else 
            {f(a.clone().into_iter().enumerate().filter(|(i,x)| !ignore_col.clone().unwrap().contains(&i)).map(|(i,x)| x).collect() )};
    while j < l {
        let q2:usize = if ignore_col.is_none() {f((*v)[j].clone())} else {f((*v)[j].clone().into_iter().enumerate().filter(|(i,x)| !ignore_col.clone().unwrap().contains(&i)).map(|(i,x)| x).collect() )};
        if q < q2 {
            (*v).insert(j,a);
            return j;
        } else if q == q2 {
            if pra < pr[j] {
                (*v).insert(j,a.clone());
                return j;
            }
        } else {

        }
        j += 1;
    }

    (*v).insert(j,a);
    j
}

/// # return
/// number of non-zero elements of `v`
pub fn active_size_of_vec(v: Array1<f32>) -> usize {
    let v2:Array1<f32> = v.into_iter().filter(|x| *x != 0.0).collect();
    v2.len()
}

/// # return
/// non-zero indices of `v`
pub fn active_indices(v:Array1<f32>) -> Array1<usize> {
    let dummy:Array1<f32> = v.clone();
    active_size_intersection(v,dummy)
}

/// # return 
/// difference of active size of v1 and v2
pub fn active_size_distance(v:Array1<f32>,v2:Array1<f32>) -> usize {
    (active_size_of_vec(v) as i32 - active_size_of_vec(v2) as i32).abs() as usize
}

/// # return
/// non-zero indices shared by `v` and `v2`
pub fn active_size_intersection(v: Array1<f32>, v2:Array1<f32>) -> Array1<usize> {
    let v3 = v * v2;
    v3.into_iter().enumerate().filter(|(i,x)| *x != 0.0).map(|(i,x)| i).collect()
}

/// # return
/// both `x1` and `x2` positive? 
pub fn is_positive_intersection(x1:f32,x2:f32) -> bool {
    if x1 > 0.0 && x2 > 0.0 {true} else {false}
}

/// # description
/// generic index-wise intersection function between `v1` and `v2`. 
/// # return 
/// indices of `v1`
pub fn arr1_intersection_indices(v1:Array1<f32>,v2:Array1<f32>,f: fn(f32,f32) ->bool) -> Array1<usize> {
    let l = if v1.len() < v2.len() {v1.len()} else {v2.len()};
    let mut sol: Vec<usize> = Vec::new();

    for i in 0..l {
        if f(v1[i].clone(),v2[i].clone()) {
            sol.push(i);
        }
    }
    Array1::from_vec(sol)
}

/// # description 
/// non-commutative function f by reference v1 on arg. v2.
pub fn intersection_difference_measure(v1:Array1<f32>,v2:Array1<f32>) -> i32 {
    let indices = arr1_intersection_indices(v1.clone(),v2.clone(),is_positive_intersection);
    let l = indices.len() as i32;
    (v2 - v1).into_iter().sum::<f32>() as i32 - l
}

/// # description
/// indices of `v` and `v2` that are equal 
pub fn arr1_intersection(v:Array1<f32>,v2:Array1<f32>) -> Vec<usize> {
    let v3 = v - v2;
    let v4: Vec<(usize,f32)> = v3.into_iter().enumerate().filter(|(i,x)| *x != 0.0).collect();
    v4.into_iter().map(|(x,x2)| x).collect()
}

/// # description 
/// <arr1\<f32\>> comparator function by active size
pub fn arr1_cmp1(v:&Array1<f32>,v2:&Array1<f32>) -> std::cmp::Ordering {
    if active_size_of_vec((*v).clone()) <= active_size_of_vec((*v2).clone()) {
        return Ordering::Less;
    }
    Ordering::Greater
}

/// # return
/// absolute sum of values of <arr1\<f32\>> 
pub fn abs_sum_arr1_f32(v:Array1<f32>) -> f32 {
    let v2:Array1<f32> = v.into_iter().map(|x| x.abs()).collect();
    v2.sum()
}

pub fn sample_arr2_sort1() -> Array2<f32> {
    arr2(&[[0.,1.,1.,1.,1.],
        [1.,0.,1.,0.,0.],
        [1.,1.,1.,0.,0.],
        [0.,0.,1.,0.,1.],
        [0.,0.,0.,1.,0.],
        [0.,1.,0.,0.,0.]])
}

pub fn sample_arr2_sort2() -> Array2<f32> {
    arr2(&[[1.,0.,0.,0.,0.],
        [0.,1.,0.,0.,0.],
        [0.,0.,1.,0.,0.],
        [0.,0.,0.,1.,0.],
        [0.,0.,0.,0.,1.],
        [0.,1.,0.,0.,0.]])
}

pub fn sample_pr_sort11() -> Array1<f32> {
    arr1(&[23.4,20.1,14.2,-10.1,100.7,23.3])
}

pub fn sample_pr_sort12() -> Array1<f32> {
    arr1(&[23.4,-10.1,14.2,20.1,23.3,100.7])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arr1_intersection_indices() {
        let mut v1: Array1<f32> = arr1(&[1.0,1.0,0.,0.,1.0]);
        let mut v2: Array1<f32> = arr1(&[0.0,1.0,1.0,0.,1.0]);

        let q = arr1_intersection_indices(v1,v2,is_positive_intersection);
        assert_eq!(q,arr1(&[1,4]));
    }

    #[test]
    fn test_sort_arr2_AND_sort_arr2_tie_breakers() {
        // case 1
        let mut x2 = sample_arr2_sort1();
        let x3 = sort_arr2(x2.clone(),arr1_cmp1);

        // case 2
        let mut pr = sample_pr_sort11();
        let x4 = sort_arr2_tie_breakers(x2.clone(),None,pr,active_size_of_vec).0;

        let sol1:Array2<f32> = arr2(&[[0., 1., 0., 0., 0.],
         [0., 0., 0., 1., 0.],
         [0., 0., 1., 0., 1.],
         [1., 0., 1., 0., 0.],
         [1., 1., 1., 0., 0.],
         [0., 1., 1., 1., 1.]]);

        assert_eq!(sol1.clone(),x4);
        assert_eq!(sol1.clone(),x3);

        // case 3
        let mut pr2 = sample_pr_sort12();
        let x5 = sort_arr2_tie_breakers(x2.clone(),None,pr2,active_size_of_vec).0;
        let sol2:Array2<f32> = arr2(&[[0., 0., 0., 1., 0.],
                            [0., 1., 0., 0., 0.],
                            [1., 0., 1., 0., 0.],
                            [0., 0., 1., 0., 1.],
                            [1., 1., 1., 0., 0.],
                            [0., 1., 1., 1., 1.]]);
        assert_eq!(sol2.clone(),x5);

        // case 4
        let mut a = sample_arr2_sort2();
        pr = sample_pr_sort12();
        let sol3:Array2<f32> = arr2(&[[0., 1., 0., 0., 0.],
                            [0., 0., 1., 0., 0.],
                            [0., 0., 0., 1., 0.],
                            [0., 0., 0., 0., 1.],
                            [1., 0., 0., 0., 0.],
                            [0., 1., 0., 0., 0.]]);
        let x6 = sort_arr2_tie_breakers(a,None,pr,active_size_of_vec).0;
        assert_eq!(sol3.clone(),x6);
    }


}
