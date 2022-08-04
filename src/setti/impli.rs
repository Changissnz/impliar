use crate::setti::setc;
use crate::setti::sets;
use crate::setti::impf;
use crate::setti::set_gen;
use crate::setti::setf;
use std::collections::HashMap;
use crate::metrice::vcsv;
use ndarray::Array1;

/*
CAUTION: kernel allows for duplicates
*/
pub struct Impli {
    pub kernel: Vec<String>,

    // kernel correspondence file
    pub kcf: String,
    // stringized kernel vec -> f32
    kchm: HashMap<String,f32>,

    // implication chain file
    pub icf: String,
    ifs: impf::ImpFSeq 
}

pub fn build_Impli(kernel: Vec<String>,kcf: String,icf:String) -> Impli {
    let vv:Vec<f32> = Array1::ones(kernel.len()).into_iter().collect();
    let qr = (0.,1.);
    let ifs = impf::file_to_ImpFSeq(&icf,vv,qr);
    Impli{kernel:kernel,kcf:kcf,kchm:HashMap::new(),icf:icf,ifs:ifs}
}

impl Impli {

    //// init functions
    pub fn init_kcf(&mut self) {
        
        // open kcf file and read
        let a1 = vcsv::csv_to_arr1(&self.kcf).unwrap();

        // generate all possible 2-vecs
        let l = self.kernel.len() - 1;
        let mut d: Vec<Vec<String>> = Vec::new();
        for i in 0..l {
            let v = set_gen::fcollect_vec(self.kernel.clone(),i, 2);
            d.extend(v);
        }

        // add (k,v) pairs to correspondence hash map
        let dl = d.len();
        assert_eq!(dl,a1.len());

        for i in 0..dl {
            let vs = setf::vec_to_str(d[i].clone());
            self.kchm.insert(vs,a1[i]);
        }
    }

    ////////////////////////////////////

    pub fn source_of_choice(&mut self) {

    }

    pub fn next_src(&mut self) {

    }

    pub fn output_layer(&mut self) -> Vec<Vec<String>> {
        Vec::new()
    }

}


///////////////////////////////////////////////////
/////// used to serialize and deserialize data  
/*
use serde::{Serialize, Deserialize};

// here the "magic" conversion is generated
#[derive(Debug, Serialize, Deserialize)]
struct T {
    i: i32,
    f: f64,
}
*/ 