//! struct used to generate statements represented as
//! k-set sequences. 
use crate::setti::setc;
use crate::setti::sets;
use crate::setti::impf;
use crate::setti::implif;
use crate::setti::set_gen;
use crate::setti::setf;
use crate::setti::setf::Count;
use std::collections::{HashMap,HashSet};
use crate::metrice::vcsv;
use crate::setti::strng_srt;
use crate::enci::implie; 
use crate::enci::ohop; 
use ndarray::Array1;

pub struct EIStatement {
    pub statement: Vec<Vec<Vec<String>>>,
    pub statement_options:Vec<Vec<String>>,
    pub statement_closure: Vec<f32>,
    pub ei_scores: Vec<Vec<f32>>
}

pub fn build_new_EIStatement() -> EIStatement {
    EIStatement{statement: Vec::new(), statement_options: Vec::new(),
        statement_closure:Vec::new(),ei_scores:Vec::new()}
}

impl EIStatement {

    pub fn summarize_at(&mut self,i: usize) {
        let n1 = self.statement[i].len(); 
        let n2 = self.statement_options[i].len(); 
        let n3 = self.statement_closure[i].clone();
        let n4 = self.statement[i][0].len();

        println!("- statement info @ {}",i);
        println!("* number of nodes: {}",n1);
        println!("* number of options: {}",n2);
        println!("* closure rate: {}",n3);
        println!("* k value: {}",n4);
        println!("---------------------------");
    } 
}

/// Struct used for implication structure <impli::Impli>.
/// 
/// # terminology
/// Generates a sequence of k-statements from an initial
/// kernel of elements. The sequence is called the `ei_statement`. 
/// 
/// Each k-statement is a sequence of k-vectors (called k-nodes),
/// and k is an arbitrary usize less than or equal to the number of
/// total unique elements.
/// The unique elements of <impli::Impli> are the kernel 
/// elements and the elements generated from qualifying k-nodes
/// in each generated k-statement.
///
/// Uses the following generators to aid in generating the next k-statement:
/// - k-value generator (i32): determine the value of k for the next k-statement. 
/// - options ratio generator (f32): in range \[0,1\], determine the total number of
/// unique elements to consider for k-statement.
/// - closure ratio generator (f32): in range \[0,1\], determine the ratio of total number of
/// combinations of `nCr`, where `n` is the number of options and `r` is the k value.
/// - seed size generator (i32): determines the number of new elements generated from
/// each k-node in the previous k-statement.
/// 
/// # algorithm description
/// for a k-statement, do the following: 
/// - fetch options ratio from generator. 
/// - rank unique elements of <impli::Impli> according to the
/// function of <implif::ImpliSSf>, and fetch those x number of
/// elements according to options ratio.
/// - fetch k-value from generator.
/// - fetch closure ratio from generator.
/// - fetch the appropriate number of combinations according to the
///   `nCr` and closure ratio values, which constitutes the
///     ei-statement. 
/// - update <implif::ImpliSSF> according to the generated ei-statement.
///
/// # NOTE:
/// maximum number of options set at 10 due to computation issue. 
pub struct Impli {
    /// starting elements
    pub kernel: Vec<String>,
    /// kernel new element existence value generator
    pub impfvec_e: HashMap<String,impf::ImpF>,
    /// kernel new element implication value generator
    pub impfvec_i: HashMap<String,impf::ImpF>, 
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
    pub ei_statement: EIStatement,
    
    /// set scores of last statement
    /// sequence of (Vec,f32) pairs. 
    current_set_scores: Vec<(Vec<String>,f32)>
}

pub fn is_proper_kernel_value(s:String) -> bool {

    for c in s.chars() {
        if char::is_numeric(c) {
            return false;
        }

        if c == '_' || c == ',' {
            return false;
        }
    }

    true 
}

pub fn build_Impli(kernel: Vec<String>, 
    impfvec_fn: Vec<(String,String)>,kfn:String,ofn:String,cfn:String,
    ssf: implif::ImpElementSeedSizeF,ifn:fn(f32,f32,f32,f32,f32,f32) -> f32,
    ifn2: fn(Vec<f32>) -> f32, ew:f32,iw:f32) -> Impli {
    // TODO: check kernel for proper values
    for k in kernel.iter() {
        assert!(is_proper_kernel_value((*k).clone())); 
    }
    
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
        ssf:ssf,issf:issf,iss_hash:issv,
        ei_statement: build_new_EIStatement(),current_set_scores:Vec::new()}
}

impl Impli {

    /// # description
    /// outputs the first `n` (existence,implication) values from
    /// the generator of element `k`. 
    pub fn output_ei_generated(&mut self,k:String,n: usize) -> Vec<(f32,f32)> {
        let mut v: Vec<(f32,f32)> = Vec::new();
        for i in 0..n {
            let f1 = self.impfvec_e.get_mut(&k).unwrap().next();
            let f2 = self.impfvec_i.get_mut(&k).unwrap().next();
            v.push((f1,f2));
        }
        v
    }

    /// # description
    /// outputs the first `n` values from the k-value generator. 
    pub fn output_k_generated(&mut self,n:usize) -> Vec<i32> {
        (0..n).into_iter().map(|x| self.kstatement_fn.0.next()).collect()
    }

    /// # description
    /// outputs the first `n` values from the options-ratio generator. 
    pub fn output_options_ratio_generated(&mut self,n:usize) -> Vec<f32> {
        (0..n).into_iter().map(|x| self.options_ratio_fn.0.next()).collect()
    }

