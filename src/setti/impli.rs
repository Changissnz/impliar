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
use crate::setti::strng_srt;
use crate::enci::implie; 
use crate::enci::ohop; 
use ndarray::Array1;

pub struct Impli {
    /// starting elements
    pub kernel: Vec<String>,
    /// kernel new element existence value generator
    impfvec_e: HashMap<String,impf::ImpF>,
    /// kernel new element implication value generator
    impfvec_i: HashMap<String,impf::ImpF>, 
    /// filenames for existence/implication function vectors 
    impfvec_fn: Vec<(String,String)>,
    /// used to determine the k-value of next statement's vectors 
    kstatement_fn: (impf::ImpFI32,String),
    /// used to determine the options ratio
    options_ratio_fn: (impf::ImpF,String),
    /// used to determine the closure of the options for a k-statement
    closure_ratio_fn: (impf::ImpF,String),
    /// seed size function
    ssf: implif::ImpElementSeedSizeF,
    /// used to determine the minumum implication of statement 
    issf : implif::ImpliSSF,
    /// elements -> their <implie::ImpSetSource> counter
    iss_hash: HashMap<String,implie::ImpSetSource>,
    /// output of <impli::Impli> 
    ei_statement: Vec<Vec<Vec<String>>>,
    /// set scores of last statement
    /// sequence of (Vec,f32) pairs. 
    current_set_scores: Vec<(Vec<String>,f32)>

    // TODO: save file

}

/// TODO 
pub fn is_proper_kernel_value(s:String) -> bool {
    false 
}

pub fn build_Impli(kernel: Vec<String>, 
    impfvec_fn: Vec<(String,String)>,kfn:String,ofn:String,cfn:String,
    ssf: implif::ImpElementSeedSizeF,ifn:fn(f32,f32,f32,f32,f32,f32) -> f32,
    ifn2: fn(Vec<f32>) -> f32, ew:f32,iw:f32) -> Impli {
    // TODO: check kernel for proper values
    
    // declare the 2 vectors of ImpF for existence and implication generation
    // and make the kernel table
    let l = kernel.len();
    let mut v1: HashMap<String,impf::ImpF> = HashMap::new();
    let mut v2: HashMap<String,impf::ImpF> = HashMap::new();
    let mut kt: HashMap<String,(f32,f32)> = HashMap::new();
    for i in 0..l {
        // load existence generator
        let ipf1 = impf::load_ImpF(impfvec_fn[i].0.clone().as_str());
        // load implication generator
        let ipf2 = impf::load_ImpF(impfvec_fn[i].1.clone().as_str());
        kt.insert(kernel[i].clone(),(ipf1.v,ipf2.v)); 
        v1.insert(kernel[i].clone(),ipf1); 
        v2.insert(kernel[i].clone(),ipf2);
    }

    // load the k-statement generator
    let fk = impf::load_ImpFI32(&kfn);

    // load the options-ratio generator
    let fo = impf::load_ImpF(&ofn);

    // load the closure ratio generator
    let fc = impf::load_ImpF(&cfn); 

    // load the ImpliSSF function
    let issf = implif::build_ImpliSSF(ifn,ifn2,kt.clone(),ew,iw); 
    
    // declare a ImpSetSource for each kernel element
    let mut issv: HashMap<String,implie::ImpSetSource> = HashMap::new();
    for k in kernel.iter() {
        let ie = implie::build_ImpSetSource(k.to_string());
        issv.insert(k.to_string(),ie);
    }
    
    Impli{kernel:kernel,
        impfvec_e:v1,impfvec_i:v2,impfvec_fn:impfvec_fn,
        kstatement_fn:(fk,kfn), options_ratio_fn:(fo,ofn), closure_ratio_fn:(fc,cfn),
        ssf:ssf,issf:issf,iss_hash:issv,ei_statement:Vec::new(),current_set_scores:Vec::new()}
}


///////// implication goes here

impl Impli {

    /// # description
    pub fn output_statement(&mut self,verbose:bool) {

        // fetch available options
        let o = self.gather_options(verbose);

        // fetch k
        let mut k = self.kstatement_fn.0.next() as usize; 
        k = vec![k,self.iss_hash.len()].into_iter().min().unwrap(); 

        // calculate (n,k)
        let m = setc::nCr(o.len(),k);

        // fetch closure
        let c = self.closure_ratio_fn.0.next();

        // calculate required number of k-vectors
        let rn = (c * m as f32).round() as usize; 

        // fetch the k-vectors
        self.gather_k_vectors(o,rn,k);

        // gather new elements from previous k-statement
        self.gather_new_elements();
    
    }

    /// # description
    /// calculates new elements from k-statement to be loaded. 
    pub fn gather_new_elements(&mut self) {
        let l = self.current_set_scores.len();
        if l == 0 {
            return ;
        }

        let mut s:Vec<Vec<String>> = Vec::new();
        for l_ in self.current_set_scores.clone().iter() {
            self.add_new_element((*l_).0.clone(),(*l_).1.clone()); 
            s.push((*l_).0.clone()); 
        }

        // clear 
        self.current_set_scores = Vec::new();
        self.ei_statement.push(s); 
    }

