//use crate::enci::i_mem;
use crate::enci::i_mem;

/*
consists of 4 contrastructs: 00,01,10,11
*/
#[derive(Clone)]
pub struct Costillo {
    pub xs: Vec<i_mem::ContraStruct>
}

/*
*/
pub fn build_Costillo(ex:Vec<Option<f32>>,got:Vec<Option<f32>>) -> Costillo {
    assert_eq!(ex.len(),4);
    assert_eq!(got.len(),4);

    let x0 = i_mem::build_contrastruct(vec![0,0],ex[0],got[0]);
    let x1 = i_mem::build_contrastruct(vec![0,1],ex[1],got[1]);
    let x2 = i_mem::build_contrastruct(vec![1,0],ex[2],got[2]);
    let x3 = i_mem::build_contrastruct(vec![1,1],ex[3],got[3]);

    Costillo{xs:vec![x0,x1,x2,x3]}
}

impl Costillo {

    /*
    CAUTION: does not assign None to Contrastruct variables
    */
    pub fn mod_i(&mut self, i: (usize,usize),e: Option<f32>, g: Option<f32>) {
        let i_ = self.i_to_index(i);

        if !e.is_none() {
            self.xs[i_].expected = e;
        }

        if !g.is_none() {
            self.xs[i_].got = g;
        }
    }

    pub fn i_to_index(&mut self,i:(usize,usize)) -> usize {
        if i == (0,0) {
            return 0;
        } else if i == (0,1) {
            return 1;
        } else if i == (1,0) {
            return 2;
        } else if i == (1,1) {
            return 3;
        }

        assert!(false);
        4
    }

    pub fn clear_i(&mut self,i:(usize,usize),c:(usize,usize)) {
        assert!(c.0 == 0 || c.0 == 1);
        assert!(c.1 == 0 || c.1 == 1);

        let i_ = self.i_to_index(i);
        let mut q = (self.xs[i_].expected.clone(),self.xs[i_].got.clone());

        if c.0 == 1 {
            q = (None,q.1);
        }

        if c.1 == 1 {
            q = (q.0,None);
        }

        self.xs[i_] = i_mem::build_contrastruct(vec![i.0,i.1],q.0,q.1);
    }

}
