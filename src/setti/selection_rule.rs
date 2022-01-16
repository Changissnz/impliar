/*
struct that is a restriction or requirement
that implements
*/
use crate::setti::matrixf;
use ndarray::{Dim,Array1,Array2};
use ndarray::array;
use std::collections::HashSet;

pub fn build_rmatrix(rs:usize,idn:i32,resreq: Vec<(usize,Vec<usize>)>,k:usize) ->Array2<i32> {

    assert!(rs > 0 && k > 0);
    assert!(rs >= k);

    // empty array
    let mut sol:Array2<i32>  = Array2::zeros((rs, k));

    // case: empty rule
    if resreq.len() == 0 || rs == 0 {
        return sol;
    }

    for r in resreq.iter() {
        // get index of restricted in reference
        let mut s = (*r).0.clone();
        let mut nu = Array1::zeros(rs);
        for rx in (*r).1.iter() {
            nu[Dim([*rx])] = idn;
        }
        matrixf::replace_vec_in_arr2(&mut sol,&mut nu,s,false);
    }
    sol
}

////////////////////////////////////// methods for RuleCheck

////////////////////////////////////// methods for Restriction

pub struct Restriction {
    // rows are reference elements, columns are indices
    pub data: ndarray::Array2<i32>
}

/*
restricted := Vec<index in 0..k,indices not allowed>
*/
pub fn build_restriction(rs:usize,restricted: Vec<(usize,Vec<usize>)>,k:usize) -> Restriction {
    let mut rm = build_restriction_matrix(rs,restricted,k);
    Restriction{data:rm}
}

pub fn build_restriction_matrix(rs:usize,restricted: Vec<(usize,Vec<usize>)>,k:usize) ->Array2<i32> {
    build_rmatrix(rs,1,restricted,k)
}

////////////////////////////////////// methods for Requirement

pub struct Requirement {
    pub data: ndarray::Array2<i32>,
    pub allReq:bool,
}

pub fn build_requirement(rs:usize,required: Vec<(usize,Vec<usize>)>,k:usize,allReq:bool) -> Requirement {
    let mut x = build_requirement_matrix(rs,required,k);
    Requirement{data:x,allReq:allReq}
}

pub fn build_requirement_matrix(rs:usize,required: Vec<(usize,Vec<usize>)>,k:usize) ->Array2<i32> {
    build_rmatrix(rs,-1,required,k)
}

pub fn check_rule_contents(restricted: &Restriction,
        required: &Requirement) -> bool {
    let mut q = (*restricted).data.clone() * (*required).data.clone();
    let mut h:HashSet<i32> = HashSet::new();
    h.insert(-1);
    let l = (*restricted).data.raw_dim()[0];
    for i in 0..l {
        if !matrixf::exist_any_in_vec_of_arr2(&mut q,h.clone(),i,true) {
            return false;
        }
    }
    true
}

pub fn std_collision_score(y1:&(usize,&i32)) -> bool {
    *((*y1).1) == -1
}

pub fn collision_score(resReq: Array2<i32>,f: fn(&(usize,&i32))->bool) ->i32 {
    let mut x:Array1<_> = resReq.iter().enumerate().filter(f).collect();//.sum()
    let mut x2:Array1<i32> = x.iter().map(|y| *(*y).1).collect();
    x2.len() as i32
}



////////////////////////////////////////////

pub fn test_rule_contents() -> (Restriction,Requirement){

    let rs = 9;
    let k = 4;
    let restriction:Vec<(usize,Vec<usize>)> = vec![
                            (0,vec![0,1,3,4]),(2,vec![0,1,2,5,7])];

    let requirement:Vec<(usize,Vec<usize>)> = vec![
                            (0,vec![0,4,5,6]),(2,vec![5,8])];

    let mut res = build_restriction(rs,restriction,k);
    let mut req = build_requirement(rs,requirement,k,true);
    (res,req)

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_restriction_matrix() {

        let rs:usize = 10;
        let k:usize = 6;
        let mut rest: Vec<(usize,Vec<usize>)> = Vec::new();
        rest.push((0,vec![0,4,6]));
        rest.push((2,vec![1,3,4]));
        rest.push((3,vec![5,6,7,8]));

        let mut sol = build_restriction_matrix(rs,rest,k);

        let mut a2 = array![[1, 0, 0, 0, 0, 0],
                     [0, 0, 1, 0, 0, 0],
                     [0, 0, 0, 0, 0, 0],
                     [0, 0, 1, 0, 0, 0],
                     [1, 0, 1, 0, 0, 0],
                     [0, 0, 0, 1, 0, 0],
                     [1, 0, 0, 1, 0, 0],
                     [0, 0, 0, 1, 0, 0],
                     [0, 0, 0, 1, 0, 0],
                     [0, 0, 0, 0, 0, 0]];

        assert_eq!(sol,a2);
    }

    #[test]
    fn test_initialize_mod_structs() {

        let rs:usize = 10;
        let k:usize = 6;
        let mut rest: Vec<(usize,Vec<usize>)> = Vec::new();
        rest.push((0,vec![0,4,6]));
        rest.push((2,vec![1,3,4]));
        rest.push((3,vec![5,6,7,8]));

        let mut xx = build_rmatrix(rs,-1,rest.clone(),k);
        let mut xy = build_restriction(rs,rest,k);
    }

    #[test]
    fn test_check_rule_contents() {
        let mut x = test_rule_contents();
        let c = check_rule_contents(&x.0,&x.1);
        assert!(!c);
    }

    #[test]
    fn test_collision_score() {
        let mut x = test_rule_contents();
        let mut x3 = x.0.data * x.1.data;
        let mut nx = collision_score(x3.clone(),std_collision_score);
        assert_eq!(nx,3);
    }

}
