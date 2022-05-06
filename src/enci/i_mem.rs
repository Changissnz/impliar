/*
memory structure for interpolator
*/

use ndarray::{Array,Array1,Array2,arr1,arr2,s};
use std::collections::{HashMap,HashSet};


pub struct ContraStruct {
    // timestamp, element
    pub index_identifier:Vec<usize>,
    pub expected:Option<f32>,
    pub got:Option<f32>
}

pub fn build_contrastruct(index_identifier:Vec<usize>,expected:Option<f32>,got:Option<f32>) -> ContraStruct {
    ContraStruct{index_identifier:index_identifier,expected:expected,got:got}
}

pub struct IMem {
    pub soln_log:Vec<Array1<Option<f32>>>,
    pub contradiction_log: Vec<ContraStruct>,
    pub i:usize
}

pub fn build_one_imem() -> IMem {
    IMem{soln_log:Vec::new(),contradiction_log:Vec::new(),i:0}
}

impl IMem {

    /*
    */
    pub fn timestamp_differences(&mut self,t1:usize,t2:usize) -> (HashSet<usize>,HashSet<usize>) {

        // get t1 +
        let d1: HashSet<usize> = t1.difference(&t2).into_iter().collect();
        // get t1 -
        let d2: HashSet<usize> = t2.difference(&t1);

        (d1,d2)
    }

    pub fn add_soln(&mut self,soln: Array1<Option<f32>>) {
        self.soln_log.push(soln);
    }

    pub fn add_contradiction(&mut self,c: ContraStruct) {
        self.contradiction_log.push(c);
    }

    pub fn contrastructs_at_ii(&mut self,ii: Vec<Option<f32>>) -> Vec<ContraStruct> {
        let mut sol: Vec<ContraStruct> = Vec::new();
        for c in self.contradiction_log.into_iter() {
            let mut stat:bool = true;
            for (j,ii_) in ii.iter().enumerate() {
                if (*ii_).is_none() {
                    continue;
                }

                let xi = (*ii_).unwrap();

                if xi != c.index_identifier[j] {
                    stat = false;
                    break;
                }

            }

            if stat {
                sol.push(c.clone());
            }
        }
        sol
    }


    pub fn add_contradicted_sequence(&mut self,ieg: Array2<Option<f32>>) {
        if ieg.dim().0 == 0 {return;}
        assert_eq!(ieg.dim().1,3);

        let l = ieg.dim().0.clone();

        // add each element as contradiction
        for i in 0..l {

            let r = ieg.row(i).to_owned();
            let i_ = r[0].unwrap().clone();
            let mut ii = Vec::new();
            ii.push(self.i);
            ii.push(i_);
            let cs = self.build_contrastruct(ii,r[1].clone(),r[2].clone());
            self.add_contradiction(cs);
        }

        self.i += 1;
    }

}
