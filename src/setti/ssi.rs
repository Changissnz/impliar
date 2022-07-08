/*
partitioned vector iterator
*/
use ndarray::Array1;

pub struct SSI {
    pub v: Vec<usize>,
    k:usize,
    finished:bool,
    init:bool
}

pub fn build_SSI(vl:usize,k:usize) -> SSI {
    let v:Vec<usize> = Array1::zeros(vl).into_iter().collect();
    SSI{v:v,k:k,finished:false,init:true}
}

impl SSI {

    /*
    increments from index 0 and carries over to indices 1..|v|
    */
    pub fn next_(&mut self,i:usize) -> bool {
        assert!(i < self.v.len());

        self.v[i] = self.v[i] + 1;
        if self.v[i] >= self.k {
            self.v[i] = 0;
            return false;
        }
        true
    }

    pub fn is_finished(&mut self) -> bool {
        if self.init { return false;}

        let v:Vec<usize> = Array1::zeros(self.v.len()).into_iter().collect();
        self.v == v
    }

    pub fn next(&mut self) -> Vec<usize> {
        println!("---");
        let mut i: usize = 0;
        while !self.next_(i) {
            i += 1;
            if i == self.v.len() {break;}
            /*
            println!("V");
            if self.is_finished() {
                break;
            }
            */
        }
        self.init = false;
        self.v.clone()
    }

}
