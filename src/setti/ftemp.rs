/*
function templates
*/

///////////// type: f32,pairwise

pub fn basic_add() -> fn(f32,f32) -> f32 {

    fn f(x:f32,x2:f32) -> f32 {
        x + x2
    }

    f
}

pub fn basic_mult() -> fn(f32,f32) -> f32 {

    fn f(x:f32,x2:f32) -> f32 {
        x * x2
    }

    f
}