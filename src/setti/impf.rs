use crate::metrice::vcsv; 
use ndarray::Array1;

extern crate round;
use round::{round};

/*
structs in file used for constructing modular chain functions
for struct<Impli> and storing them in files
*/
pub struct ImpF {
    // multiples
    m: Array1<f32>,
    // current value
    v: f32,
    // current index
    i:usize,
    // min,max range of v
    qr: (f32,f32)
}

impl ImpF {

    pub fn next(&mut self) -> f32 {
        let l = self.m.len();
        self.i = self.i % self.m.len();
        self.v *= self.m[self.i].clone();
        if self.v < self.qr.0 {
            self.v = self.qr.0.clone();
        }

        if self.v > self.qr.1 {
            self.v = self.qr.1.clone();
        }

        self.i += 1;
        self.v.clone()
    }

    pub fn to_file(&mut self,filename: &str,file_mode:&str) {
        let q = vec![self.m.clone()];
        vcsv::arr1_seq_to_csv(q,filename,file_mode);
    }

}


/////////////////////////////////////////////////////////////

pub struct ImpFSeq {
    iv: Vec<ImpF>
}

impl ImpFSeq {

    pub fn next(&mut self,i:usize) -> f32 {
        self.iv[i].next()
    }

    pub fn to_file(&mut self,filename: &str) {
        let l = self.iv.len();
        if l == 0 {
            return; 
        }

        // write the first one
        self.iv[0].to_file(filename,"w");

        // append the rest
        for i in 1..l {
            self.iv[i].to_file(filename,"a");
        }
    }
}

pub fn build_ImpF(m: Array1<f32>,v:f32,qr:(f32,f32)) -> ImpF {
    ImpF{m:m,v:v,i:0,qr:qr}
}

pub fn file_to_ImpFSeq(f: &str,vv:Vec<f32>,qr:(f32,f32)) -> ImpFSeq {
    let s = vcsv::csv_to_arr1_seq(f).unwrap();
    let l = vv.len();
    assert!(s.len() == l);
    let mut d: Vec<ImpF> = Vec::new(); 
    for i in 0..l {
        let f = build_ImpF(s[i].clone(),vv[i],qr.clone());
        d.push(f);
    }

    ImpFSeq{iv:d}
}  

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__file_to_ImpFSeq() {
        let f = "src/data/i_f1.csv";
        let vv = vec![0.2,0.4,0.5,0.6];
        let qr = (0.,1.);
        let ifs = file_to_ImpFSeq(f,vv,qr);
    }

    #[test]
    pub fn test__ImpFSeq__next() {
        let f = "src/data/i_f1.csv";
        let vv = vec![0.2,0.4,0.5,0.6];
        let qr = (0.,1.);
        let mut ifs = file_to_ImpFSeq(f,vv,qr);
        
        let mut res:Vec<f32> = Vec::new();
        for i in 0..4 {
            let q = round(ifs.next(i) as f64,5) as f32;
            res.push(q); 
        }
        assert_eq!(res,vec![0.6, 0.2, 1.0, 0.48]);
    }
}