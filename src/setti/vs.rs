/*
implementation of VSelect element
*/

use std::fmt;


/*
Each element in data is [start,end] index, start < end.
data is ordered.
*/
#[derive(Clone,Debug)]
pub struct VSelect {
    pub data: Vec<(usize,usize)>
}

pub fn check_valid_vselect_vec(data: Vec<(usize,usize)>) -> bool {

    let l = data.len();
    if l == 0 {
        return true;
    }

    let mut d = data[0].clone();
    if d.0 >= d.1 {
        return false;
    }
    let mut q = d.1;
    for i in 1..l {
        let g = data[i].clone();
        if g.0 <= q {
            return false;
        }

        if g.0 >= g.1 {
            return false;
        }

        q = g.1.clone();
    }
    true
}

pub fn ranges_coincide(r1:(usize,usize),r2:(usize,usize)) -> bool {
    assert!(r1.0 <= r1.1 && r2.0 <= r2.1);
    if r1.1 >= r2.0 && r2.1 >= r1.0 {true} else {false}
}


pub fn build_vselect(v:Vec<(usize,usize)>) -> VSelect {
    assert!(check_valid_vselect_vec(v.clone()));
    VSelect{data:v}
}

impl fmt::Display for VSelect {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.data)
        }
}

impl VSelect {

    pub fn len(&mut self) -> usize {
        self.data.len()
    }

    pub fn size(&mut self) -> usize {
        self.data.iter().fold(0,|num,&val| num + (val.1 - val.0) + 1)
    }

    /*
    outputs the first index available during forward mode
    */
    pub fn available_forward(&mut self, n:usize) -> Option<usize> {
        let l = self.data.len();
        if l == 0 {
            return Some(0);
        }

        let m = self.data[l - 1].1.clone() + 1;
        if m < n {
            return Some(m);
        }
        None
    }

    pub fn add_elemente(&mut self, e:(usize,usize)) -> Option<usize> {
        let mut l:usize = self.data.len();
        let l2 = self.data.len();
        for (i,x) in self.data.iter().enumerate() {

            if e.1 <= (*x).0 {

                if i > 0 {
                    let q = self.data[i - 1].clone();
                    if ranges_coincide(q,e.clone()) {
                        return None;
                    }
                }

                l = i;
                break
            }
        }

        let (mut x1,mut x2) = (self.data[0..l].to_vec(), self.data[l..l2].to_vec());
        x1.push(e.to_owned());
        x1.extend(x2);
        self.data = x1;
        Some(l)
    }

    /*
    calculates if valid for forward
    */
    pub fn is_valid_pre_vselect(&mut self,n:usize,k:usize,d:usize,s:usize) -> bool {
        if self.size() > k {
            return false;
        }
        if self.size() == k {
            return true;
        }

        if s > k - self.size() {
            return false;
        }

        let m = self.max();
        //let sz:usize = self.subvec_option_size(n,d,s,m);
        let sz:usize = self.max_possible_option_size(n,d,s,m);
        sz + self.size() >= k
    }

    pub fn max(&mut self) -> usize {
        let l = self.len();
        if self.len() == 0 {
            return 0;
        }
        self.data[l - 1].1 + 1
    }

    /*
    calculates the complement VSelect based on distance
    */
    pub fn complement(&mut self, n:usize, d: usize) -> VSelect {
        let mut d2: Vec<(usize,usize)> = Vec::new();
        let (mut i,mut s,mut e): (usize,usize,usize) = (0,0,0);
        let l = self.len();

        /*
        iterate through and collect complementary chunks
        */
        while i < l {
            if s < self.data[i].0 {

                if self.data[i].0 >= d {
                    e = self.data[i].0 - d;
                    if s < e {
                        d2.push((s,e));
                    }
                }

            }
            s = self.data[i].1 + d;
            i += 1;
        }

        // get the last
        if s < n - 1 {
            d2.push((s,n - 1));
        }

        build_vselect(d2)
    }


    /*
    assumes already in simplified form
    calculates the number of the subvectors >= minSize
    */
    pub fn subvec_option_size(&mut self,n:usize,d:usize,s:usize,i:usize) -> usize {

        let mut vs2 = self.complement(n,d);
        let x:usize = vs2.data.iter().fold(0,|num,&val| if val.1 - val.0 + 1 >= s && val.0 >= i {num + (val.1 - s + 1) - val.0 + 1} else {num});
        x
    }

    pub fn max_possible_option_size(&mut self,n:usize,d: usize,s:usize,i:usize) -> usize {
        let mut vs2 = self.complement(n,d);
        vs2.data.iter().fold(0,|num,&val| if val.1 - val.0 + 1 >= s && val.0 >= i {num + (val.1 - val.0 + 1)} else {num})
    }
}

pub fn sample_VSelect_1() -> VSelect {
    let data:Vec<(usize,usize)> = vec![(0,3),(4,5),(10,12)];
    build_vselect(data)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_VSelect_available_forward() {
        let mut vs = sample_VSelect_1();
        let q = vs.available_forward(13);
        assert!(q.is_none());
        let q2 = vs.available_forward(14);
        assert_eq!(q2.unwrap(),13);
    }

    #[test]
    fn test_VSelect_add_elemente() {
        let mut vs = sample_VSelect_1();
        let sol1:Vec<(usize,usize)> = vec![(0, 3), (4, 5), (10, 12), (13, 17)];
        let sol2:Vec<(usize,usize)> = vec![(0, 3), (4, 5), (7, 9), (10, 12), (13, 17)];
        let sol3:Vec<(usize,usize)> = vec![(0, 3), (4, 5), (10, 12)];

        vs.add_elemente((13,17));
        assert_eq!(vs.data,sol1);

        vs.add_elemente((7,9));
        assert_eq!(vs.data,sol2);

        let data2 = sol1[1..3].to_vec();
        let mut vs2 = build_vselect(data2.clone());

        vs2.add_elemente((0,3));
        assert_eq!(vs2.data,sol3);

        let x = vs2.add_elemente((0,3));
        assert!(x.is_none());
    }

    #[test]
    fn test_VSelect_complement() {
        let mut vs = build_vselect(Vec::new());
        let mut vs2 = vs.complement(20,2);
        assert_eq!(vs2.data,vec![(0, 19)]);

        let d:Vec<(usize,usize)> = vec![(0,2),(4,9)];
        vs = build_vselect(d);
        vs2 = vs.complement(20,2);
        assert_eq!(vs2.data,vec![(11, 19)]);

        let d2:Vec<(usize,usize)> = vec![(0,2),(4,9), (17,20),(21,24),(30,32)];
        vs = build_vselect(d2.clone());
        vs2 = vs.complement(50,3);
        assert_eq!(vs2.data,vec![(12, 14), (35, 49)]);
    }

    #[test]
    fn test_VSelect_subvec_option_size() {
        let d = vec![(13,19),(28,36),(44,49),(53,59),(75,90)];
        let mut uv: VSelect = build_vselect(d);
        let sos = uv.subvec_option_size(100,4,3,0);
        assert_eq!(sos,19);
    }

    #[test]
    fn test_VSelect_is_valid_pre_vselect() {

        let mut vs = build_vselect(vec![(0,3)]);
        assert!(vs.is_valid_pre_vselect(20,8,2,4));

        vs = build_vselect(vec![(0,4)]);
        assert!(!vs.is_valid_pre_vselect(20,8,2,4));
    }

}
