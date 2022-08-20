//! methods for parenthetical notation
use std::str::FromStr;
use substring::Substring;

/// # return
/// stringized range [0..v -1]
pub fn usize_to_choice_vector(v:usize) -> Vec<String> {
    let mut sol: Vec<String> = Vec::new();
    for i in 0..v {
        sol.push(i.to_string());
    }
    sol
}

/// # arguments 
/// `c` := ordered vector of elements comprising the string
/// `q` := the indices of open ( and close )
/// 
/// # return
/// parenthetical string based on open-close indices of `q`
pub fn rangevec_to_parenthetical_string(c: Vec<usize>, q: Vec<(usize,usize)>) -> String {


    if c.len() == 0 {
        return "".to_string();
    }

    let mut p:String = "".to_string();
    let mut x:usize = 0;
    let l = q.len();
    for (i,c_) in c.iter().enumerate() {
        if x < l {

            if q[x].0 == *c_ {
                p = p + "(";
            }
        }
        p = p + c_.to_string().as_str() + "_";

        if x < l {
            if q[x].1 == *c_ {

                // get last char
                let lp = p.len();
                if p.substring(lp - 1,lp) == "_" {
                    p = p.substring(0,lp - 1).to_owned().to_string();
                }

                p = p + ")";
                x += 1;
            }
        }
    }


    let l2 = p.len() - 1;
    p.substring(0,l2).to_owned().to_string()
}

/// # return
/// converted decision vector, <Vec\<usize as str\>> into a range vector, 
/// a sequence of ordered longest contiguous indices
pub fn decisionvec_to_rangevec(v:Vec<String>) -> Vec<(usize,usize)> {
    // determine longest contiguous
    let uv = stringvec_to_usizevec(v.clone());
    let csv = continuous_subvectors(uv.clone());
    csv
}

/// # return
/// converts vector of stringized usize into vector of usizes
pub fn stringvec_to_usizevec(v:Vec<String>) -> Vec<usize> {
    let mut uv: Vec<usize> = Vec::new();
    for v_ in v.iter() {
        let x_ = usize::from_str((*v_).as_str()).unwrap();
        uv.push(x_);
    }
    uv
}

/// # return
/// outputs all continuous subvectors of size >= 2.
///
/// # caution
/// v assumed to be ordered.
pub fn continuous_subvectors(v:Vec<usize>) -> Vec<(usize,usize)> {
    let l = v.len();
    let mut i:usize = 0;
    let mut sol:Vec<(usize,usize)> = Vec::new();
    while i < l {
        let (v1,s):(Vec<usize>,usize) = longest_continuous_f_subvector_from_index(v.clone(),i,is_contiguous);
        let l2 = v1.len();
        i = s;
        if v1.len() < 2 {
            continue;
        }

        sol.push((v1[0].clone(),v1[l2 - 1].clone()));
    }
    sol
}

/// # return
/// if element at v\[i\] is distance 1 to v\[i -1\]?
pub fn is_contiguous(v:Vec<usize>,i:usize) -> bool {
    assert!(i < v.len());
    if i == 0 {
        return true;
    }

    let b1 = v[i - 1].clone();
    let b2 = v[i].clone();
    b2 - b1 == 1
}

/// # description
/// determines the highest index j in v\[i + 1..\] such that x(v\[i\],v\[j\])
pub fn longest_continuous_f_subvector_from_index<T>(v:Vec<T>,i:usize,x:fn(Vec<T>,usize) -> bool) -> (Vec<T>,usize)
where
T: Clone
{
    let mut sol:Vec<T> = Vec::new();
    sol.push(v[i].clone());
    let l = v.len();
    let mut j_ = i;
    for i_ in i + 1..l {
        if x(v.clone(),i_) {
            sol.push(v[i_].clone());
            j_ = i_;
            continue;
        }
        break;
    }

    (sol,j_ + 1)
}

/////////////////////////////////////// test methods

pub fn sample_decision_vec_1() -> Vec<String> {
    vec!["0".to_string(), "1".to_string(), "2".to_string(),
        "4".to_string(), "7".to_string(),"8".to_string(),"15".to_string(),
        "16".to_string(), "17".to_string(),"19".to_string()]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decisionvec_to_rangevec() {
        let v2 = sample_decision_vec_1();
        let x = decisionvec_to_rangevec(v2);
        let sol:Vec<(usize,usize)> = vec![(0,2),(7,8),(15,17)];
        assert_eq!(sol,x);
    }

    #[test]
    fn test_rangevec_to_parenthetical_string() {
        let v2 = sample_decision_vec_1();
        let x = decisionvec_to_rangevec(v2.clone());
        let q = stringvec_to_usizevec(v2.clone());
        let res:String = rangevec_to_parenthetical_string(q.clone(),x.clone());
        assert_eq!(res,"(0_1_2)4_(7_8)(15_16_17)19".to_string());

        let res2:String = rangevec_to_parenthetical_string(q.clone(),Vec::new());
        assert_eq!(res2,"0_1_2_4_7_8_15_16_17_19".to_string());
    }

}
