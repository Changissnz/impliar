#![allow(dead_code)]
#![allow(unused_variables)]
/*
set functions
*/
pub use std::collections::HashSet;
pub use std::collections::HashMap;
use std::string::ToString;
use std::string::String;
use substring::Substring;
use std::fmt;

pub struct VectorCounter {
    pub data: HashMap<String,i32>,
}

pub trait Count<T> {
    fn countv(&mut self,v:Vec<T>);// -> &mut VectorCounter;
}

impl<T> Count<T> for VectorCounter
where T: ToString{
    fn countv(&mut self,v: Vec<T>) {

        for v_ in v.iter() {
            let x = v_.to_string();

            if self.data.contains_key(&x) {
                let x2 = self.data[&x];
                self.data.insert(x,x2 + 1);
            } else {
                self.data.insert(x,1);
            }
        }
    }
}

impl fmt::Display for VectorCounter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut s = String::from("");
            for k in self.data.keys() {
                s = s + &(k).to_string() + "|=|";// + ",";
                s = s + &(self.data[k].to_string()) +", ";
            }

            if s.len() > 0 {
                s = (s.substring(0,s.len() -1)).to_owned();
            }

            write!(f, "{}", s)//self.number)
        }
}

pub fn vec_to_str<T>(s: Vec<T>) -> String
    where
    T : ToString {

    let mut h = String::from("");
    for s_ in s.iter() {
        h = h + &(s_.to_string());
        h.push_str("-");
    }

    if h.len() > 0 {
        h = (h.substring(0,h.len() -1)).to_owned();
    }

    h
    //s.join("-");
    //itertools::join(s, ", ")
}


pub fn str_to_vec(s:String) -> Vec<String> {
    let mut j: i32 = 0;
    let mut v: Vec<String> = Vec::new();
    let mut c = 0;
    while true {
        // get substring starting at j
        let sx = s.to_string().substring(j as usize, s.len()).to_owned();
        let i = next_str(sx.clone());//(*s4).to_string());
        if i == -1 {
            v.push(s.to_string().substring(j as usize, s.len()).to_owned());
            break;
        }
        let ss = s.to_string().substring(j as usize,(j + i) as usize).to_owned();
        v.push(ss.clone());
        j += i + 1;
        c += 1;
        if c >= 5 {
            break;
        }
    }
    v
}

pub fn next_str(s:String) -> i32 {

    for (i,s_) in s.chars().enumerate() {
        if s_ == '-' {
            return i as i32;
        }
    }
    -1
}


pub fn generic_vec_to_stringvec<T>(v:Vec<T>) -> Vec<String>
    where
    T: ToString
 {

    let mut v2:Vec<String> = Vec::new();
    for v_ in v.iter() {
        let mut v3 = (*v_).to_string();
        v2.push(v3);
    }
    v2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__str_to_vec() {
        // case 1
        let mut s = "lasjdflsadjfsal;fjsald;fjsadl;-flsakdjflas;dfjls;adkjf";
        let sol: Vec<String> = vec!["lasjdflsadjfsal;fjsald;fjsadl;".to_string(),"flsakdjflas;dfjls;adkjf".to_string()];

        let v1 = str_to_vec(s.to_string());

        for (i,v) in v1.iter().enumerate() {
            assert_eq!(v.to_string(),sol[i]);
        }

        // case 2
        let mut s2 = "lasjdflsadjfsal;fjsald;fjsadl;";
        let sol2: Vec<String> = vec!["lasjdflsadjfsal;fjsald;fjsadl;".to_string()];
        let v2 = str_to_vec(s2.to_string());

        for (i,v) in v2.iter().enumerate() {
            assert_eq!(v.to_string(),sol2[i]);
        }

        // case 3
        let mut s = "arbitrox-bartinuell-radinox".to_string();
        let mut s2 = str_to_vec(s);
        assert_eq!(s2,vec!["arbitrox".to_string(), "bartinuell".to_string(),"radinox".to_string()]);

    }

    #[test]
    fn test__vec_to_str() {
        // case 1
        let mut s = vec!["lasjdflsadjfsal;fjsald;fjsadl;"];
        let mut v1 = vec_to_str(s);
        assert_eq!(v1,"lasjdflsadjfsal;fjsald;fjsadl;".to_string());

        // case 2
        let mut s2 = vec!["one","two","2","three"];
        v1 = vec_to_str(s2);
        assert_eq!(v1,"one-two-2-three".to_string());
    }

    #[test]
    fn test_VectorCounter_countv() {

        let mut y1 = vec![1,2,3];
        let mut y2 = vec![2,3,7];
        let mut x = VectorCounter{data:HashMap::new()};
        x.countv(y1);
        x.countv(y2);

        let mut ans = (x.data.get_mut("1").unwrap()).clone();
        assert_eq!(ans,1);

        let mut ans = (x.data.get_mut("2").unwrap()).clone();
        assert_eq!(ans,2);

        let mut ans = (x.data.get_mut("3").unwrap()).clone();
        assert_eq!(ans,2);

        let mut ans = (x.data.get_mut("7").unwrap()).clone();
        assert_eq!(ans,1);
    }

}
