use ndarray::{Array,Array1,Array2,arr1,arr2};
use std::collections::HashSet;
use std::hash::Hash;

/*
collects the subset in d with the most matches
typical values for arguments have Array1 corresponding to indices.
*/
pub fn fcollect_max_proper_hashfit_wrt_reference(mut d:Vec<Array1<usize>>,r:Array1<usize>,g:fn(Vec<Array1<usize>>,Array1<usize>) -> f32,verbose:bool) -> Vec<Array1<usize>> {

    // cache will be (element,score,start index of search for possible next)
    let mut q: Vec<(Vec<Array1<usize>>,f32,usize)> = Vec::new();
    let sol:Vec<Array1<usize>> = Vec::new();

    // populate with initial
    // iterate through cache until empty
    let mut result:Vec<Array1<usize>> = Vec::new();
    let mut s:f32 = f32::MIN;

    for (i,d_) in d.iter().enumerate() {
        let one = vec![((*d_).clone())];
        let sf = g(one.clone(),r.clone());
        q.push((one.clone(),sf,i));
        if sf > s {
            s = sf;
            result = one.clone();
        }

        if verbose {
            println!("hashfit is {:?}",one);
            println!("score is {}",sf);
            println!("----");
        }
    }

    let mut l = q.len();
    let d2: Vec<HashSet<String>> = d.clone().into_iter().map(|x| HashSet::<String>::from_iter((HashSet::<usize>::from_iter(x.into_iter())).into_iter().map(|y| y.to_string()))).collect();

    while l > 0 {
        // 2 hashes: usize -> string
        let mut f_: Vec<Array1<usize>> = q[0].0.clone();
        if verbose {
            println!("[0] hashfit is {:?}",f_);
        }

        let mut f: Vec<HashSet<String>> = f_.clone().into_iter().map(|x| HashSet::<String>::from_iter((HashSet::<usize>::from_iter(x.into_iter())).into_iter().map(|y| y.to_string()))).collect();

        let i:Option<usize> = next_possible_forward_string_hash_fit(d2.clone(),f.clone(),q[0].2);

        // update f_ and push it to cache
        let mut sf:f32 = 0.0;
        if !i.is_none() {
            f_.push(d[i.unwrap()].clone());
            sf = g(f_.clone(),r.clone());
            if verbose {
                println!("next possible {}",i.unwrap());
                println!("hashfit is {:?}",f_);
                println!("score is {}",sf);
                println!("----");
            }

            q.push((f_.clone(),sf,i.unwrap() + 1));
        } else {
            if verbose {
                println!("no more for hashfit {:?}",f_);
                println!("----");

            }
            sf = q[0].1.clone();
        }
        if sf > s {
            if verbose {
                println!("SF {} S {}",sf,s);
                println!("TRUE @ {:?}",f_);
            }

            s = sf;
            result = f_.clone();
        }

        q = q[1..].to_vec();
        l = q.len();

    }
    if verbose {
        println!("RESOLSHA {:?}",result);
    }

    result
}

/*
Calculates the fitness of d to reconstruct (fit) r.
(number of elements of r activated) - (number of unique elements of d not activated)

CAUTION: no arg. check
*/
pub fn hashfit_score1(d:Vec<Array1<usize>>,r:Array1<usize>) -> f32 {
    let mut hr:HashSet<usize> = HashSet::<usize>::from_iter(r.clone().into_iter());
    let mut hr2:HashSet<usize> = hr.clone();

    let mut s:i32 = 0;
    let mut s2:i32 = 0;

    // accounts for extra
    for d_ in d.into_iter() {
        for d2 in d_.into_iter() {
            if hr.contains(&(d2).clone()) {
                hr2.remove(&d2);
                s += 1;
            } else {
                s2 += 1;
            }
        }
    }

    (s - s2) as f32
}

/*
this is the actual functionale used de la fcollect_max_proper_hashfit_wrt_reference.
*/
pub fn hashfit_score2(d:Vec<Array1<usize>>,r:Array1<usize>) -> f32 {

    let mut hr:HashSet<usize> = HashSet::<usize>::from_iter(r.clone().into_iter());
    let mut hr2:HashSet<usize> = hr.clone();
    let mut s:f32 = 0.;

    for d_ in d.into_iter() {
        for d2 in d_.into_iter() {
            if hr.contains(&(d2).clone()) {
                hr2.remove(&d2);
                s += 1.0;
            }
        }
    }
    s

}

/*
f is the ordered (by d) sub-vector of d. Considers the subvector of d after the last
element of f

CAUTION: order-check for f not coded
*/
pub fn next_possible_forward_string_hash_fit(d:Vec<HashSet<String>>,f:Vec<HashSet<String>>,si:usize) -> Option<usize> {
    let l = d.len();
    let mut j:usize = 0;
    let mut s:usize = si; //
    for i in si..l {
        if d[i] == f[j] {
            j += 1;
            s = i;
        }

        if j >= f.len() {
            break;
        }
    }
    s += 1;

    let mut sol: Option<usize> = None;
    for i in s..l {
        let mut f2 = f.clone();
        f2.push(d[i].clone());
        if is_proper_hash_fit(f2) {
            sol = Some(i);
            break
        }
    }

    sol
}

/*
arr1 version of `is_proper_hash_fit`
*/
pub fn is_proper_hash_fit_arr1<T>(ahf:Vec<Array1<T>>) -> bool
where
T: Eq + Clone + Hash
{
    let ahf2: Vec<HashSet<T>> = ahf.into_iter().map(|x| HashSet::from_iter(x)).collect();
    is_proper_hash_fit(ahf2)
}

