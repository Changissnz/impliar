/*
demonstrates the randomness of a sequence of values;
NOT  a conclusive test
*/
use crate::metrice::fc;
use crate::enci::ns;
use crate::setti::matrixf;
use crate::setti::ssi;
use ndarray::{Array1,Array2};
use round::{round_up};

pub fn default_FrqncCl(v:Vec<f32>,r:(f32,f32)) -> fc::FrqncCl {
    let l = v.len();
    let mut f = fc::build_FrqncCl(v,r,l);
    f.count_pc();
    f.analyze();
    f
}

/*
one randomness test that uses VectorCounter, FrqncCl
on one of the types i32,f32
*/
pub struct RTest1<T>{
    pub s: Box<dyn ns::RN<T>>,// the random seed w next function
    r: (f32,f32), // ?????????
    n: usize, // number of samples, size per sample
    pub data: Array2<f32>, // all the samples collected
    data_config: Option<Array2<f32>>,
    slide_rate: usize,
    slide_index: ssi::SSI,

    // if FrqncCl.analysis falls within this range, print the
    // corresponding data configuration and its analysis
    alert_range: (f32,f32)
}

pub fn build_RTest1<T>(s: Box<dyn ns::RN<T>>,r:(f32,f32),n:usize,slide_rate:usize,ar:(f32,f32))
    -> RTest1<T>
    where T: Clone + Default
     {
    assert!(slide_rate < n);
    assert!(ar.0 < ar.1);

    let mut d:Array2<f32> = Array2::default((n,n));
    let si:Vec<usize> = Array1::zeros(n).into_iter().collect();

    //let q = round_up((n as f32 / slide_rate as f32).into(),0) as usize;
    //println!("Q: {}",q);
    let si: ssi::SSI = ssi::build_SSI(n,slide_rate + 1);
    RTest1{s:s,r:r,n:n,data:d,slide_rate:slide_rate,slide_index:si,alert_range:ar,data_config:None}
}

impl<T> RTest1<T>
where T: Into<f64>{

    pub fn run(&mut self) {
        self.initial_data_load();
        self.full_config_scan();
    }

    pub fn one_sample(&mut self) -> Vec<f64> {
        let mut x: Vec<f64> = Vec::new();

        for i in 0..self.n {
            x.push(self.s.r_next().into());//f64::from(self.s.r_next()) as f32);
        }
        x
    }

    pub fn add_one_sample(&mut self,i:usize) -> f32 {
        let s1:Vec<f32> = self.one_sample().into_iter().map(|x| x as f32).collect();
        let frc = default_FrqncCl(s1.clone(),self.r.clone());

        // add sample
        let mut s2: Array1<f32> = s1.into_iter().collect();
        matrixf::replace_vec_in_arr2(&mut self.data,&mut s2,i,true);
        frc.analysis
    }

    pub fn initial_data_load(&mut self) {
        for i in 0..self.n {
            let f = self.add_one_sample(i);
            self.alert_on_random(f,Some(i),None);
        }
    }

    pub fn full_config_scan(&mut self) {
        let mut st:bool = true;
        self.data_config = Some(self.data.clone());
        while st {
            self.column_scan();
            st = self.increment_si();
            self.mod_config();
        }

    }

    pub fn column_scan(&mut self) {
        for i in 0..self.n {
            let x:Array1<f32> = self.data_config.as_ref().unwrap().column(i).to_owned().clone();
            let frc = default_FrqncCl(x.into_iter().collect(),self.r.clone());
            self.alert_on_random(frc.analysis,None,Some(i));
        }
    }

    /*
    increments slide index
    */
    pub fn increment_si(&mut self) -> bool {
        self.slide_index.next();
        if self.slide_index.is_finished() {
            return false;
        }
        true
    }

    /*
    */
    pub fn mod_config(&mut self) {
        let mut q: Array2<f32> = Array2::zeros((self.n,self.n));
        for i in 0..self.n {
            let mut r = self.data.row(i).to_owned().clone();
            let sls = self.slide_index.v[i] * self.slide_rate;
            r = matrixf::slide_arr1(r,sls);
            matrixf::replace_vec_in_arr2(&mut q,&mut r,i,true);
        }

        self.data_config = Some(q);
    }

    /*
    prints an alert if analysis f is in alert_range.
    If r is not None, value f is for the row r.
    If c is not None, value f is for the column c at config `slide_index`
    */
    pub fn alert_on_random(&mut self,f:f32,r:Option<usize>,c:Option<usize>) {
        assert!(r.is_none() || c.is_none());

        if f >= self.alert_range.0 && f <= self.alert_range.1 {
            if !r.is_none() {
                println!("initial,row {} measure: {}",r.unwrap(),f);
            } else {
                println!("col {} measure {}\n config {:?}",c.unwrap(),f,self.slide_index.v);
            }
        }
    }
}
