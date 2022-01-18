/*
struct that is a restriction or requirement
that implements
*/
use crate::setti::matrixf;
use crate::setti::setf;
use crate::setti::strng_srt;
use ndarray::{Dim,Array,Array1,Array2,array,s};
use std::collections::HashSet;

pub fn build_rmatrix(rs:usize,idn:i32,res_req: Vec<(usize,Vec<usize>)>,k:usize) ->Array2<i32> {

    assert!(rs > 0 && k > 0);
    assert!(rs >= k);

    // empty array
    let mut sol:Array2<i32>  = Array2::zeros((rs, k));

    // case: empty rule
    if res_req.len() == 0 || rs == 0 {
        return sol;
    }

    for r in res_req.iter() {
        // get index of restricted in reference
        let s = (*r).0.clone();
        let mut nu = Array1::zeros(rs);
        for rx in (*r).1.iter() {
            nu[Dim([*rx])] = idn;
        }
        matrixf::replace_vec_in_arr2(&mut sol,&mut nu,s,false);
    }
    sol
}
/*
impl<T> fmt::Display for SetImp<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let s2 = setf::vec_to_str(self.operating_start.clone());
        s.push_str(s2.as_str());
        write!(f, "({})", s)
    }
}
*/
////////////////////////////////////// methods for Restriction
#[derive(Clone)]
pub struct Restriction {
    // rows are reference elements, columns are indices
    pub data: ndarray::Array2<i32>
}

impl Restriction {

    pub fn restrict_row(&mut self, i: usize) {
        println!("restricting {}",i);
        let k = self.data.raw_dim()[1];
        let mut res:Array1<i32> = Array1::ones(k);
        //res = res;// * -1;
        //let mut b = self.slice_mut(s![i, ..]);
        matrixf::replace_vec_in_arr2(&mut self.data,&mut res,i,true)
    }
}

/*
restricted := Vec<index in 0..k,indices not allowed>
*/
pub fn build_restriction(rs:usize,restricted: Vec<(usize,Vec<usize>)>,k:usize) -> Restriction {
    let rm = build_restriction_matrix(rs,restricted,k);
    Restriction{data:rm}
}

pub fn build_restriction_matrix(rs:usize,restricted: Vec<(usize,Vec<usize>)>,k:usize) ->Array2<i32> {
    build_rmatrix(rs,1,restricted,k)
}

////////////////////////////////////// methods for Requirement
#[derive(Clone)]
pub struct Requirement {
    pub data: ndarray::Array2<i32>,
    pub all_req:bool,
}

pub fn build_requirement(rs:usize,required: Vec<(usize,Vec<usize>)>,k:usize,all_req:bool) -> Requirement {
    let x = build_requirement_matrix(rs,required,k);
    Requirement{data:x,all_req:all_req}
}

pub fn build_requirement_matrix(rs:usize,required: Vec<(usize,Vec<usize>)>,k:usize) ->Array2<i32> {
    build_rmatrix(rs,-1,required,k)
}

impl Requirement {

    pub fn restrict_row(&mut self, i: usize) {
        println!("require {}",i);
        let k = self.data.raw_dim()[1];
        let mut res:Array1<i32> = Array1::zeros(k);
        res = res;// * -1;
        //let mut b = self.slice_mut(s![i, ..]);
        matrixf::replace_vec_in_arr2(&mut self.data,&mut res,i,true)
    }
}

////////////////////////////////////// methods for RuleCheck

#[derive(Clone)]
pub struct SelectionRule {
    pub res: Restriction,
    pub req: Requirement,
    pub choice: Vec<usize>
}



/*
SelectionRule will be able to update for every "batch"
of equally-sized sequences

Selection is a process that takes place along the y column,
and be one of elimination|non-elimination.
Elimination is when the existence of an element at index i in a vector
results in the element not able to be selected again at a later time
t >= i + 1.
*/
impl SelectionRule{

    pub fn content_check(&mut self) ->bool {
        check_rule_contents(&mut self.res,&mut self.req)
    }

    pub fn vec_at_col_index(&mut self,i:usize,is_res:bool,is_pos:bool) -> Array1<usize> {
        let c = if is_res {self.res.data.slice_mut(s![..,i])} else
                {self.req.data.slice_mut(s![..,i])};

        let mut y:i32 = -1;
        if is_pos {
            y = if is_res {1} else {-1};
        } else {
            y = 0;
        }

        let fx  = |x:&(usize,&i32)| *((*x).1) == y;

        let rq:Array1<_> = c.iter().enumerate().filter(fx).collect();
        let rqi:Array1<usize> = rq.iter().map(|j:&(usize,&i32)| (*j).0).collect();
        rqi
    }

