//! structs used for constructing modular chain functions
//! for struct<Impli> and storing them in files
use crate::metrice::vcsv; 
use crate::setti::strng_srt;
use ndarray::Array1;
extern crate round;
use round::{round};
extern crate savefile;
extern crate savefile_derive;
use savefile::prelude::*;

pub fn save_ImpF(impf:&ImpF,bin_name:&str) {
    save_file(("src/data/".to_string() + bin_name).as_str(),0,impf).unwrap();
}

pub fn load_ImpF(bin_name:&str) -> ImpF {
    load_file(("src/data/".to_string() + bin_name).as_str(), 0).unwrap()
}

pub fn save_ImpFI32(impf:&ImpFI32,bin_name:&str) {
    save_file(("src/data/".to_string() + bin_name).as_str(),0,&impf.i).unwrap();
}

pub fn load_ImpFI32(bin_name:&str) -> ImpFI32 {
    let impf = load_file(("src/data/".to_string() + bin_name).as_str(), 0).unwrap();
    build_ImpFI32(impf) 
}

/// struct used for modular transformation of f32
#[derive(Savefile)]
pub struct ImpF {
    /// multiples of length n
    m: Vec<f32>,
    /// adders of length n 
    a: Vec<f32>,
    /// current value
    pub v: f32,
    /// current index
    i:usize,
    /// min,max range of v
    qr: (f32,f32)
}

pub fn build_ImpF(m:Vec<f32>,a:Vec<f32>,v:f32,i:usize,qr:(f32,f32)) -> ImpF {
    ImpF{m:m,a:a,v:v,i:i,qr:qr} 
}

impl ImpF {

    pub fn next(&mut self) -> f32 {
        
        let l = self.m.len();
        self.i = self.i % self.m.len();
        self.v = self.v * self.m[self.i].clone() + self.a[self.i].clone();
        self.v = strng_srt::f32_mod_in_range(self.v,self.qr.clone());
        self.i += 1;
        self.v.clone()
    }
}

/// i32 version of ImpF
#[derive(Savefile)]
pub struct ImpFI32 {
    i : ImpF,
}

pub fn build_ImpFI32(i: ImpF) -> ImpFI32 {
    ImpFI32{i:i}
}

impl ImpFI32 {

    pub fn next(&mut self) -> i32 {
        let n = self.i.next();
        n.round() as i32
    }
}


pub fn sample_ImpFI32_save_to_file() {
    let i1 = build_ImpF(vec![0.5,2.,0.5,2.],vec![0.,0.1,0.,0.1],1000.,0,(3.,8.));
    let i = build_ImpFI32(i1);
    save_ImpFI32(&i,"ifk1");
}

/// # description
/// sample ImpF vec used for existence values of <impli::Impli> kernel size of 4;
/// non-uniform generators. 
pub fn sample_ImpF_vec_e_1_save_to_file(base_fp: String) {
    let i1 = build_ImpF(vec![0.5,2.,0.25,4.],vec![0.2,0.1,0.15,0.05],0.4,0,(0.,2.));
    let i2 = build_ImpF(vec![0.5,1.5],vec![0.2,-0.1],0.7,0,(0.,2.));
    let i3 = build_ImpF(vec![0.5,1.,1.,2.,0.5],vec![0.,0.,0.,0.,0.],1.,0,(0.,2.));
    let i4 = build_ImpF(vec![2.,0.25,1.],vec![0.5,0.1,-0.2],0.4,0,(0.,2.));
    
    save_ImpF(&i1,(base_fp.clone() + "1").as_str());
    save_ImpF(&i2,(base_fp.clone() + "2").as_str());
    save_ImpF(&i3,(base_fp.clone() + "3").as_str());
    save_ImpF(&i4,(base_fp.clone() + "4").as_str());
}

/// # description
/// sample ImpF vec used for existence values of <impli::Impli> kernel size of 4;
/// non-uniform generators. 
pub fn sample_ImpF_vec_i_1_save_to_file(base_fp: String) {
    let i1 = build_ImpF(vec![1.],vec![1.],0.5,0,(0.,2.));
   
    save_ImpF(&i1,(base_fp.clone() + "1").as_str());
    save_ImpF(&i1,(base_fp.clone() + "2").as_str());
    save_ImpF(&i1,(base_fp.clone() + "3").as_str());
    save_ImpF(&i1,(base_fp.clone() + "4").as_str());
}

pub fn sample_ImpF_options_ratio_save_to_file(base_fp:String) {
    let i1 = build_ImpF(vec![0.4,4.,0.1,0.2],vec![0.2,0.5,-0.5,0.05],0.2,0,(0.,1.));
    save_ImpF(&i1,&base_fp);
}

pub fn sample_ImpF_options_ratio_save_to_file2(base_fp:String) {
    let i1 = build_ImpF(vec![1.],vec![0.],1.0,0,(0.,1.));
    save_ImpF(&i1,&base_fp);
}

pub fn sample_ImpF_closure_ratio_save_to_file(base_fp:String) {
    let i1 = build_ImpF(vec![1.],vec![0.],1.,0,(0.,1.));
    save_ImpF(&i1,&base_fp);
}

