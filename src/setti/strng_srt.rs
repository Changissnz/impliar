#![allow(dead_code)]
#![allow(unused_variables)]
use std::cmp::Ordering;
use substring::Substring;
use asciis::asc::Asciis;

pub fn str_cmp3(s1: &String, s2: &String) -> std::cmp::Ordering {

    if lessr_str(s1,s2) == s1 {
    	return Ordering::Less;
    }
    Ordering::Greater
}

pub fn lessr_str<'life>(s1: &'life String, s2: &'life String) -> &'life String {
	let b = str_lessr(s1,s2);
    if b {
       return s1;
    }
    s2
}
//*/

pub fn str_lessr<'life>(s1: &'life String, s2: &'life String) -> bool {
    assert!((s1.len() > 0) & (s2.len() > 0), "strings cannot be empty");

    for (i,s) in s1.chars().enumerate() {
        if i > s2.len() - 1 {
            return false;
        }
        let m1 = s as u32;
        let m2 = s2.chars().nth(i).unwrap() as u32;// s2[i] as u32;
        if m1 < m2 {
            return true;
        } else if m1 == m2 {
            continue;
        } else {
            return false;
        }
    }
    true
}

pub fn sort_string_vector(v1: &mut Vec<String>) {
    (*v1).sort_by(str_cmp3);
}


////////////////////////////////////////////// methods on inc1string below
pub fn sort_inc1string_vector(v1: &mut Vec<String>) {
    (*v1).sort_by(str_cmp2);
}

pub fn is_proper_string(s:String) -> bool {
    //s.chars().all(char::is_alphabetic)//numeric)
    s.chars().all(char::is_alphanumeric)

}

pub fn inc1string_to_u32(s:String) -> u32 {
    assert_eq!(is_proper_string(s.clone()),true);
    let mut x:u32 = (122 * (s.len() - 1)) as u32;
    let asc = Asciis{};
    let mut t = (s.substring(s.len() -1,s.len())).to_owned();
    let mut r:u32 = (u32::try_from(asc.ord(t.as_str()).unwrap())).unwrap();// as u32;
    x + r
}

pub fn inc1string_vector_max(s:Vec<String>) -> String {
    let s2:Vec<u32> = s.iter().map(|x| inc1string_to_u32((*x).clone())).collect();
    // get max
    let s2m:u32 = *s2.iter().max().unwrap();

    // get index of max
    let im = s2.iter().position(|&r| r == s2m).unwrap();

    s[im].clone()
}

pub fn str_cmp2(s1: &String, s2: &String) -> std::cmp::Ordering {

    if inc1string_to_u32((*s1).clone()) <= inc1string_to_u32((*s2).clone()) {
        return Ordering::Less;
    }

    Ordering::Greater
}

////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_string_vector() {

        // case 1:
        let mut e = vec!["sdfasdfas".to_string(),
            "sdfasdfas#".to_string(),"10.3121".to_string()];

        let esol = vec!["10.3121".to_string(),"sdfasdfas".to_string(),
            "sdfasdfas#".to_string()];

        sort_string_vector(&mut e);
        for (i,e_) in e.iter().enumerate() {
              assert_eq!(e_.to_string(),esol[i].to_string(), "case1: expected {} got {}",
                e_.to_string(),esol[i].to_string());
        }

        // case 2:
        let mut e2 = vec!["SADINSKY".to_string(),
            "SADISHKY".to_string(),"SADICKsky".to_string()];

        let e2sol = vec!["SADICKsky".to_string(),
            "SADINSKY".to_string(),"SADISHKY".to_string()];
        sort_string_vector(&mut e2);
        for (i,e_) in e2.iter().enumerate() {
              assert_eq!(e_.to_string(),e2sol[i].to_string(), "case2: expected {} got {}",
                e_.to_string(),e2sol[i].to_string());
        }
    }


    #[test]
    fn test_inc1string_vector_max() {
        let mut v3 = vec!["ant".to_string(),"balkans".to_string(),"blacks".to_string()];
        let mut s3 = inc1string_vector_max(v3);
        assert_eq!(s3,"balkans".to_string());
    }

    #[test]
    fn test_sort_inc1string_vector() {
        let mut sv2 = vec!["a".to_string(),"aa".to_string(),"c".to_string(),
            "ba".to_string(),"za".to_string()];
        sort_inc1string_vector(&mut sv2);
        let mut sol = vec!["a".to_string(),"c".to_string(),"za".to_string(),
            "ba".to_string(),"aa".to_string()];
        assert_eq!(sol,sv2);
    }

}
