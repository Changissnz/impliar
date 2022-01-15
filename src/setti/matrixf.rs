/*
some matrix functions
*/
use std::collections::HashSet;
use ndarray::array;
use std::hash::Hash;
use std::cmp::Eq;

use ndarray::{Array,Array1,Array2, arr1,arr2,arr3, stack,s,Axis,Dim};

pub fn replace_vec_in_arr2<T>(a:&mut Array2<T>,q:&mut Array1<T>,i:usize,isRow:bool)
where
T:Clone
 {
    if isRow {
        assert!(i < a.raw_dim()[0]);
        let mut b = a.slice_mut(s![i, ..]);
        b.assign(&q);
    } else {
        assert!(i < a.raw_dim()[1]);
        let mut b = a.slice_mut(s![.., i]);
        b.assign(&q);
    }
}

pub fn exist_any_in_vec_of_arr2<T>(a:&mut Array2<T>,b:HashSet<T>,i:usize,isRow:bool) -> bool
where
T:Clone + Eq + Hash
 {
    let fx = |x| {b.contains(x)};
    let c = if isRow {a.slice_mut(s![i,..])} else {a.slice_mut(s![..,i])};
    let f2:Array1<_> = c.into_iter().filter(|x| b.contains(x)).collect();
    f2.raw_dim()[0] > 0
}

/*
*/
pub fn replace_subarr2<T>(a:&mut Array2<T>,b:&mut Array2<T>,startDim:(usize,usize))
where
    T:Clone
{
    let mut i = startDim.0;
    let mut ei = startDim.0 + b.raw_dim()[0];
    let mut j = startDim.1;
    let mut ej = startDim.1 + b.raw_dim()[1];

    assert!(a.raw_dim()[0] > startDim.0 && a.raw_dim()[1] > startDim.1);
    assert!(ei <= a.raw_dim()[0]);
    assert!(ej <= a.raw_dim()[1]);

    let mut x = 0;
    for i_ in i..ei {
        let mut c = a.slice_mut(s![i_,j..ej]);
        let mut c2 = b.slice_mut(s![x,..]);
        c.assign(&c2);
    }
}

/*
can be used to modify vector
*/
pub fn map_function_on_subvector<T>(v: &mut Vec<T>,f: fn(T) -> T ,indices:Vec<usize>,replace:bool) -> Vec<T>
where
    T:Clone
 {
    let mut sol: Vec<T> = indices.iter().map(|i| f(v[*i].clone())).collect();
    if replace {
        for i in 0..indices.len() {
            v[indices[i]] = sol[i].clone();
        }
    }
    sol
}

/////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_map_function_on_subvector() {
        let mut axx : Vec<u32> = vec![1,2,43,56,6111];
        let mut modI:Vec<usize> = vec![0,2];
        let mut q = map_function_on_subvector(&mut axx,|x| x + 3 + 3 * x ,modI,true);
        let sol = vec![7,2,175,56,6111];
        assert_eq!(sol,axx);
    }

    #[test]
    fn test_replace_vec_in_arr2() {
        let mut ax1 : Array2<i32> = Array2::zeros((5, 4));
        let mut ax2: Array1<i32> = arr1(&[0,0,1,1]);
        replace_vec_in_arr2(&mut ax1,&mut ax2,0,true);
        let mut aRow = ax1.slice_mut(s![0, ..]);
        assert_eq!(aRow,ax2);
    }

    #[test]
    fn test_exist_any_in_vec_of_arr2() {


        let mut x1:Array2<i32> = Array2::zeros((5,4));
        let mut m1: Array1<i32> = arr1(&[1,43,56,76]);
        let mut m2:Array1<i32> = arr1(&[100,4,516,176,3]);

        replace_vec_in_arr2(&mut x1,&mut m1,0,true);
        replace_vec_in_arr2(&mut x1,&mut m2,2,false);
        /*
        assert_eq!(x1,arr2(&[[1, 43, 100, 76],
         [0, 0, 4, 0],
         [0, 0, 516, 0],
         [0, 0, 176, 0],
         [0, 0, 3, 0]]));
         */
         let mut b:HashSet<i32> = HashSet::new();
         b.insert(1);
         b.insert(4);

         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),0,false));
         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),0,true));
         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),1,true));
         assert!(!exist_any_in_vec_of_arr2(&mut x1,b.clone(),1,false));
         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),2,false));



    }

}
