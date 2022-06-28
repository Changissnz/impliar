/*
code for guessing soln
*/
use std::sync::{Arc, Mutex, Once};

pub static START: Once = Once::new();

pub static mut ARCMUT: Vec<Arc<Mutex<i32>>> = Vec::new();

pub fn addx() -> i32 {
    let mut arc_clone = unsafe { ARCMUT[0].clone() };
    let mut unlcok = arc_clone.lock().unwrap();
    *unlcok += 100;
    *unlcok
}

//////////////////////////////
pub static mut ARCMUT_F32: Vec<Arc<Mutex<f32>>> = Vec::new();

pub fn initialize_storage_empty(sz:usize) {

    let mut store: Vec<Arc<Mutex<f32>>> = Vec::new();
    for i in 0..sz {
        let a = Arc::new(Mutex::new(0.));
        store.push(a);
    }

    let so: Once = Once::new();
    so.call_once(|| unsafe {
        ARCMUT_F32 = store;
    });
}

pub fn mod_st_at(f:f32,i:usize) {
    let mut arc_clone = unsafe { ARCMUT_F32[i].clone() };
    let mut unlcok = arc_clone.lock().unwrap();
    *unlcok = f;
    //*unlcok
}

pub fn st_at(i:usize) -> f32 {
    let mut arc_clone = unsafe { ARCMUT_F32[i].clone() };
    let x = arc_clone.lock().unwrap().clone();
    x
}

pub fn op_st_at(f:fn(f32,f32) -> f32,f2:f32,i:usize) -> f32 {
    let f2_ = st_at(i);
    f(f2,f2_)
}

/*
pub fn gessir_add_plug_f32(f:f32,i:usize) {

}

pub fn gessir_add_f32() {

}
*/
