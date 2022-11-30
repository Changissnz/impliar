use crate::setti::disinc;
use crate::setti::setf;

/*
Disclude-Include Forward Branching Chain Head
*/
pub struct DisIncForwardBranchingCH {
    qual_intervals: Vec<(f32,f32)>,
    branch_cache: Vec<disinc::DisIncForwardChainHead>
}

pub fn build_DisIncForwardBranchingCH(qi: Vec<(f32,f32)>) -> DisIncForwardBranchingCH {

    for q in qi.clone().into_iter() {
        assert!(q.0 >= -1. && q.1 <= 1. && q.0 <= q.1);
    }

    DisIncForwardBranchingCH{qual_intervals:qi,branch_cache:Vec::new()}
}

impl DisIncForwardBranchingCH {

    /*

    */
    pub fn next_branch(&mut self, b: &mut disinc::DisIncForwardChainHead,x:Vec<String>) -> Vec<disinc::DisIncForwardChainHead> {
        // iterate through qual_intervals and check for
        let mut sol: Vec<disinc::DisIncForwardChainHead> = Vec::new();
        let q2 = self.qual_intervals.clone();
        for (i,q) in q2.into_iter() {
            // set qual interval
            let mut b2 = b.clone();
            self.change_float_range(&mut b2,(i,q));

            // process next
            let f = b2.decision_process(x.clone());

            if f {
                sol.push(b2);
            }
        }

        sol

    }

    pub fn change_float_range(&mut self, dif: &mut disinc::DisIncForwardChainHead, new_range:(f32,f32)) {
        assert!(new_range.0 > -1. && new_range.1 < 1.);
        dif.dsr.float_range = new_range;
    }

    /*
    modifies identifier of DisIncForwardChainHead
    */
    pub fn change_identifier(&mut self, dif: &mut disinc::DisIncForwardChainHead, i:usize) {

        //
        let mut v = setf::str_to_vec(dif.idn(),'_');
        v.push(i.to_string());
        dif.mod_identifier(setf::vec_to_str(v,'_'));
    }

}
