/*
some matrix functions
*/
use ndarray::array;
use ndarray::{Array,Array1,Array2, arr2,arr3, stack,s,Axis,Dim};

pub fn replace_vec_in_arr2<T>(a:&mut Array2<T>,q:&mut Array1<T>,i:usize,isRow:bool)
where
T:Clone
 {
    if isRow {
        assert!(i < a.raw_dim()[0] && i >= 0);
        let mut b = a.slice_mut(s![i, ..]);
        b.assign(&q);
    } else {
        assert!(i < a.raw_dim()[1] && i >= 0);
        let mut b = a.slice_mut(s![.., i]);
        b.assign(&q);
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
            v[i] = sol[i].clone();
        }
    }
    sol
}

//vec

//pub fn replace_element_in_arr2(a)

//pub fn is_subvector_equal() {
//}

/*
2|0,4
9|0,13,4

* duplicates? 
*/
