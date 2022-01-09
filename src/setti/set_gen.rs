#![allow(dead_code)]
#![allow(unused_variables)]
// set-generating functions
pub use std::collections::HashSet;
pub use std::collections::HashMap;

use crate::setti::setf;
use crate::setti::strng_srt;

pub struct SGen {
    pub value: Vec<String>,
    pub data: Vec<HashSet<String>>,
    pub next: Vec<HashSet<String>>
}

impl SGen {

    pub fn fcollect_all(&mut self,k : usize) {
        let mut x :i32 = 0;
        while true {
            self.next = fcollect(self.value.clone(), x as usize,k as usize);
            self.add_next();

            if self.next.len() == 0 {
                break;
            }

            x += 1;
        }

    }

    pub fn add_next(&mut self) {
        for x in self.next.iter() {
            let h: HashSet<String> = x.clone();
            self.data.push(h);
        }
    }
}

pub fn fcollect(s: Vec<String>, r: usize,k: usize) -> Vec<HashSet<String>> {
    let mut result: Vec<HashSet<String>> = Vec::new();

    // make a queue and a dictionary
    // value == <index of last element, valid index for next>
    let mut d: HashMap<String,Vec<usize>> = HashMap::new();
    let mut q: Vec<String> = Vec::new();

    // initialize the queue and dictionary
    let d2: Vec<String> = vec![s[r].clone()];
    let d3 = vec![r,r+1];
    d.insert(setf::vec_to_str(d2),d3);
    q.push(s[r].clone());

    //let mut c = 0;
    while q.len() > 0 {
        // fetch next base
        let x = q[0].clone();
        q = q[1..].to_vec();

        //println!("next base: {}", x);

        // get info for add-on
        let mut ni = (d.get_mut(&x).unwrap()).clone();
        let mut x3 = setf::str_to_vec(x.clone());

        //println!("check0 {}|{}",ni[0],ni[1]);
        //println!("check {}|{}|{}|{}",ni[1],x3.len(),k,s.len());

            // determine if terminate
        let slen = s.len() + 1;
    	let vil = valid_index_limit(ni[1] as i32, x3.len() as i32,
    		k as i32, slen as i32);

    	if !vil {
            //println!("not valid index");
            //println!("-------------------------------------------");
    		continue;
    	}

        while x3.len() < k {

    		let x32 = x3.clone();

            //println!("_: {},{}",ni[0],ni[1]);

    		// UPDATE ELEMENT
    		let n = s[ni[1]].clone();
    		x3.push(n);

    		// UPDATE OLD KEY
            //println!("\t---------|----------");
    		let ok = setf::vec_to_str(x32);
            //println!("updating old key {}", ok);
            //println!("\t* index for update: {},{}",ni[0],ni[1]);
	        let mut nok = (d.get_mut(&(ok.to_string())).unwrap()).clone();
    		nok[1] = nok[1] + 1;
            //println!("\t* new key {}|{}", nok[0],nok[1]);
    		d.insert(ok.to_string(),nok);

    		// UPDATE NEW KEY
    		let nk = setf::vec_to_str(x3.clone());
            //println!("updating new key {}", nk);

            //let mut nk2 = nk.clone();
    			// case: new key does not exist
    		if !d.contains_key(&(nk.clone())) {
    			let ans = vec![ni[1],ni[1] + 1];
                //println!("\tkey not contained: {}",nk);
                if ni[1] + 1 >= s.len() {
                    continue;
                }
                //println!("\t\tupdating: {},{}|{}",ans[0],ans[1] - 1,ans[1]);
    			d.insert(nk.clone(),ans.clone());
    		} else {// case: new key exist
    			let mut ans = (d.get_mut(&(nk.clone())).unwrap()).clone();
    			ans[1] = ans[1] + 1;
                //println!("\tkey contained, updating {},{}|{}",ans[0],ans[1] - 1, ans[1]);
                //println!("\tkey contained");

                if ni[1] +1 >= s.len() {
                    continue;
                }
                //println!("updating {},{}|{}",ans[0],ans[1] - 1, ans[1]);
                d.insert(nk.clone(),ans.clone());
    		}

    		// UPDATE RELEVANT VARS
    		ni = (d.get_mut(&(nk.clone())).unwrap()).clone();

            if x3.len() >= k {
                continue;
            }

            //println!("\n\tupdate key after: {}|{}",ni[0],ni[1]);
    		q.push(nk.to_string());
        }
        let nk = setf::vec_to_str(x3.clone());

        //println!("^ key is {}",nk);
        let h: HashSet<String> = x3.into_iter().collect();
        result.push(h);
        q.push(x.clone());
        //println!("-------------------------------------------");
   }
    result
}

/*
i := index at
r := present length
k := wanted length
l := length of entire sequence
*/
pub fn valid_index_limit(i: i32, r: i32, k: i32, l:i32) -> bool {

    if i > l - 1 {
        return false;
    }

    if i + k - r < l {
        return true;
    }

    false
}


pub fn stringized_srted_vec(v: &mut Vec<String>) -> String {
    strng_srt::sort_string_vector(v);
    //"sdfasdf".to_string()
    setf::vec_to_str(v.to_vec())
}

pub fn string_hashset_to_vector(h: HashSet<String>) -> Vec<String> {
    let mut v:Vec<String> = Vec::new();

    for h_ in h.iter() {
        v.push((*h_).clone());
    }
    v
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stringized_srted_vec() {
        let mut x = vec!["a".to_string(), "ar".to_string(), "bxx".to_string()];
        let mut y = stringized_srted_vec(&mut x);
        assert_eq!(y,"a-ar-bxx".to_string());
    }


    #[test]
    fn test_fcollect() {

            let mut x = vec!["a".to_string(), "ar".to_string(), "bxx".to_string(),
                "d".to_string(), "dr".to_string(), "dxx".to_string()];

            let mut y = stringized_srted_vec(&mut x);
            //println!("{}",y);
            let mut s = fcollect(x,0,3);
            /*
            for s_ in s.iter() {
                let mut s2: Vec<String> = Vec::from_iter((*s_).clone());
                let s3 = stringized_srted_vec(&mut s2);
                println!("^^: {}", s3);
            }
            */

            assert_eq!(s.len(),10);

    }


    #[test]
    fn test_SGen_fcollect() {

        let mut value = vec!["arbitrox".to_string(), "bartinuell".to_string(),
            "radinox".to_string(), "reskeyiazam".to_string(),"garbitol".to_string(),
            "weinfarsitol".to_string()];

        let mut sg = SGen{value:value,data:Vec::new(),
                                    next:Vec::new()};
        sg.fcollect_all(3);

        for x in sg.data.iter() {
            let mut v: Vec<String> = Vec::from_iter((*x).clone());
            let mut v2 = stringized_srted_vec(&mut v);
        }
        assert_eq!(20,sg.data.len());
    }
}
