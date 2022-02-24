// FUTURE: write comments

#[derive(Clone)]
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

    for i in 1..l {
        let g = data[i].clone();
        if g.0 >= g.1 {
            return false;
        }
        if g.0 < d.1 {
            return false;
        }
        d = g;
    }
    true
}

pub fn ranges_coincide(r1:(usize,usize),r2:(usize,usize)) -> bool {
    assert!(r1.0 < r1.1 && r2.0 < r2.1);
    if r1.1 >= r2.0 && r2.1 >= r1.0 {true} else {false}
}


pub fn build_vselect(v:Vec<(usize,usize)>) -> VSelect {
    assert!(check_valid_vselect_vec(v.clone()));
    VSelect{data:v}
}

impl VSelect {

    pub fn len(&mut self) -> usize {
        self.data.len()
    }

    pub fn size(&mut self) -> usize {
        self.data.iter().fold(0,|num,&val| num + (val.1 - val.0))
    }

    /*
    outputs the first index available during forward mode
    */
    pub fn available_forward(&mut self, n:usize) -> Option<usize> {
        let l = self.data.len();
        if l == 0 {
            return Some(0);
        }

        let m = self.data[l - 1].1.clone();
        if m < n {
            return Some(m);
        }
        None
    }

    pub fn available_binary(&mut self, n:usize) -> Vec<(usize,usize)> {
        Vec::new()
    }

    pub fn add_elemente(&mut self, n:usize, e:(usize,usize)) -> Option<usize> {
        let mut l:usize = self.data.len();//self.available_forward(n);
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

}

/*
Calculates the vector of range options for a ds-forward element of size `wanted_size` and distance
`distance`.
*/
pub fn options_for_dsf_element(n:usize,mut vs:VSelect, wanted_size:usize,distance:usize) -> Vec<(usize,usize)> {
    assert!(distance > 0);
    let l = vs.len();

    // case
    if wanted_size > vs.size() {
        return Vec::new();
    }

    // get the first available forward including the distance
    let x2 = vs.available_forward(n);
    if x2.is_none() {
        return Vec::new();
    }

    ////println!("available forward: {}",x2.unwrap());
    let mut x:usize = x2.unwrap();

    let mut c: Vec<(usize,usize)> = Vec::new();
    if x > 0 {
        x = x + distance;
    }

    // iterate from
    let mut sol: Vec<(usize,usize)> = Vec::new();

    while x < n - wanted_size {
        sol.push((x,x+wanted_size));
        x += 1;
    }
    sol
}

// each element is VSelect of size k
pub struct DSFGen {
    n: usize,
    k: usize,
    d: usize,
    s: usize,
    cache: Vec<VSelect>,
    results: Vec<VSelect>,
    stat: bool
}

pub fn build_DSFGen(n: usize,k: usize,d: usize,s: usize) -> DSFGen {
    // check arguments
    assert!(n > d);
    assert!(n >= k);

    // get initial cache
    let mut c: Vec<(usize,usize)> = Vec::new();
    let mut vs: VSelect = build_vselect(c);
    let mut cache: Vec<VSelect> = vec![vs];
    DSFGen{n:n,k: k,d:d,s: s,cache: cache,results: Vec::new(),stat: true}
}

// to get all possible options, iterate min:(1|2) through n - k (preselect size)

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
        let q = vs.available_forward(12);
        assert!(q.is_none());
        let q2 = vs.available_forward(13);
        assert_eq!(q2.unwrap(),12);
    }

    #[test]
    fn test_VSelect_add_elemente() {
        let mut vs = sample_VSelect_1();
        let sol1:Vec<(usize,usize)> = vec![(0, 3), (4, 5), (10, 12), (13, 17)];
        let sol2:Vec<(usize,usize)> = vec![(0, 3), (4, 5), (7, 9), (10, 12), (13, 17)];
        let sol3:Vec<(usize,usize)> = vec![(0, 3), (4, 5), (10, 12)];

        vs.add_elemente(20, (13,17));
        assert_eq!(vs.data,sol1);

        vs.add_elemente(20,(7,9));
        assert_eq!(vs.data,sol2);

        let data2 = sol1[1..3].to_vec();
        let mut vs2 = build_vselect(data2.clone());

        vs2.add_elemente(20,(0,3));
        assert_eq!(vs2.data,sol3);

        let x = vs2.add_elemente(20,(0,3));
        assert!(x.is_none());
    }

}
