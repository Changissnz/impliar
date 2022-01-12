/*
this is a struct used to increment elements
*/
use asciis::asc::Asciis;
use substring::Substring;
use std::str::FromStr;

pub struct Inc1 {
    pub value: i32
}

/*
i32 incrementor
*/
impl Inc1 {

    pub fn next(&mut self) -> i32 {
        let v = self.value;
        self.value += 1;
        v
    }
}

pub struct Inc1String {
    pub value: String,

}

/*
alphabetical incrementor
*/
impl Inc1String {

    pub fn next(&mut self) -> String {
        let asc = Asciis{};

        // case: empty
        if self.value.len() == 0 {
            self.value = "a".to_string();
            return self.value.clone();
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

        x
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_Inc1String() {
        let mut x = Inc1String{value:"volfina".to_string()};
        assert_eq!(x.next(), "volfina".to_string());
        assert_eq!(x.next(), "volfinb".to_string());
        assert_eq!(x.next(), "volfinc".to_string());
    }

    #[test]
    fn test_Inc1() {

        let mut x = Inc1{value:10};
        assert_eq!(x.next(), 10);
        assert_eq!(x.next(), 11);
        assert_eq!(x.next(), 12);
        assert_eq!(x.next(), 13);
    }

}