/*
determines if `vh` is a vector of hashsets that do not contain
intersecting elements.
*/
pub fn is_proper_hash_fit<T>(vh: Vec<HashSet<T>>) -> bool
where
T: Eq + Clone + Hash
{
    let mut checker: HashSet<T> = HashSet::new();

    for q in vh.iter() {
        let mut ws = checker.len() + (*q).len();
        for q_ in q.iter() {
            checker.insert((*q_).clone());
        }

        if checker.len() != ws {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hashfit_score1() {

        let da = arr1(&[2,3]);
        let db = arr1(&[5,6,7]);
        let dc = arr1(&[1,4]);

        let d1 = vec![da.clone(),db.clone(),dc.clone()];
        let d2 = vec![db.clone()];

        let f = arr1(&[1,2,3,4,5,6,7]);

        assert_eq!(7.0,hashfit_score1(d1.clone(),f.clone()));
        assert_eq!(3.0,hashfit_score1(d2.clone(),f.clone()));
    }

    #[test]
    fn test_is_proper_hash_fit() {

        let h1:HashSet<i32> = HashSet::from_iter([1,2,3]);
        let h2:HashSet<i32> = HashSet::from_iter([2,3,4]);
        let h3:HashSet<i32> = HashSet::from_iter([5,6]);
        let h4:HashSet<i32> = HashSet::from_iter([7,8]);

        let vh: Vec<HashSet<i32>> = vec![h1.clone(),h2,h3.clone()];
        let vh2: Vec<HashSet<i32>> = vec![h1.clone(),h3.clone(),h4];

        let b = is_proper_hash_fit(vh);
        assert!(!b);

        let b2 = is_proper_hash_fit(vh2);
        assert!(b2);
    }

    #[test]
    fn test_next_possible_forward_string_hash_fit() {
        // case: 1 and 2
        let hs1:HashSet<String> = HashSet::<String>::from_iter(["0".to_string(),"1".to_string(),"2".to_string()]);
        let hs2:HashSet<String> = HashSet::<String>::from_iter(["3".to_string(),"4".to_string()]);
        let hs3:HashSet<String> = HashSet::<String>::from_iter(["5".to_string()]);


        let d: Vec<HashSet<String>> = vec![hs1.clone(),hs2.clone(),hs3.clone()];
        let f1: Vec<HashSet<String>> = vec![hs1.clone()];
        let f2: Vec<HashSet<String>> = vec![hs1.clone(),hs2.clone()];

        let o:Option<usize> = next_possible_forward_string_hash_fit(d.clone(),f1,0);
        let o2:Option<usize> = next_possible_forward_string_hash_fit(d.clone(),f2,0);
        assert_eq!(o.unwrap(),1);
        assert_eq!(o2.unwrap(),2);

        // case: 3
        let hs1:HashSet<String> = HashSet::<String>::from_iter(["0".to_string(),"1".to_string(),"2".to_string()]);
        let hs2:HashSet<String> = HashSet::<String>::from_iter(["3".to_string(),"4".to_string()]);
        let hs3:HashSet<String> = HashSet::<String>::from_iter(["5".to_string()]);
        let hs4:HashSet<String> = HashSet::<String>::from_iter(["6".to_string(),"7".to_string()]);
        let hs5:HashSet<String> = HashSet::<String>::from_iter(["8".to_string(),"9".to_string(),"10".to_string()]);
        let hs6:HashSet<String> = HashSet::<String>::from_iter(["11".to_string(),"12".to_string()]);
        let d2: Vec<HashSet<String>> = vec![hs1.clone(),hs2.clone(),hs3.clone(),hs4.clone(),hs5.clone(),hs6.clone()];
        let f3: Vec<HashSet<String>> = vec![hs1.clone(),hs3.clone()];

        let o3:Option<usize> = next_possible_forward_string_hash_fit(d2.clone(),f3,0);
        assert_eq!(o3.unwrap(),3);
    }

    #[test]
    fn test_fcollect_max_proper_hashfit_wrt_reference() {

        let a1:Array1<usize> = arr1(&[0,3,4]);
        let a2:Array1<usize> = arr1(&[1,2]);
        let a3:Array1<usize> = arr1(&[5,6]);
        let hx1 :Vec<Array1<usize>> = vec![a1,a2,a3];
        let r:Array1<usize> = arr1(&[1,2]);

        let r2:Array1<usize> = arr1(&[0,1,2,3]);
        let r3:Array1<usize> = arr1(&[0,1,5,6]);
        let r4:Array1<usize> = arr1(&[1,6]);

        let vh4 = fcollect_max_proper_hashfit_wrt_reference(
                    hx1.clone(),r4.clone(),hashfit_score2,false);

        let vh4sol2 = vec![arr1(&[1,2]),arr1(&[5,6])];
        assert_eq!(vh4.clone(),vh4sol2);

        let vh42 = fcollect_max_proper_hashfit_wrt_reference(
                    hx1.clone(),r4.clone(),hashfit_score1,false);

        let vh4sol1 = vec![arr1(&[1,2])];
        assert_eq!(vh42.clone(),vh4sol1);


        let vh21 = fcollect_max_proper_hashfit_wrt_reference(
                hx1.clone(),r2.clone(),hashfit_score1,false);

        let vh22 = fcollect_max_proper_hashfit_wrt_reference(
                hx1.clone(),r2.clone(),hashfit_score2,false);

        let vh2sol = vec![arr1(&[0,3,4]), arr1(&[1,2])];
        assert_eq!(vh21.clone(),vh2sol);
        assert_eq!(vh21.clone(),vh22.clone());
    }
}
