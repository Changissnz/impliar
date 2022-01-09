#![allow(dead_code)]
#![allow(unused_variables)]
use std::cmp::Ordering;

pub fn str_cmp3(s1: &String, s2: &String) -> std::cmp::Ordering {

    if lessr_str(s1,s2) == s1 {
    	return Ordering::Less;
    }
    Ordering::Greater
}

pub fn lessr_str<'life>(s1: &'life String, s2: &'life String) -> &'life String {
	//let mux b :bool = str_lessr(s1,s2);
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

}
