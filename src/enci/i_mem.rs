/*
memory structure for interpolator
*/

use ndarray::{Array,Array1,Array2,arr1,arr2,s};
use std::collections::{HashMap,HashSet};

#[derive(Clone)]
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
    // TODO: add map_log
    pub contradiction_log: Vec<ContraStruct>,
    pub i:usize
}

pub fn build_one_imem() -> IMem {
    IMem{soln_log:Vec::new(),contradiction_log:Vec::new(),i:0}
}

impl IMem {

    /*
    */
    pub fn timestamp_differences(&mut self,t1:usize,t2:usize,f: fn(&Array1<Option<f32>>) -> HashSet<usize>) -> (HashSet<usize>,HashSet<usize>) {
        let t1_:HashSet<usize> = f(&self.soln_log[t1]);
        let t2_:HashSet<usize> = f(&self.soln_log[t2]);

        // get t1 +
        let d1: HashSet<usize> = t1_.difference(&t2_).into_iter().map(|x| *x).collect();

        // get t1 -
        let d2: HashSet<usize> = t2_.difference(&t1_).into_iter().map(|x| *x).collect();

        (d1,d2)
    }


    pub fn add_soln(&mut self,soln: Array1<Option<f32>>) {
        self.soln_log.push(soln);
    }

    pub fn add_contradiction(&mut self,c: ContraStruct) {
        self.contradiction_log.push(c);
    }

    pub fn contrastructs_at_ii(&mut self,ii: Vec<Option<usize>>) -> Vec<ContraStruct> {
        let mut sol: Vec<ContraStruct> = Vec::new();
        for c in self.contradiction_log.iter() {
            let mut stat:bool = true;
            for (j,ii_) in ii.iter().enumerate() {
                if (*ii_).is_none() {
                    continue;
                }

                let xi = (*ii_).unwrap();

                if xi != (*c).index_identifier[j] {
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
            let i_ = r[0].unwrap().clone() as usize;
            let mut ii:Vec<usize> = Vec::new();
            ii.push(self.i);
            ii.push(i_);
            let cs = build_contrastruct(ii,r[1].clone(),r[2].clone());
            self.add_contradiction(cs);
        }

        self.i += 1;
    }

}