    /// # description
    /// outputs the first `n` values from the closure-ratio generator. 
    pub fn output_closure_ratio_generated(&mut self,n:usize) -> Vec<f32> {
        (0..n).into_iter().map(|x| self.closure_ratio_fn.0.next()).collect() 
    }

    /// # description
    /// calculates the next ei-statement. 
    pub fn output_statement(&mut self,verbose:bool) {

        // fetch available options
        let o = self.gather_options(verbose);

        // fetch k
        let mut k = self.kstatement_fn.0.next() as usize; 
        k = vec![k,o.len()].into_iter().min().unwrap(); 

        // calculate (n,k)
        let m = setc::nCr(o.len(),k);

        // fetch closure
        let c = self.closure_ratio_fn.0.next();
        if verbose {
            println!("closure ratio for {} elements: {}",m,c);
        }

        // calculate required number of k-vectors
        let rn = (c * m as f32).round() as usize; 

        // fetch the k-vectors
        self.gather_k_vectors(o.clone(),rn,k,verbose);

        // gather new elements from previous k-statement
        self.gather_new_elements(c,o,verbose);
    }

    /// # description
    /// calculates new elements from k-statement to be loaded, and
    /// updates `ie_statement`.  
    pub fn gather_new_elements(&mut self,c: f32, o:Vec<String>,verbose:bool) {
        let l = self.current_set_scores.len();
        if l == 0 {
            return ;
        }

        let mut s:Vec<Vec<String>> = Vec::new();
        let mut x2: Vec<f32> = Vec::new();
        
        for l_ in self.current_set_scores.clone().iter() {
            self.add_new_element((*l_).0.clone(),(*l_).1.clone(),verbose); 
            s.push((*l_).0.clone());

            for lx in (*l_).0.iter() {
                self.issf.update_element((*lx).clone(),None);
            }

            x2.push((*l_).1.clone());
        }

        self.ei_statement.statement.push(s);
        self.ei_statement.statement_options.push(o);
        self.ei_statement.statement_closure.push(c);
        self.ei_statement.ei_scores.push(x2);

        // clear 
        self.current_set_scores = Vec::new();
    }

    /// # description
    /// creates a new <implie::ImpSetSource> for `s`
    /// and adds `s` to <impf::ImpliSSF> 
    pub fn add_new_element(&mut self,s:Vec<String>,f:f32,verbose:bool) {

        // get new element size
        let q = self.ssf.size(f,s.len() as i32);
        let x = self.strvec_to_impsetsource(s.clone());

        if verbose {
            println!("* gathering new elements for {}: {}",x.idn.clone(),q);
        }

        // case: no new elements
        if q == 0 {
            return;
        }

        // check if s is an element
        let stat = self.iss_hash.contains_key(&(x.idn)); 

        // case: make new ImpSetSource for `s`
        if !stat {
            if verbose {
                println!("\t\t* new key")
            }

            self.iss_hash.insert(x.idn.clone(),x.clone()); 
        }
        
        for _ in 0..q {
            let vx = self.iss_hash.get_mut(&(x.idn)).unwrap().increment();
                        
            // insert new ImpSetSource for new element
            let vxq = vx.idn.clone(); 
            self.iss_hash.insert(vxq.clone(),vx); 

            // insert the new element's existence and implication 
            let (es_,is_) = self.ei_of_new_element(vxq.clone()); 
            
            if verbose {
                println!("\t- e {},i {}",es_.clone(),is_.clone());
            }

            self.issf.update_element(vxq.clone(),Some((es_,is_)));
        }
    }

    /// # description
    /// gather all k-vectors (stringized) and their <impli::ImpliSSF.f>
    /// scores. 
    pub fn gather_k_vectors(&mut self,o: Vec<String>,rn:usize,k:usize,verbose:bool) {
        if verbose {
            println!("$ gathering k-vectors of size {}",rn);
        }
        
        self.current_set_scores = Vec::new();
        if k == 0 {
            return; 
        }
        // gather elements
        let mut v: Vec<Vec<String>> = Vec::new();
        let l = o.len() - k + 1;
        for i in 0..l {
            let x = set_gen::fcollect_vec(o.clone(),i,k);
            let mut rem = rn as i32 - v.len() as i32;
            if rem <= 0 {
                break;
            }
            rem = vec![rem,x.len() as i32].into_iter().min().unwrap();
            v.extend(x[0..rem as usize].to_vec()); 
        }

        // reformat each vec into a impsetsource and calculate its score 
        for v_ in v.iter() {
            let s = self.issf.apply2((*v_).clone());
            self.current_set_scores.push(((*v_).clone(),s));
            if verbose {
                println!("\tstatement:\n{:?}\n\tscore: {}",v_,s);
            }
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
        let o_ = vec![(o * v.len() as f32).round() as usize,10].into_iter().min().unwrap();

        if verbose {
            println!("number of options for {}: {}",v.len(),o_);
        }

        v = v[0..o_].to_vec();
        v.into_iter().map(|x| x.0).collect() 
    }

    /// # description
    /// calculates the (existence,implication) pair of new element with idn `stridn`
    pub fn ei_of_new_element(&mut self,stridn:String) -> (f32,f32) {
        // parse `stridn`
        let mut x2 = ohop::build_order_of_operator(stridn.clone());
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
        "ifk1".to_string(),"ifo2".to_string(),"ifc".to_string(),
        iessf,implif::impli_element_score_1,implif::impli_set_score_1,1.,1.)
}