/*
parenthetical float operator,
struct is specialized to process parenthetical
strings by order of operator given a float map.
outputs a float.
*/
use crate::enci::ohop;
use crate::setti::setf;
use crate::setti::ftemp;
use std::collections::HashMap;

pub struct OperatorSequence {
    pub s: Vec<fn(f32,f32) -> f32>
}

pub fn build_operator_sequence(s: Vec<fn(f32,f32) -> f32>) -> OperatorSequence {
    OperatorSequence{s:s}
}

/*
Uses sequence of functions in OperatorSequence
to iteratively calculate all unknown variables
in OrderOfOperator.

Sequence of functions will not be in accordance
with parenthetical expression if parentheses exist
in.

Order of sequence of functions corresponds to the
branch syntax calculated from OrderOfOperator.
*/
pub struct PFOperator {
    pub oo: ohop::OrderOfOperator,

    pub os: OperatorSequence,

    // branches processing index of oo
    pub pi: usize,

    // f32 for direct vars in str_repr
    pub hm: HashMap<String,f32>,
    // f32 for branches
    pub branch_soln: HashMap<String,f32>,

    // f32 for pexpr of oo.branches
    pub pexpr_soln:Vec<f32>,

    pub solvable:bool,
    pub processed:bool,
    pub output: Option<f32>
}

pub fn build_PFOperator(oo: ohop::OrderOfOperator,os: OperatorSequence,hm: HashMap<String,f32>,) -> PFOperator {
    assert!(os.s.len() > 0);
    PFOperator{oo:oo,os:os,pi:0,hm:hm,branch_soln:HashMap::new(),
        pexpr_soln:Vec::new(),solvable:true,processed:false,output:None}
}


impl PFOperator {

    /*
    determines if variable is unknown
    */
    pub fn is_unknown(&mut self,k:String) -> bool {
        !self.hm.contains_key(&k) && !self.branch_soln.contains_key(&k)
    }

    pub fn try_solving_key(&mut self,k:String) -> bool {
        if self.hm.contains_key(&k) {
            return true;
        }

        if self.branch_soln.contains_key(&k) {
            return true;
        }

        false
    }

    pub fn fetch_value(&mut self,identifier:String) -> Option<f32> {

        if self.hm.contains_key(&identifier) {
            return Some(self.hm.get_mut(&identifier).unwrap().clone());
        }

        if self.branch_soln.contains_key(&identifier) {
            return Some(self.branch_soln.get_mut(&identifier).unwrap().clone());
        }

        None
    }

    /*
    solves pexpr if known; shrinks function sequence of os
    after processing

    pexpr := expression w/o parentheses, process from L->R
    */
    pub fn solve_pexpr_if_known(&mut self,pexpr:String) -> Option<f32> {
        let x = setf::str_to_vec(pexpr.clone());
        let unk:Vec<String> = x.iter().filter(|y| self.is_unknown((*y).clone())).map(|y| (*y).clone()).collect();

        if unk.len() > 0 {
            return None;
        }

        let l = x.len();

        let mut fv = self.fetch_value(x[0].clone()).unwrap();

        //println!("solving pexpr: {}",pexpr);
        //println!("len of functions: {}",self.os.s.len());
        for i in 1..l {
            //println!("prev fv: {}",fv);
            let fv2:f32 = self.fetch_value(x[i].clone()).unwrap();
            //println!("next fv: {}",fv2);
            fv = self.os.s[0](fv,fv2);
            //println!("new fv: {}",fv);
            self.os.s = self.os.s[1..].to_vec();
        }

        //println!("after");
        //println!("len of functions: {}",self.os.s.len());
        Some(fv)
    }

