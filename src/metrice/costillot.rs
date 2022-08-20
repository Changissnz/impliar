/*
a table of costillos
*/
use crate::metrice::costillo;
use crate::setti::matrixf;

/*
/// consider the [`with_capacity`] method to prevent excessive
/// [`with_capacity`]: #method.with_capacity
*/

/// Creates a new empty `String`.
/// 
/// sfsadfasdfa 
/// - X
/// - Y 
/// 
/// sdfsadfsadf
/// # SADFSADFADF 
pub struct CostilloT {
    csvec: Vec<costillo::Costillo>,
    dimn: (usize,usize)
}

pub fn build_CostilloT(csvec:Vec<costillo::Costillo>,dimn:(usize,usize)) -> CostilloT {
    assert_eq!(csvec.len(),dimn.0 * dimn.1);
    CostilloT{csvec:csvec,dimn:dimn}
}

impl CostilloT {

    // update functions here
    pub fn mod_i(&mut self,i:(usize,usize,usize,usize),e: Option<f32>, g: Option<f32>) {
        let ix = matrixf::two_index_to_one_index((i.0,i.1),self.dimn.0,self.dimn.1);
        self.csvec[ix].mod_i((i.2,i.3),e,g);
    }

    pub fn clear_i(&mut self,i:(usize,usize,usize,usize),c:(usize,usize)) {
        let ix = matrixf::two_index_to_one_index((i.0,i.1),self.dimn.0,self.dimn.1);
        self.csvec[ix].clear_i((i.2,i.3),c);
    }
}
