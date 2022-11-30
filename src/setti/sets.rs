/*
two classes of seeds:
- selector seeds on Vec<string>
- numerical seeds
*/
//////////////////////////////////////////////////////////////////
use crate::setti::inc;

pub struct ISeed {
    data: Vec<inc::Inc1String>
}

impl ISeed {

    pub fn init(&mut self) {
        let l = self.data.len();
        for i in 0..l {
            self.data[i].inc_();
        }
    }

    pub fn next(&mut self,i:usize) -> String {
        self.data[i].inc_()
    }
}