    /*
    Selects the choice `ch` and marks it off the restricted matrix if `eliminate` is
    set to true.

    WARNING: does not check if choice valid.
    */
    pub fn select_choice_at_col_index(&mut self, ch: usize, ci: usize, eliminate:bool) -> usize {
        self.res.data[Dim([ch,ci])] = 1;
        if !eliminate {
            return ch;
        }
        self.res.restrict_row(ch);
        self.req.restrict_row(ch);
        ch
    }

    /*
    Calculates available choice at column.
    */
    pub fn choices_at_col_index(&mut self,i:usize) -> HashSet<usize> {
        if i >= self.res.data.raw_dim()[1] {
            return HashSet::new();
        }

        let vq = self.vec_at_col_index(i,false,true);
        let vs:HashSet<usize> = self.vec_at_col_index(i,true,true).into_iter().collect();

        let mut vqx1:Vec<usize> = vs.iter().map(|x| *x).collect();

        // case: required elements at index
        if vq.len() > 0 {
            // minus restricted from required
            let mut vqi_: HashSet<usize> = vq.into_iter().collect();
            let sol1: HashSet<usize> = vqi_.difference(&vs).map(|x| *x).collect();
            return sol1;
        }

        let mut vq0: HashSet<usize> = self.vec_at_col_index(i,false,false).into_iter().collect();
        let sol2: HashSet<usize> = vq0.difference(&vs).map(|x| *x).collect();
        let mut vqx3:Vec<usize> = sol2.iter().map(|x| *x).collect();
        sol2
    }
}

pub fn check_rule_contents(restricted: &Restriction,
        required: &Requirement) -> bool {
    let mut q = (*restricted).data.clone() * (*required).data.clone();
    let mut h:HashSet<i32> = HashSet::new();
    h.insert(-1);
    let l = (*restricted).data.raw_dim()[0];
    for i in 0..l {
        if matrixf::exist_any_in_vec_of_arr2(&mut q,h.clone(),i,true) {
            return false;
        }
    }
    true
}

pub fn std_collision_score(y1:&(usize,&i32)) -> bool {
    *((*y1).1) == -1
}

pub fn collision_score(res_req: Array2<i32>,f: fn(&(usize,&i32))->bool) ->i32 {
    let x:Array1<_> = res_req.iter().enumerate().filter(f).collect();
    let x2:Array1<i32> = x.iter().map(|y| *(*y).1).collect();
    x2.len() as i32
}


pub fn one_index_to_two_index(i:usize,x:usize,y:usize) -> (usize,usize) {
    assert!(i < x * y);

    let oi = i / y;
    let oi2 = i % y;
    (oi,oi2)
}

/*
fixes collisions between restricted and required by
flipping a colliding element from either `restricted`
or `required` based on `preference`
*/
pub fn fix_rule_contents_1(restricted: &mut Restriction,
        required: &mut Requirement,preference:Array1<i32>) -> bool {
    let res_req = (*restricted).data.clone() * (*required).data.clone();
    let x:Array1<_> = res_req.iter().enumerate().filter(std_collision_score).collect();
    let x2:Array1<usize> = x.iter().map(|y| (*y).0).collect();

    if x2.len() == 0 {
        return false;
    }

    let (rs,cs) = (restricted.data.raw_dim()[0],restricted.data.raw_dim()[1]);
    for x2_ in x2.iter() {
        let g = one_index_to_two_index(*x2_,rs,cs);
        // flip restricted
        if preference[*x2_] == 1 {
            (*restricted).data[Dim([g.0,g.1])] = 0;
        } else { // flip required
            (*required).data[Dim([g.0,g.1])] = 0;
        }
    }

    true
}


////////////////////////////////////////////

pub fn test_rule_contents() -> (Restriction,Requirement){

    let rs = 9;
    let k = 4;
    let restriction:Vec<(usize,Vec<usize>)> = vec![
                            (0,vec![0,1,3,4]),(2,vec![0,1,2,5,7])];

    let requirement:Vec<(usize,Vec<usize>)> = vec![
                            (0,vec![0,4,5,6]),(2,vec![5,8])];

    let res = build_restriction(rs,restriction,k);
    let req = build_requirement(rs,requirement,k,true);
    (res,req)
}

