//! file contains test cases for btchcorrctr
use crate::enci::{skew,skewf32};
use crate::setti::dessi;
use ndarray::{arr1,Array1};

pub fn batch_1() -> (Vec<skewf32::SkewF32>,Vec<Array1<f32>>) {

    // scale by k = 5
    let k:usize = 5;

    let r1 = arr1(&[3.513,4.221,5.4646,6.88,20.5]);
    let r2 = arr1(&[1.,2.222,3.001,4.5]);
    let r3 = arr1(&[30.30303,16.5,25.1]);

    let s1 = arr1(&[15.,16.,22.2,7.1,14.]);
    let s2 = arr1(&[5.,6.5,10.5,9.5]);
    let s3 = arr1(&[80.1,50.3,67.5]);
    let sk1 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s1,k)),
                   None,vec![2],None);
   let sk2 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s2,k)),
                  None,vec![2],None);
   let sk3 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s3,k)),
                 None,vec![2],None);
   let s1k = skewf32::SkewF32{sk:sk1,s:k};
   let s2k = skewf32::SkewF32{sk:sk2,s:k};
   let s3k = skewf32::SkewF32{sk:sk3,s:k};

    (vec![s1k,s2k,s3k],vec![r1,r2,r3])
}

pub fn batch_2() -> (Vec<skewf32::SkewF32>,Vec<Array1<f32>>) {
   let k:usize = 5;

   let r1 = arr1(&[2.,4.,2.,4.]);
   let r2 = arr1(&[5.,7.,82.]);
   let r3 = arr1(&[12.,35.,83.]);
   let r4 = arr1(&[1.,21.]);

   let s1 = arr1(&[4.,16.,4.,16.]);
   let s2 = arr1(&[12.,35.,83.]);
   let s3 = arr1(&[70.,150.,20.]);
   let s4 = arr1(&[21.,84.7]);

   let sk1 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s1,k)),
                  None,vec![2],None);
   let sk2 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s2,k)),
                 None,vec![2],None);
   let sk3 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s3,k)),
                None,vec![2],None);
   let sk4 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s4,k)),
      None,vec![2],None);

   let s1k = skewf32::SkewF32{sk:sk1,s:k};
   let s2k = skewf32::SkewF32{sk:sk2,s:k};
   let s3k = skewf32::SkewF32{sk:sk3,s:k};
   let s4k = skewf32::SkewF32{sk:sk4,s:k};

   (vec![s1k,s2k,s3k,s4k],vec![r1,r2,r3,r4])
}

pub fn batch_3() -> (Vec<skewf32::SkewF32>,Vec<Array1<f32>>) {

   // scale by k = 5
   let k:usize = 5;
   let s1 = arr1(&[1.,2.,1.,2.,1.]);
   let s2 = arr1(&[1.,2.,1.,2.,1.]);
   let s3 = arr1(&[1.,2.,1.,2.,1.]);

   let r1 = arr1(&[2.,3.,2.,3.,2.]);
   let r2 = arr1(&[2.,3.,2.,3.,2.]);
   let r3 = arr1(&[2.,3.,2.,3.,2.]);


   let sk1 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s1,k)),
                  None,vec![2],None);
   let sk2 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s2,k)),
                 None,vec![2],None);
   let sk3 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s3,k)),
                None,vec![2],None);
   let s1k = skewf32::SkewF32{sk:sk1,s:k};
   let s2k = skewf32::SkewF32{sk:sk2,s:k};
   let s3k = skewf32::SkewF32{sk:sk3,s:k};
   (vec![s1k,s2k,s3k],vec![r1,r2,r3])
}

pub fn batch_4() -> (Vec<skewf32::SkewF32>,Vec<Array1<f32>>) {

   // scale by k = 5
   let k:usize = 5;
   let s1 = arr1(&[10.,12.,10.,12.,10.]);
   let s2 = arr1(&[10.,12.,10.,12.,10.]);
   let s3 = arr1(&[10.,12.,10.,12.,10.]);

   let r1 = arr1(&[2.,3.,2.,3.,2.]);
   let r2 = arr1(&[2.,3.,2.,3.,2.]);
   let r3 = arr1(&[2.,3.,2.,3.,2.]);


   let sk1 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s1,k)),
                  None,vec![2],None);
   let sk2 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s2,k)),
                 None,vec![2],None);
   let sk3 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s3,k)),
                None,vec![2],None);
   let s1k = skewf32::SkewF32{sk:sk1,s:k};
   let s2k = skewf32::SkewF32{sk:sk2,s:k};
   let s3k = skewf32::SkewF32{sk:sk3,s:k};

   (vec![s1k,s2k,s3k],vec![r1,r2,r3])
}

pub fn batch_5() -> (Vec<skewf32::SkewF32>,Vec<Array1<f32>>) {

   // scale by k = 5
   let k:usize = 5;
   let s1 = arr1(&[10.,12.,10.,12.,10.]);
   let s2 = arr1(&[10.,12.,18.,21.,10.]);
   let s3 = arr1(&[10.,30.,16.,12.,14.]);
 
   let r1 = arr1(&[2.,1.,2.,3.,2.]);
   let r2 = arr1(&[2.,6.,2.,3.,2.]);
   let r3 = arr1(&[10.,3.,2.,30.,2.]);
 
 
   let sk1 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s1,k)),
                   None,vec![2],None);
   let sk2 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s2,k)),
                  None,vec![2],None);
   let sk3 = skew::build_skew(None,None,Some(dessi::scale_arr1_f32_to_arr1_i32(s3,k)),
                 None,vec![2],None);
   let s1k = skewf32::SkewF32{sk:sk1,s:k};
   let s2k = skewf32::SkewF32{sk:sk2,s:k};
   let s3k = skewf32::SkewF32{sk:sk3,s:k};
 
   (vec![s1k,s2k,s3k],vec![r1,r2,r3])
 }
