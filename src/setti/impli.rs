//! struct used to generate statements represented as
//! k-set sequences. 
use crate::setti::setc;
use crate::setti::sets;
use crate::setti::impf;
use crate::setti::implif;
use crate::setti::set_gen;
use crate::setti::setf;
use std::collections::{HashMap,HashSet};
use crate::metrice::vcsv;
use ndarray::Array1;

/// TODO: make each initial element an ImpSetSource
///         REQUIRES REFORMATTING OF STRING
/// notation:
/// merging two elements e, e2
///     e + e2 
/// new element x for e
///     e__x
///
/// merge element (e + e2) and (e3 + e4)?
///
/// M = e + e2 + e3 + e4
/// calculating the parents of M?
/// - get the median + 
///
/// M2 = e + e2 + e3 + e4 + e5
/// calculating the parents of M2?
/// CANNOT
///
/// CONCLUSION
/// have to find way to use `parentnot` and `ohop`! 
pub struct Impli {
    /// starting elements
    pub kernel: Vec<String>,
    /// starting existence/implication values for every single element
    /// existence_implication
    pub start_lone_file:String,
    /// start element -> (existence,implication)
    pub kernel_table: HashMap<String,(f32,f32)>,
    /// kernel new element existence value generator
    impfvec_e: Vec<impf::ImpF>,
    /// kernel new element implication value generator
    impfvec_i: Vec<impf::ImpF>, 
    /// filenames for existence/implication function vectors 
    impfvec_fn: Vec<(String,String)>,
    /// used to determine the k-values of  
    kstatement_fn: (impf::ImpFI32,String), 
    // seed size function
    ssf: implif::ImpElementSeedSizeF,
    // used to determine the minumum implication of statement 
    implication_fn: fn(f32,f32,f32,f32,f32,f32) -> f32    
}

pub fn build_Impli(kernel: Vec<String>,start_lone_file:String, impfvec_fn: Vec<(String,String)>,kfn:String,ssf: implif::ImpElementSeedSizeF,ifn:fn(f32,f32,f32,f32,f32,f32) -> f32) -> Impli {
    // make the kernel table
    let q = "src/data/".to_string() + start_lone_file.as_str(); 
    let vseq = vcsv::csv_to_arr1_seq(q.as_str()).unwrap(); 

    let mut kt: HashMap<String,(f32,f32)> = HashMap::new();
    for (i,k) in kernel.iter().enumerate() {
        kt.insert(k.to_string(),(vseq[i][0],vseq[i][1]));
    }

    // declare the 2 vectors of ImpF for existence and implication generation
    let l = kernel.len();
    let mut v1: Vec<impf::ImpF> = Vec::new();
    let mut v2: Vec<impf::ImpF> = Vec::new();

    for i in 0..l {
        // load existence generator
        let ipf1 = impf::load_ImpF(impfvec_fn[i].0.clone().as_str());
        // load implication generator
        let ipf2 = impf::load_ImpF(impfvec_fn[i].1.clone().as_str());

        v1.push(ipf1);
        v2.push(ipf2);
    }

    // load the k-statement and implication generators
    let fk = impf::load_ImpFI32(&kfn);
    Impli{kernel:kernel,start_lone_file:start_lone_file,kernel_table:kt,
        impfvec_e:v1,impfvec_i:v2,impfvec_fn:impfvec_fn,
        kstatement_fn:(fk,kfn),ssf:ssf,implication_fn:ifn}

}