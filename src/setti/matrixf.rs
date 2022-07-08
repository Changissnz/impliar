/*
some matrix functions
*/
use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;
use ndarray::{Array1,Array2, array,arr1,arr2,s};
use std::default::Default;
use std::collections::HashMap;


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
*/
pub fn subarr2_by_indices<T>(a:Array2<T>,indices:Vec<usize>,is_row:bool) -> Array2<T>
where
T:Clone + Default

 {
    assert!(indices.len() > 0);

    let l = if is_row {a.dim().0} else {a.dim().1};
    assert!(l > *(indices.iter().max().unwrap()));

    let (r,c) = (a.dim().0,a.dim().1);
    let mut a2:Array2<T> = if is_row {Array2::default((indices.len(),c))} else {Array2::default((r,indices.len()))};
    for (j,i) in indices.into_iter().enumerate() {
        let mut x = if is_row {a.row(i).to_owned()} else {a.column(i).to_owned()};
        replace_vec_in_arr2(&mut a2,&mut x,j,is_row);
    }
    a2
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

pub fn label_to_iset_map<T>(v: Vec<T>) -> HashMap<T,Vec<usize>>
where
    T: Clone + Hash + Eq {

    let mut h:HashMap<T,Vec<usize>> = HashMap::new();

    for (i,v_) in v.into_iter().enumerate() {
        if h.contains_key(&v_) {
            let mut l = h.get_mut(&v_).unwrap();
            l.push(i);
            let x:Vec<usize> = l.clone();
            *h.get_mut(&v_).unwrap() = x;
        } else {
            h.insert(v_,vec![i]);
        }
    }
    h
}


pub fn one_index_to_two_index(i:usize,x:usize,y:usize) -> (usize,usize) {
    assert!(i < x * y);

    let oi = i / y;
    let oi2 = i % y;
    (oi,oi2)
}

pub fn two_index_to_one_index(i2:(usize,usize),x:usize,y:usize) -> usize {
    assert!(i2.0 < x && i2.1 < y);
    let mut i:usize = 0;
    for x in 0..i2.0 {
        i += x;
    }
    i + i2.1
}

/*
slides arr1 by k spots to the right
*/
pub fn slide_arr1<T>(a: Array1<T>,k:usize) -> Array1<T>
where T:Clone + Default
{
    let l = a.len();
    //assert!(k < l);
    let k_ = k % l;
    // first segment [:l -k]
    let mut a2:Vec<T> = a.slice(s![l -k_..l]).to_owned().into_iter().collect();
    a2.extend(a.slice(s![0..l-k_]).to_owned());
    a2.into_iter().collect()
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

    #[test]
    fn test_label_to_iset_map() {
        let h = label_to_iset_map(vec![0,0,1,1,1,1,0]);
        let ans:HashMap<usize,Vec<usize>> = HashMap::from_iter([(1,vec![2, 3, 4, 5]), (0,vec![0, 1, 6])]);
        assert_eq!(h,ans);
    }

}