    /// # description
    /// creates a new <implie::ImpSetSource> for `s`
    /// and adds `s` to <impf::ImpliSSF> 
    pub fn add_new_element(&mut self,s:Vec<String>,f:f32) {

        // get new element size
        let q = self.ssf.size(f,s.len() as i32);

        // case: no new elements
        if q == 0 {
            return;
        }

        // check if s is an element
        let x = self.strvec_to_impsetsource(s.clone());
        let stat = self.iss_hash.contains_key(&(x.idn)); 

        // case: make new ImpSetSource for `s`
        if !stat {
            self.iss_hash.insert(x.idn.clone(),x.clone()); 
        }
        
        for _ in 0..q {
            let vx = self.iss_hash.get_mut(&(x.idn)).unwrap().increment();
                        
            // insert new ImpSetSource for new element
            let vxq = vx.idn.clone(); 
            self.iss_hash.insert(vxq.clone(),vx); 

            // insert the new element's existence and implication 
            let (es_,is_) = self.ei_of_new_element(vxq.clone()); 
            self.issf.ht.insert(vxq,(es_,is_));   
        }
    }

    /// # description
    /// gather all k-vectors (stringized) and their <impli::ImpliSSF.f>
    /// scores. 
    pub fn gather_k_vectors(&mut self,o: Vec<String>,rn:usize,k:usize) {

        // gather elements
        let mut v: Vec<Vec<String>> = Vec::new();
        let l = o.len() - k;
        for i in 0..l {
            let x = set_gen::fcollect_vec(o.clone(),i,k);
            let rem = rn as i32 - v.len() as i32;
            if rem <= 0 {
                break;
            }
            v.extend(x[0..rem as usize].to_vec()); 
        }

        // reformat each vec into a impsetsource and calculate its score 
        self.current_set_scores = Vec::new();
        for v_ in v.iter() {
            let s = self.issf.apply2((*v_).clone());
            self.current_set_scores.push(((*v_).clone(),s));
        }
    }

    /// # description
    /// converts a vector of strings to an <implie::ImpSetSource> 
    pub fn strvec_to_impsetsource(&mut self,v:Vec<String>) -> implie::ImpSetSource {
        let l = v.len();
        assert!(l > 0);

        let mut x = self.iss_hash.get_mut(&v[0]).unwrap().clone();
        for i in 1..l {
            x = x + self.iss_hash.get_mut(&v[i]).unwrap().clone()
        }
        x
    }

    /// # description
    /// sorted elements by score of <implif::ImpliSSF.f>, descending order
    pub fn gather_options(&mut self,verbose:bool) -> Vec<String> {

        let mut v: Vec<(String,f32)> = Vec::new();
        for k in self.issf.ht.clone().into_keys() {
            let f = self.issf.apply(k.clone());
            if verbose {
                println!("{:?}\t{:?}",k.clone(),f);
            }
            v.push((k.clone(),f));
        }
        
        v.sort_by(strng_srt::str_cmp4);

        // get the number of elements to consider based on 
        let o = self.options_ratio_fn.0.next();
        let o_ = (o * v.len() as f32).round() as usize;
        v = v[0..o_].to_vec(); 
        v.into_iter().map(|x| x.0).collect() 
    }

    /// # description
    /// calculates the (existence,implication) pair of new element with idn `stridn`
    pub fn ei_of_new_element(&mut self,stridn:String) -> (f32,f32) {
        // parse `stridn`
        let mut x2 = ohop::build_order_of_operator(stridn);
        x2.process();
        let cf = ohop::parse_OrderOfOperator__comma_format(&mut x2,ohop::str_alphabebetical_filter);
        self.ei_pair_by_generator_sequence(cf)  
    }

    pub fn ei_pair_by_generator_sequence(&mut self,v:Vec<String>) -> (f32,f32) {
        let (mut e,mut i): (f32,f32) = (0.,0.);
        let l = v.len();
        if l == 0 {
            return (0.,0.);
        }

        for v_ in v.iter() {
            let e_ = self.impfvec_e.get_mut(v_).unwrap().next(); 
            let i_ = self.impfvec_i.get_mut(v_).unwrap().next(); 
            e += e_;
            i += i_;
        }
        
        (e / l as f32,i / l as f32) 
    }
}


///////// end implication here

pub fn sample_Impli_1() -> Impli {
    let k = vec!["abasco".to_string(),"batanya".to_string(),"ourototvos".to_string(),
                "resteni".to_string()];
    let ifns = vec![("ife1".to_string(),"ifi1".to_string()),
            ("ife2".to_string(),"ifi2".to_string()),
            ("ife3".to_string(),"ifi3".to_string()),
            ("ife4".to_string(),"ifi4".to_string())];

    let iessf = implif::build_ImpElementSeedSizeF(1.,0.3);
    build_Impli(k,ifns,
        "ifk1".to_string(),"ifo".to_string(),"ifc".to_string(),
        iessf,implif::impli_element_score_1,implif::impli_set_score_1,1.,1.)
}