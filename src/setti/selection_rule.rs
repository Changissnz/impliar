/*
struct that is a restriction or requirement
that implements
*/
use crate::setti::matrixf;
use crate::setti::setf;
use crate::setti::strng_srt;
use ndarray::{Dim,Array,Array1,Array2,array,arr2,s};
use std::collections::HashSet;
use std::fmt;

/*
*/
pub fn default_rmatrix(n: usize,k: usize, idn: i32) -> Array2<i32> {
    Array2::ones((n, k)) * idn
}

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

////////////////////////////////////// methods for Restriction
#[derive(Clone)]
pub struct Restriction {
    // rows are reference elements, columns are indices
    pub data: ndarray::Array2<i32>
}

impl Restriction {

    pub fn restrict_row(&mut self, i: usize) {
        let k = self.data.raw_dim()[1];
        let mut res:Array1<i32> = Array1::ones(k);
        matrixf::replace_vec_in_arr2(&mut self.data,&mut res,i,true)
    }

    // TODO: not tested.
    pub fn restrict_subrow(&mut self, i:usize, start:usize,end:usize) {
        // copy
        let mut k:usize = self.data.raw_dim()[1];
        let mut x: Array1<i32> = Array1::zeros(k);

        for j in 0..k {
            if j >= start && j <= end {
                x[j] = 1;
            } else {
                x[j] = self.data[Dim((i,j))];
            }
        }

        let mut b = self.data.slice_mut(s![i, ..]);
        b.assign(&x);
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
    //pub all_req:bool,
}

pub fn build_requirement(rs:usize,required: Vec<(usize,Vec<usize>)>,k:usize) -> Requirement {
    let x = build_requirement_matrix(rs,required,k);
    Requirement{data:x}//,all_req:all_req}
}

pub fn build_requirement_matrix(rs:usize,required: Vec<(usize,Vec<usize>)>,k:usize) ->Array2<i32> {
    build_rmatrix(rs,-1,required,k)
}

impl Requirement {

    pub fn restrict_row(&mut self, i: usize) {
        let k = self.data.raw_dim()[1];
        let mut res:Array1<i32> = Array1::zeros(k);
        res = res;
        matrixf::replace_vec_in_arr2(&mut self.data,&mut res,i,true)
    }

    // TODO: not tested.
    pub fn restrict_subrow(&mut self, i:usize, start:usize,end:usize) {
        // copy
        let mut k:usize = self.data.raw_dim()[1];
        let mut x: Array1<i32> = Array1::zeros(k);

        for j in 0..k {
            if j >= start && j <= end {
                x[j] = 0;
                continue;
            } else {
                x[j] = self.data[Dim((i,j))];
            }
        }

        let mut b = self.data.slice_mut(s![i, ..]);
        b.assign(&x);
    }
}

////////////////////////////////////// methods for RuleCheck

#[derive(Clone)]
pub struct SelectionRule {
    pub res: Restriction,
    pub req: Requirement,
    pub choice: Vec<usize>
}

pub fn empty_selection_rule() {

}


/*
calculates the next choice given choice (of len k) by greedy forward selection (first available).

Forward selection selects the first available index i for the subvector "[i:i + distance]".

both `choice` and `output` are ordered vectors.
*/
pub fn next_available_forward(choice:Vec<usize>,n:usize,distance:usize) -> Option<Vec<usize>> {
    let mut l:usize = choice.len();
    let mut j:i32 = -1;
    let mut m_:usize = 0;
    for i in 1..l {
        let mut ix: usize = l - i;
        // subchoice
        let mut c2: Vec<usize> = choice[0..choice.len() - i].to_vec().clone();

        let mut w_:Vec<String> = c2.iter().map(|x| x.to_string()).collect();
        let mut s:String = strng_srt::stringized_srted_vec(&mut w_);

        // get first available index after
        let m:usize = choice[choice.len() - 1] + 1;
        if m + distance <= n {
            j = ix as i32;
            m_ = m;
            break
        }
    }

    if j == -1 {

        // get max of previous chunk (size is distance)
        // try iterating one forward from min
        let x:usize = choice[0];
        let x2:usize = x + l;
        if x2 < n {
            let mut c_:Vec<usize> = Vec::new();
            for i in 1..l+1 {
                c_.push(x + i);
            }
            return Some(c_);
        }

        return None;
    } else {
        // pre
        let mut c_:Vec<usize> = choice[0..choice.len() - distance].to_vec().clone();
        let mut cw_:Vec<String> = c_.iter().map(|x| x.to_string()).collect();

        // post
        let mut s:String = strng_srt::stringized_srted_vec(&mut cw_);

        for d_ in m_..m_ + distance {
            c_.push(d_);
        }

        return Some(c_);
    }
}

impl fmt::Display for SelectionRule {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut q = "* selection rule ".to_string();
        q.push_str(&format!("\n*\trestriction matrix\n"));
        q.push_str(&format!("{:?}\n",self.res.data));
        q.push_str(&format!("\n*\trequirement matrix\n"));
        q.push_str(&format!("{:?}",self.req.data));
        write!(f, "{}", q)
    }
}

/*
SelectionRule will be able to update for every "batch"
of equally-sized sequences.

Selection is a process that takes place along the y column,
and be one of elimination|non-elimination.
Elimination is when the existence of an element at index i in a vector
results in the element not able to be selected again at a later time
t >= i + 1.
*/
impl SelectionRule{

