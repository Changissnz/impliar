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

}
