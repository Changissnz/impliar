/*
a table of costillos
*/
use crate::metrice::costillo;
use crate::setti::matrixf;

pub struct Costillo_T {
    csvec: Vec<costillo::Costillo>,
    dimn: (usize,usize)
}

pub fn build_Costillo_T(csvec:Vec<costillo::Costillo>,dimn:(usize,usize)) -> Costillo_T {
    assert_eq!(csvec.len(),dimn.0 * dimn.1);
    Costillo_T{csvec:csvec,dimn:dimn}
}

impl Costillo_T {

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