pub fn test_rule_contents_2() -> (Restriction,Requirement){

        let rs:usize = 10;
        let k:usize = 6;
        let mut rest: Vec<(usize,Vec<usize>)> = Vec::new();
        rest.push((0,vec![2,3]));
        rest.push((2,vec![0,3]));
        rest.push((3,vec![0,2]));
        let rst = build_restriction(rs,rest,k);

        let mut req: Vec<(usize,Vec<usize>)> = Vec::new();
        req.push((0,vec![0,5,7]));
        req.push((2,vec![5,7,9,2]));
        req.push((3,vec![2,5,7]));
        let rq = build_requirement(rs,req,k,true);
        (rst,rq)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_restriction_matrix() {

        let rs:usize = 10;
        let k:usize = 6;
        let mut rest: Vec<(usize,Vec<usize>)> = Vec::new();
        rest.push((0,vec![0,4,6]));
        rest.push((2,vec![1,3,4]));
        rest.push((3,vec![5,6,7,8]));

        let mut sol = build_restriction_matrix(rs,rest,k);

        let mut a2 = array![[1, 0, 0, 0, 0, 0],
                     [0, 0, 1, 0, 0, 0],
                     [0, 0, 0, 0, 0, 0],
                     [0, 0, 1, 0, 0, 0],
                     [1, 0, 1, 0, 0, 0],
                     [0, 0, 0, 1, 0, 0],
                     [1, 0, 0, 1, 0, 0],
                     [0, 0, 0, 1, 0, 0],
                     [0, 0, 0, 1, 0, 0],
                     [0, 0, 0, 0, 0, 0]];

        assert_eq!(sol,a2);
    }

    #[test]
    fn test_initialize_restriction() {

        let rs:usize = 10;
        let k:usize = 6;
        let mut rest: Vec<(usize,Vec<usize>)> = Vec::new();
        rest.push((0,vec![0,4,6]));
        rest.push((2,vec![1,3,4]));
        rest.push((3,vec![5,6,7,8]));

        let mut xx = build_rmatrix(rs,-1,rest.clone(),k);
        let mut xy = build_restriction(rs,rest,k);
    }

    #[test]
    fn test_check_rule_contents() {
        let mut x = test_rule_contents();
        let c = check_rule_contents(&x.0,&x.1);
        assert!(!c);
    }

    #[test]
    fn test_collision_score() {
        let mut x = test_rule_contents();
        let mut x3 = x.0.data * x.1.data;
        let mut nx = collision_score(x3.clone(),std_collision_score);
        assert_eq!(nx,3);
    }

    #[test]
    fn test_fix_rule_contents_1() {
        let (mut res, mut req): (Restriction,Requirement) = test_rule_contents();

        let mut res_req = res.data.clone() * req.data.clone();
        let mut preference:Array1<i32> = array![0,0,0,0,
                                            0,0,0,0,
                                            0,0,0,0,
                                            0,0,0,0,
                                            1,0,0,0,
                                            0,0,1,0,
                                            0,0,0,0,
                                            0,0,0,0,
                                            0,0,0,0];

        let b = fix_rule_contents_1(&mut res, &mut req, preference);

        let resSol = array![[1, 0, 1, 0],
         [1, 0, 1, 0],
         [0, 0, 1, 0],
         [1, 0, 0, 0],
         [0, 0, 0, 0],
         [0, 0, 0, 0],
         [0, 0, 0, 0],
         [0, 0, 1, 0],
         [0, 0, 0, 0]];

         let reqSol = array![[0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [-1, 0, 0, 0],
            [-1, 0, -1, 0],
            [-1, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, -1, 0]];

         assert_eq!(resSol,res.data);
         assert_eq!(reqSol,req.data);

        res_req = res.data.clone() * req.data.clone();
        let score = collision_score(res_req,std_collision_score);
        assert_eq!(score,0);
        let b2 = check_rule_contents(&mut res,&mut req);
        assert!(b2);
    }

    #[test]
    fn test_initialize_SelectionRule() {
        let (mut res,mut req) = test_rule_contents_2();
        // calculate number of possibilities
        let mut sr = SelectionRule{res:res,req:req,choice:Vec::new()};
    }

    #[test]
    fn test_SelectionRule_vec_at_col_index() {
        let (mut res,mut req) = test_rule_contents_2();
        let mut sr = SelectionRule{res:res,req:req,
            choice:Vec::new()};

        let r3 = sr.vec_at_col_index(3,true,true);
        let r4 = sr.vec_at_col_index(3,false,true);
        assert_eq!(r3,array![0,2]);
        assert_eq!(r4,array![2,5,7]);

        let r5 = sr.vec_at_col_index(0,true,true);
        let r6 = sr.vec_at_col_index(0,false,true);
        assert_eq!(r5,array![2,3]);
        assert_eq!(r6,array![0,5,7]);
    }


    #[test]
    fn test_SelectionRule_choices_at_col_index() {
        let (mut res,mut req) = test_rule_contents_2();
        let mut sr = SelectionRule{res:res,req:req,
            choice:Vec::new()};
        let mut sol :Vec<String> = vec!["0-5-7".to_string(),"0-1-2-3-4-5-6-7-8-9".to_string(),
                                "2-5-7-9".to_string(), "5-7".to_string(),
                                "0-1-2-3-4-5-6-7-8-9".to_string(), "0-1-2-3-4-5-6-7-8-9".to_string()];

        for i in 0..6 {
            let mut c = sr.choices_at_col_index(i);
            let mut ch: HashSet<String>  = c.iter().map(|x| (*x).to_string()).collect();
            let csh = strng_srt::stringized_srted_hash(ch);
            assert_eq!(csh,sol[i]);
        }
    }



}