    /*
    processes one branch for all unknown values
    */
    pub fn process_one_branch(&mut self) -> Option<f32> {

        let cached_values:Vec<f32> = Vec::new();
        let b = self.oo.branches[self.pi].clone();

        // split into elmts
        let es = setf::str_to_vec(b.clone());

        // get all initial unknown vars (of branches)
        let mut unk:Vec<String> = es.iter().filter(|y| self.is_unknown((*y).clone())).map(|y| (*y).clone()).collect();
        let mut l = unk.len();

        // process for each value of unknown var
        while l > 0 {

            let q = unk[0].clone();
            unk = unk[1..].to_vec();

            // get unknown elements of unknown branch
            let bi = self.oo.branch_identifiers.get_mut(&q).unwrap().clone();
            let x = setf::str_to_vec(bi.clone());
            let mut unk2:Vec<String> = x.iter().filter(|y| self.is_unknown((*y).clone())).map(|y| (*y).clone()).collect();

            // case: cannot be solved, missing var value

            // case: unknown elements exist
            if unk2.len() > 0 {
                unk2.push(q.clone());
                unk2.extend_from_slice(&unk);
                unk = unk2.clone();
            } else {
            // case: no more unknown, solve
                let solved:f32 = self.solve_pexpr_if_known(bi.clone()).unwrap();
                self.branch_soln.insert(q.clone(),solved);
            }

            l = unk.len();
        }

        // reset function and solve knowned pexpr
        let sol = self.solve_pexpr_if_known(b.clone());

        if sol.is_none() {
            return sol;
        }

        self.pexpr_soln.push(sol.clone().unwrap());
        sol
    }

    pub fn process(&mut self) -> bool {
        if self.processed {
            return self.solvable;
        }

        let l = self.oo.branches.len();

        while self.pi < l {
            if self.process_one_branch().is_none() {
                self.solvable = false;
                break;
            }
            self.pi += 1;
        }

        self.processed = true;
        if self.solvable {
            self.output = Some(self.process_solved());
        }

        return self.solvable
    }

    pub fn process_solved(&mut self) -> f32 {
        assert!(self.processed);

        let mut fv:f32 = self.pexpr_soln[0].clone();
        let l = self.pexpr_soln.len();

        for i in 1..l {
            fv = self.os.s[0](fv,self.pexpr_soln[i]);
            self.os.s = self.os.s[1..].to_vec();
        }
        fv

    }

    pub fn output(&mut self) -> Option<f32> {
        if !self.processed {
            return None;
        }

        if !self.solvable {
            return None;
        }

        self.output.clone()

    }
}

pub fn test_sample_PFOperator_1() -> PFOperator {
    // declare order of operator
    let mut x = ohop::build_order_of_operator("(8_5_1(2_3))4(5_6)7".to_string());
    x.process();

    // declare sequence of functions
    let mut s: Vec<fn(f32,f32) -> f32> = Vec::new();
    for i in 0..8 {
        if i % 2 == 1 {
            s.push(ftemp::basic_add());
        } else {
            s.push(ftemp::basic_mult());

        }
    }

    let os = build_operator_sequence(s);
    let vars_to_value:HashMap<String,f32> = HashMap::from_iter([
        ("1".to_string(),32.),
        ("2".to_string(),3.2),
        ("3".to_string(),0.32),
        ("4".to_string(),312.),
        ("5".to_string(),13.),
        ("6".to_string(),23.),
        ("7".to_string(),31.),
        ("8".to_string(),50.)]);

    // make a sequence of add and mult functions
    let pfo = build_PFOperator(x,os,vars_to_value);
    pfo
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test__PFOperator__process_one_branch() {
        let mut pfo = test_sample_PFOperator_1();
        pfo.process_one_branch();

        let actual_soln:HashMap<String,f32> = HashMap::from([("XX5A".to_string(),1.024), ("XX5B".to_string(),2017.024)]);
        assert_eq!(actual_soln,pfo.branch_soln);
    }

    #[test]
    fn test__PFOperator__process() {
        let mut pfo = test_sample_PFOperator_1();
        pfo.process();
        assert_eq!(630427.5,pfo.output.unwrap());
    }



}
