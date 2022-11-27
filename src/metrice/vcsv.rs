/*
csv functions for reading/write vec<arr1>
*/
use ndarray::{Array1,arr1,Array2,arr2};
use std::any::type_name;
use std::fs::{File,OpenOptions};
use std::io::{Read,Write, BufReader, BufRead,Error};
use crate::setti::setf;

/// # description
/// displays type of `T`
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/// # description
/// writes sequence of <arr1\<f32\>> into `filename` based on `file_mode` append (a) or write (w). 
pub fn arr1_seq_to_csv(a:Vec<Array1<f32>>,filename: &str,file_mode:&str) {
    // open file based on write|append mode
    let mut fileRef = if file_mode == "a" {OpenOptions::new().append(true).open(filename).expect("unable to open file")}
                    else {OpenOptions::new().read(true).write(true).create(true).truncate(true).open(filename).expect("Unable to open file")};

    let r = a.len();
    for i in 0..r {
        let mut s = setf::vec_to_str(a[i].clone().into_iter().collect(),'_');
        s += "\n";
        fileRef.write_all(s.as_bytes()).expect("write failed");
    }
}

/// # description
/// loads `filepath` contents into a sequence of <arr1\<f32\>>
pub fn csv_to_arr1_seq(filepath: &str) -> Result<Vec<Array1<f32>>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut sol: Vec<Array1<f32>> = Vec::new();

    for line in reader.lines() {
        let s: Vec<String> = setf::str_to_vec(line?,'_');
        let s2:Array1<f32> = s.into_iter().map(|x| x.parse::<f32>().unwrap()).collect();
        sol.push(s2);
    }

    Ok(sol)
}

/// # description
/// writes <arr1\<f32\>> into `filename` based on `file_mode` append (a) or write (w). 
pub fn arr1_to_csv(a:Array1<f32>,filename: &str,file_mode:&str) {
    let mut fileRef = if file_mode == "a" {OpenOptions::new().append(true).open(filename).expect("unable to open file")}
                    else {OpenOptions::new().read(true).write(true).create(true).truncate(true).open(filename).expect("Unable to open file")};

    let r = a.len();
    for i in 0..r {
        let mut s = setf::vec_to_str(vec![a[i]],'_');
        s += "\n";
        fileRef.write_all(s.as_bytes()).expect("write failed");
    }
}

/// # description
/// loads `filepath` contents into <arr1\<f32\>>
pub fn csv_to_arr1(filepath: &str) -> Result<Array1<f32>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut sol: Vec<f32> = Vec::new();

    for line in reader.lines() {
        let s: Vec<String> = setf::str_to_vec(line?,'_');
        assert!(s.len() == 1);
        sol.push(s[0].parse::<f32>().unwrap());
    }

    Ok(sol.into_iter().collect())
}

/// # description
/// loads `filepath` into <fs::File> object
pub fn file_read_obj(filepath: &str) -> Result<BufReader<File>,Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    Ok(BufReader::new(file))
}

/// batch reader used to read the contents of a csv-file by
/// batches (each of line size `batch_size`)
pub struct BatchReader {
    /// string path of csv file
    filepath: String,
    br: BufReader<File>,
    /// maximum size of each batch
    batch_size: usize,
    /// used for numerical reads; data is singleton or multi
    singleton: bool,
    /// separator
    sep: char,
    /// status of file-read
    pub done: bool
}

pub fn build_BatchReader(fp: String, bs: usize,singleton: bool,s: char) -> BatchReader {
    let mut x = file_read_obj(fp.as_str());
    assert_eq!(true,x.is_ok());
    BatchReader{filepath:fp,br:x.unwrap(),batch_size:bs,singleton:singleton,sep:s,done:false}
}

impl BatchReader {

    pub fn read_batch(&mut self) -> Vec<String> {
        let mut c = 0;
        let mut vs: Vec<String> = Vec::new();
        // case: status is done
        if self.done {
            return vs;
        }

        for line in self.br.by_ref().lines() {

            // case: file read error
            if !line.is_ok() {
                self.done = true;
                break;
            }

            vs.push(line.unwrap());

            c += 1;
            if c == self.batch_size {
                break;
            }
        }

        // case: empty batch, done
        if c == 0 {
            self.done = true;
        }

        // case: batch not at max size, done
        if c < self.batch_size {
            self.done = true;
        }

        vs
    }

    pub fn read_numerical_sample_multi(&mut self,t:String) -> Array1<f32> {
        let s: Vec<String> = setf::str_to_vec(t,self.sep);
        s.into_iter().map(|x| x.parse::<f32>().unwrap()).collect()
    }

    pub fn read_numerical_sample_singleton(&mut self, t:String) -> f32 {
        let s: Vec<String> = setf::str_to_vec(t,self.sep);
        assert!(s.len() == 1);
        s[0].parse::<f32>().unwrap()
    }

    pub fn read_batch_numerical(&mut self) -> (Option<Vec<f32>>,Option<Vec<Array1<f32>>>) {
        let vs = self.read_batch();
        if self.singleton {
            return (Some(vs.into_iter().map(|x| self.read_numerical_sample_singleton(x)).collect()),None);
        }

        (None,Some(vs.into_iter().map(|x| self.read_numerical_sample_multi(x)).collect()))
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test__BatchReader__read_batch_numerical() {
        // case 1: 
        let mut br = build_BatchReader("src/data/f2_x.csv".to_string(),2,false,'_');
        let mut c = 0;
        while !br.done {
            br.read_batch_numerical(); 
            c += 1;
        }

        assert_eq!(c,3);

        // case 2: 
        br = build_BatchReader("src/data/f3_x.csv".to_string(),2,false,'_');
        c = 0;
        while !br.done {
            br.read_batch_numerical(); 
            c += 1;
        }

        assert_eq!(c,6);
    }

}