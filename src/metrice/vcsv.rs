/*
csv functions for reading/write vec<arr1>
*/
use ndarray::{Array1,arr1,Array2,arr2};
use std::any::type_name;
use std::fs::{File,OpenOptions};
use std::io::{Write, BufReader, BufRead, Error};

use crate::setti::setf;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn arr1_seq_to_csv(a:Vec<Array1<f32>>,filename: &str,file_mode:&str) {
    // open file based on write|append mode
    let mut fileRef = if file_mode == "a" {OpenOptions::new().append(true).open(filename).expect("unable to open file")}
                    else {OpenOptions::new().read(true).write(true).create(true).truncate(true).open(filename).expect("Unable to open file")};

    let r = a.len();
    for i in 0..r {
        let mut s = setf::vec_to_str(a[i].clone().into_iter().collect());
        s += "\n";
        fileRef.write_all(s.as_bytes()).expect("write failed");
    }
}

pub fn csv_to_arr1_seq(filepath: &str) -> Result<Vec<Array1<f32>>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut sol: Vec<Array1<f32>> = Vec::new();

    for line in reader.lines() {
        let s: Vec<String> = setf::str_to_vec(line?);
        let s2:Array1<f32> = s.into_iter().map(|x| x.parse::<f32>().unwrap()).collect();
        sol.push(s2);
    }

    Ok(sol)
}


pub fn arr1_to_csv(a:Array1<f32>,filename: &str,file_mode:&str) {
    let mut fileRef = if file_mode == "a" {OpenOptions::new().append(true).open(filename).expect("unable to open file")}
                    else {OpenOptions::new().read(true).write(true).create(true).truncate(true).open(filename).expect("Unable to open file")};

    let r = a.len();
    for i in 0..r {
        let mut s = setf::vec_to_str(vec![a[i]]);
        s += "\n";
        fileRef.write_all(s.as_bytes()).expect("write failed");
    }
}

pub fn csv_to_arr1(filepath: &str) -> Result<Array1<f32>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut sol: Vec<f32> = Vec::new();

    for line in reader.lines() {
        let s: Vec<String> = setf::str_to_vec(line?);
        sol.push(s[0].parse::<f32>().unwrap());
    }

    Ok(sol.into_iter().collect())
}
