/*
this is a struct used to increment elements
*/
use asciis::asc::Asciis;
use substring::Substring;
use std::str::FromStr;


pub struct Incr<T> {
    pub x: T,
}

impl<T> Incr<T>
where
    T:Inc
{

    pub fn increment(&mut self) {
        self.x.inc();
    }
}

pub trait Inc {
    fn inc(&mut self) {
    }
}

pub struct Inc1 {
    pub value: i32
}

/*
i32 incrementor
*/
impl Inc for Inc1 {

    fn inc(&mut self){
        self.value += 1;
    }
}

impl Inc1 {

    fn inc_(&mut self) -> i32 {
        let v = self.value.clone();
        self.inc();
        v
    }

}

pub struct Inc1String {
    pub value: String,
}

impl Inc for Inc1String {


    fn inc(&mut self) {
        let asc = Asciis{};

        // case: empty
        if self.value.len() == 0 {
            self.value = "a".to_string();
        }

        let mut x = self.value.clone();

        // get the last
        let si = self.value.len() -1;
        let ei = self.value.len();
        let mut t = (self.value.substring(self.value.len() -1,self.value.len())).to_owned();

        let mut r:i32 = asc.ord(t.as_str()).unwrap();

                // a-z: 97|122
                // A-Z: 65|90
        if r + 1 > 122 {
            let mut x = asc.chr(65).unwrap();
            self.value.push(char::from_str(x.as_str()).unwrap());
        } else if r + 1 > 90 && r >= 65 && r <=90{
            // move to range 97,122
            let mut x = asc.chr(97).unwrap();
            self.value = (self.value.substring(0,self.value.len() -1)).to_owned();
            self.value.push(char::from_str(x.as_str()).unwrap());
        } else {
            let mut x = asc.chr(r + 1).unwrap();
            self.value = (self.value.substring(0,self.value.len() -1)).to_owned();
            self.value.push(char::from_str(x.as_str()).unwrap());
        }
    }
}

/*
alphabetical incrementor
*/
impl Inc1String {

    pub fn inc_(&mut self) -> String {
        let v = self.value.clone();
        self.inc();
        v
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_Incr() {
        // work on incrementors
        let mut q = Inc1{value:3};
        let mut e = Incr{x:q};

        let mut q = 3;
        for i in 0..5 {
            assert_eq!(e.x.value,q);
            e.increment();
            q += 1;
        }

        let mut q = Inc1String{value:"a".to_string()};
        let mut e = Incr{x:q};
        let sol = vec!["a","b","c","d","e"];
        for i in 0..5 {
            println!("X: {}",e.x.value);
            assert_eq!(e.x.value.as_str(),sol[i]);
            e.increment();
        }
    }

    #[test]
    fn test_Inc1String() {
        let mut x = Inc1String{value:"volfina".to_string()};
        assert_eq!(x.inc_(), "volfina".to_string());
        assert_eq!(x.inc_(), "volfinb".to_string());
        assert_eq!(x.inc_(), "volfinc".to_string());
    }

    #[test]
    fn test_Inc1() {
        let mut x = Inc1{value:10};
        assert_eq!(x.inc_(), 10);
        assert_eq!(x.inc_(), 11);
        assert_eq!(x.inc_(), 12);
        assert_eq!(x.inc_(), 13);
    }

}
