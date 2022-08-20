use crate::setti::inc;
use crate::setti::setf;
use substring::Substring;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use crate::enci::parentnot;
use ndarray::Array1;


pub fn default_branch_identifier_seed() -> &'static str {
    "XX5A"
}

pub fn default_function_identifier_seed() -> &'static str {
    "YY67Z"
}

pub fn is_branch_identifier(s:String) -> bool {
    if s.len() < 4 {
        return false;
    }

    let q = s.substring(0,4).to_owned();
    q == default_branch_identifier_seed()
}

/// order-of-operations struct
#[derive(Clone)]
pub struct OrderOfOperator {
    pub str_repr: String,
    pub branches: Vec<String>,
    pub branch_identifiers: HashMap<String,String>,
    pub incstring: inc::Inc1String,
}

pub fn build_order_of_operator(sr:String) -> OrderOfOperator {
    let is1 = inc::Inc1String{value: default_branch_identifier_seed().to_string()};
    OrderOfOperator{str_repr:sr, branches: Vec::new(), branch_identifiers: HashMap::new(),
        incstring: is1}
}

pub fn next_element(s:String, i:usize) -> Option<(String,usize)> {
    let l = s.len();
    let mut newElement:String = "".to_string();
    let mut u: usize = i;
    for x in i..l {
        u = x;
        let x3:&str = &(s.substring(x,x+1)).to_owned();
        if x3 != "(" && x3 != ")" && x3 != "_" {
            newElement = newElement + x3;
            u = x + 1;
            continue;
        }
        break;
    }

    if newElement == "".to_string() {None} else {Some((newElement,u))}
}


impl OrderOfOperator {

    pub fn process(&mut self) {
        let mut i:usize = 0;
        while true {
            let x = self.process_next(i);
            if x.is_none() {
                break;
            }
            i = x.unwrap();
        }
    }

    /// 
    pub fn process_next(&mut self,i:usize) -> Option<usize> {
        let l = self.str_repr.len();
        let s = &self.str_repr.substring(i,i+1).to_owned();
        if i >= l {
            return None;
        }

        if s == "(" {
            let lb = self.branches.len();
            return self.new_branch(i + 1);
        } else if s == ")" {
            return self.close_branch(i);
        } else if s == "_" {
            return Some(i + 1);
        }
        else {

            // case: no available branches, make new branch
            if self.branches.len() == 0 {
                return self.new_branch(i);
            }

            let m = next_element(self.str_repr.clone(),i);
            if m.is_none() {
                return None;
            }

            // add to last branch
            self.add_to_last_branch(m.as_ref().unwrap().0.to_string());
            return Some(m.unwrap().1.clone());
        }
    }

    pub fn add_to_last_branch(&mut self,s:String) {
        let l2 = self.branches.len() - 1;
        let mut q = self.branches[l2].to_owned();//clone();
        q = q + "_" + s.as_str();
        mem::replace(&mut self.branches[l2],q);
    }

    /*
    saves old branch and

    index [i] is "("
    collects the first element for the new branch. outputs the next index
    */
    pub fn new_branch(&mut self,i: usize) -> Option<usize> {
        let x = next_element(self.str_repr.clone(),i);
        if x.is_none() {
            return None;
        }
        let q = &x.as_ref().unwrap().0;//.clone();
        let q2 = x.as_ref().unwrap().1;//.clone();
        self.branches.push(q.to_string());
        Some(q2)
    }

    /*
    call this before going on to new branch, close is ")".
    when branch is closed, all pre-branch data is merged.
    */
    fn close_branch(&mut self, i: usize) -> Option<usize> {
        // make identifier for branch
        let q = self.incstring.inc_();

        // locate the branch data: last element in branches
        let b:&str = self.branches[self.branches.len() - 1].as_str();//clone();

        // case: branch is identifier, merge last 2 branch
        if is_branch_identifier(b.to_string()) {
            let b2:&str = self.branches[self.branches.len() - 2].as_str();//.clone();
            let b3:String = b2.to_owned() + "_" + b;
            self.branch_identifiers.insert(q.clone(),b3.clone());
            let l2 = self.branches.len() - 2;
            self.branches = self.branches[0..l2].to_vec();
            self.branches.push(q);
        } else {
            // case: new branch
            // log the branch identifier
            self.branch_identifiers.insert(q.clone(),b.to_string());
            let l = self.branches.len() - 1;
            self.branches[l] = q;
        }

        Some(i + 1)

    }

}

///////////////////////////////////////// test code

pub fn sample_OrderOfOperator_soln1() -> (Vec<String>,HashMap<String,String>) {
    let vs = vec!["5".to_string(),"XX5A".to_string()];
    let mut bi:HashMap<String,String> = HashMap::new();
    bi.insert("XX5A".to_string(),"6_7".to_string());
    (vs,bi)
}

pub fn sample_OrderOfOperator_soln2() -> (Vec<String>,HashMap<String,String>) {
    let vs:Vec<String> = vec!["XX5B".to_string(),"4".to_string(),"XX5D".to_string()];
    let mut bi:HashMap<String,String> = HashMap::new();
    bi.insert("XX5B".to_string(),"8_5_1_XX5A".to_string());
    bi.insert("XX5D".to_string(),"XX5C_7".to_string());
    bi.insert("XX5A".to_string(),"2_3".to_string());
    bi.insert("XX5C".to_string(),"5_6".to_string());
    (vs,bi)
}

pub fn sample_OrderOfOperator_soln3() -> (Vec<String>,HashMap<String,String>) {
    let vs:Vec<String> = vec!["XX5B_4".to_string(),"XX5C_7".to_string()];
    let mut bi:HashMap<String,String> = HashMap::new();
    bi.insert("XX5B".to_string(),"8_5_1_XX5A".to_string());
    bi.insert("XX5A".to_string(),"2_3".to_string());
    bi.insert("XX5C".to_string(),"5_6".to_string());
    (vs,bi)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_OrderOfOperator_process() {
        // sample 1
        let mut x = build_order_of_operator("5(6_7)".to_string());
        x.process();
        let (vs,bi) = sample_OrderOfOperator_soln1();
        assert_eq!(x.branches,vs);
        assert_eq!(x.branch_identifiers,bi);

        // sample 2
        let mut x2 = build_order_of_operator("(8_5_1(2_3))(4(5_6)7)".to_string());
        x2.process();
        let (vs2,bi2) = sample_OrderOfOperator_soln2();
        assert_eq!(x2.branches,vs2);
        assert_eq!(x2.branch_identifiers,bi2);

        // sample 3
        let mut x3 = build_order_of_operator("(8_5_1(2_3))4(5_6)7".to_string());
        x3.process();
        let (vs3,bi3) = sample_OrderOfOperator_soln3();
        assert_eq!(x3.branches,vs3);
        assert_eq!(x3.branch_identifiers,bi3);

        // test equality of samples 4,5
        let mut x4 = build_order_of_operator("5_6".to_string());
        let mut x5 = build_order_of_operator("(5_6)".to_string());
        assert_eq!(x4.branches,x5.branches);
        assert_eq!(x4.branch_identifiers,x5.branch_identifiers);
    }

}