    pub fn clone(&mut self) -> SelectionRule {
        let rest = Restriction{data:self.res.data.clone()};
        let req = Requirement{data:self.req.data.clone()};
        SelectionRule{res:rest,req:req,choice:Vec::new()}
    }

    pub fn dimso(&mut self) -> (usize,usize) {
        (self.res.data.raw_dim()[0],self.res.data.raw_dim()[1])
    }

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

        // case: required elements at index
        if vq.len() > 0 {
            // minus restricted from required
            let vqi_: HashSet<usize> = vq.into_iter().collect();
            let sol1: HashSet<usize> = vqi_.difference(&vs).map(|x| *x).collect();
            return sol1;
        }

        let vq0: HashSet<usize> = self.vec_at_col_index(i,false,false).into_iter().collect();
        let sol2: HashSet<usize> = vq0.difference(&vs).map(|x| *x).collect();
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
    let req = build_requirement(rs,requirement,k);
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
        let rq = build_requirement(rs,req,k);
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
        let mut sol :Vec<String> = vec!["0_5_7".to_string(),"0_1_2_3_4_5_6_7_8_9".to_string(),
                                "2_5_7_9".to_string(), "5_7".to_string(),
                                "0_1_2_3_4_5_6_7_8_9".to_string(), "0_1_2_3_4_5_6_7_8_9".to_string()];

        for i in 0..6 {
            let mut c = sr.choices_at_col_index(i);
            let mut ch: HashSet<String>  = c.iter().map(|x| (*x).to_string()).collect();
            let csh = strng_srt::stringized_srted_hash(ch);
            assert_eq!(csh,sol[i]);
        }
    }

    #[test]
    fn test_RequirementRestriction_restrict_subrow() {
        let (mut s,mut q) = test_rule_contents_2();
        q.restrict_subrow(7,2,5);
        let mut tt = arr2(&[[-1, 0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0, 0],
         [0, 0, -1, -1, 0, 0],
         [0, 0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0, 0],
         [-1, 0, -1, -1, 0, 0],
         [0, 0, 0, 0, 0, 0],
         [-1, 0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0, 0],
         [0, 0, -1, 0, 0, 0]]);
         assert_eq!(tt,q.data);

        let mut tt2 = arr2(&[[0, 0, 1, 1, 0, 0],
                         [0, 0, 0, 0, 0, 0],
                         [1, 0, 0, 1, 0, 0],
                         [1, 0, 1, 0, 0, 0],
                         [0, 0, 0, 0, 0, 0],
                         [0, 0, 0, 0, 0, 0],
                         [0, 0, 0, 0, 0, 0],
                         [0, 0, 1, 1, 1, 1],
                         [0, 0, 0, 0, 0, 0],
                         [0, 0, 0, 0, 0, 0]]);

        s.restrict_subrow(7,2,5);
        assert_eq!(tt2,s.data);
    }

    #[test]
    fn test_next_available_forward() {
        let c1:Vec<usize> = vec![0,1,2];
        let c2:Vec<usize> = vec![0,1,3];
        let c3:Vec<usize> = vec![5,6,7];
        let c4:Vec<usize> = vec![0,1,7];
        let c5:Vec<usize> = vec![2,5,6];
        let c6:Vec<usize> = vec![2,5,7];

        let mut w: Option<Vec<usize>> = next_available_forward(c1,8,1);
        let mut w_: Vec<usize> = w.unwrap();
        let mut w__:Vec<String> = w_.iter().map(|x| x.to_string()).collect();
        let mut s:String = strng_srt::stringized_srted_vec(&mut w__);
        assert_eq!(s,"0_1_3".to_string());

        let mut w2: Option<Vec<usize>> = next_available_forward(c2,8,1);
        let mut w2_: Vec<usize> = w2.unwrap();
        let mut w2__:Vec<String> = w2_.iter().map(|x| x.to_string()).collect();
        let mut s:String = strng_srt::stringized_srted_vec(&mut w2__);
        assert_eq!(s,"0_1_4".to_string());

        let mut w3: Option<Vec<usize>> = next_available_forward(c3,8,1);
        assert!(w3.is_none());

        let mut w4: Option<Vec<usize>> = next_available_forward(c4,8,1);
        let mut w4_: Vec<usize> = w4.unwrap();
        let mut w4__:Vec<String> = w4_.iter().map(|x| x.to_string()).collect();
        let mut s:String = strng_srt::stringized_srted_vec(&mut w4__);
        assert_eq!(s,"1_2_3".to_string());

        let mut w5: Option<Vec<usize>> = next_available_forward(c5,8,1);
        let mut w5_: Vec<usize> = w5.unwrap();
        let mut w5__:Vec<String> = w5_.iter().map(|x| x.to_string()).collect();
        let mut s:String = strng_srt::stringized_srted_vec(&mut w5__);
        assert_eq!(s,"2_5_7".to_string());

        let mut w6: Option<Vec<usize>> = next_available_forward(c6,8,1);
        let mut w6_: Vec<usize> = w6.unwrap();
        let mut w6__:Vec<String> = w6_.iter().map(|x| x.to_string()).collect();
        let mut s:String = strng_srt::stringized_srted_vec(&mut w6__);
        assert_eq!(s,"3_4_5".to_string());

    }
}
