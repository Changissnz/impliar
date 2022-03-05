/*
some matrix functions
*/
use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;
use ndarray::{Array1,Array2, array,arr1,arr2,s};

pub fn replace_vec_in_arr2<T>(a:&mut Array2<T>,q:&mut Array1<T>,i:usize,is_row:bool)
where
T:Clone
 {
    if is_row {
        assert!(i < a.raw_dim()[0]);
        let mut b = a.slice_mut(s![i, ..]);
        b.assign(&q);
    } else {
        assert!(i < a.raw_dim()[1]);
        let mut b = a.slice_mut(s![.., i]);
        b.assign(&q);
    }
}

pub fn exist_any_in_vec_of_arr2<T>(a:&mut Array2<T>,b:HashSet<T>,i:usize,is_row:bool) -> bool
where
T:Clone + Eq + Hash
 {
    let c = if is_row {a.slice_mut(s![i,..])} else {a.slice_mut(s![..,i])};
    let f2:Array1<_> = c.into_iter().filter(|x| b.contains(x)).collect();
    f2.raw_dim()[0] > 0
}

pub fn anyat_index_arr1<T>(a:&mut Array1<T>,b:HashSet<T>) -> Array1<usize>
where
T:Clone + Eq + Hash
 {
    a.into_iter().map(|x| {if b.contains(x) {1} else {0}}).collect()
}


/*
return:
- vector of indices
*/
pub fn anyat_vec_in_vec_of_arr2<T>(a:&mut Array2<T>,b:HashSet<T>,i:usize,is_row:bool) -> Array1<usize>
where
T:Clone + Eq + Hash
 {
    let mut c = if is_row {a.slice_mut(s![i,..])} else {a.slice_mut(s![..,i])};
    //let f2:Array1<_> = c.into_iter().enumerate().map(|(i,x)| {if b.contains(x) {1} else {0}}).collect();
    let f2:Array1<_> = anyat_index_arr1(&mut c.to_owned(),b).to_owned();//c.into_iter().map(|x| {if b.contains(x) {1} else {0}}).collect();
    f2
}

pub fn anyat_arr2<T>(a:&mut Array2<T>,b:HashSet<T>) -> Array2<usize>
where
T:Clone + Eq + Hash
 {
    let x1 = a.raw_dim()[0];
    let x2 = a.raw_dim()[1];
    let mut q = Array2::zeros((x1,x2));

    for i in 0..x1 {
        let mut y = anyat_vec_in_vec_of_arr2(a,b.clone(),i,true);
        replace_vec_in_arr2(&mut q,&mut y,i,true);
    }

    q
}

/*
*/
pub fn replace_subarr2<T>(a:&mut Array2<T>,b:&mut Array2<T>,start_dim:(usize,usize))
where
    T:Clone
{
    let  i = start_dim.0;
    let  ei = start_dim.0 + b.raw_dim()[0];
    let  j = start_dim.1;
    let  ej = start_dim.1 + b.raw_dim()[1];

    assert!(a.raw_dim()[0] > start_dim.0 && a.raw_dim()[1] > start_dim.1);
    assert!(ei <= a.raw_dim()[0]);
    assert!(ej <= a.raw_dim()[1]);

    let mut x = 0;
    for i_ in i..ei {
        let mut c = a.slice_mut(s![i_,j..ej]);
        let c2 = b.slice_mut(s![x,..]);
        c.assign(&c2);
        x += 1;
    }
}

/*
can be used to modify vector
*/
pub fn map_function_on_subvector<T>(v: &mut Vec<T>,f: fn(T) -> T ,indices:Vec<usize>,replace:bool) -> Vec<T>
where
    T:Clone
 {
    let sol: Vec<T> = indices.iter().map(|i| f(v[*i].clone())).collect();
    if replace {
        for i in 0..indices.len() {
            v[indices[i]] = sol[i].clone();
        }
    }
    sol
}

/////////////////////////////////////////////////////////////////////////

pub fn test_d() -> Array2<i32> {
    let mut x1:Array2<i32> = Array2::zeros((5,4));
    let mut m1: Array1<i32> = arr1(&[1,43,56,76]);
    let mut m2:Array1<i32> = arr1(&[100,4,516,176,3]);
    replace_vec_in_arr2(&mut x1,&mut m1,0,true);
    replace_vec_in_arr2(&mut x1,&mut m2,2,false);
    x1
}

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
        let mut x1 = test_d();
        assert_eq!(x1,arr2(&[[1, 43, 100, 76],
         [0, 0, 4, 0],
         [0, 0, 516, 0],
         [0, 0, 176, 0],
         [0, 0, 3, 0]]));

         let mut b:HashSet<i32> = HashSet::new();
         b.insert(1);
         b.insert(4);

         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),0,false));
         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),0,true));
         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),1,true));
         assert!(!exist_any_in_vec_of_arr2(&mut x1,b.clone(),1,false));
         assert!(exist_any_in_vec_of_arr2(&mut x1,b.clone(),2,false));
    }


    #[test]
    fn test_anyat_vec_in_vec_of_arr2() {
        let mut x1 = test_d();
        let mut b:HashSet<i32> = HashSet::new();
        b.insert(1);
        b.insert(4);
        let mut sol1 = anyat_vec_in_vec_of_arr2(&mut x1,b.clone(),0,false);
        let mut sol2 = anyat_vec_in_vec_of_arr2(&mut x1,b.clone(),1,false);

        let mut m3:Array1<i32> = arr1(&[1,4,516,1,4]);
        replace_vec_in_arr2(&mut x1,&mut m3,3,false);
        let mut sol3 = anyat_vec_in_vec_of_arr2(&mut x1,b.clone(),3,false);

        let sol1_:Array1<usize> = array![1, 0, 0, 0, 0];
        let sol2_:Array1<usize> = array![0, 0, 0, 0, 0];
        let sol3_:Array1<usize> = array![1, 1, 0, 1, 1];
        assert!(sol1 == sol1_);
        assert!(sol2 == sol2_);
        assert!(sol3 == sol3_);
    }

    #[test]
    fn test_anyat_arr2() {
        let mut x1 = test_d();

        let mut b:HashSet<i32> = HashSet::new();
        b.insert(1);
        b.insert(4);
        b.insert(43);
        b.insert(100);
        b.insert(516);

        let mut c = anyat_arr2(&mut x1,b.clone());
        let mut d = arr2(&[[1, 1, 1, 0],
                        [0, 0, 1, 0],
                        [0, 0, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0]]);

        assert_eq!(c,d);
    }

}
