use ndarray::{Array1,Array2,arr1};
use crate::setti::selection_rule;

/*
each selection rule is accompanied by a binary vector, 1 is greedy search.
*/
#[derive(Clone)]
pub struct BFGSelectionRule {
    pub sr: selection_rule::SelectionRule,
    // column-wise index in search
    pub i:usize,
    pub next_path:Vec<usize>
}

pub fn build_BFGSelectionRule(sr:selection_rule::SelectionRule) -> BFGSelectionRule {
    BFGSelectionRule{sr:sr,i:0,next_path:Vec::new()}
}

pub fn default_BFGSelectionRule(r:usize,c:usize) -> BFGSelectionRule {
    let rq:Array2<i32> = Array2::ones((r,c));
    let rs:Array2<i32> = Array2::zeros((r,c));
    let sr = selection_rule::SelectionRule{res:selection_rule::Restriction{data:rs},req:selection_rule::Requirement{data:rq},choice:Vec::new()};
    BFGSelectionRule{sr:sr,i:0,next_path:Vec::new()}
}

impl BFGSelectionRule {

}

/*
brute-force greedy searcher;
designed as a generic structure used to select options given a SelectionRule.
Generization is implemented by the function f,
exterior to struct.
*/
pub struct BFGSearcher {

    /*
    p = cache[0]
    tmp_cache <- search(p)
    tmp_cache -> (cache|all_cache)
    */
    cache:Vec<BFGSelectionRule>,
        // for each rule in tmp_cache
    tmp_cache:Vec<BFGSelectionRule>,
    all_cache:Vec<BFGSelectionRule>,

    // timestamp 0..|selection_rule.columns|
    pub ts:usize
}

/*
*/
pub fn build_BFGSearcher(x: BFGSelectionRule) -> BFGSearcher {
    BFGSearcher{cache:vec![x],tmp_cache:Vec::new(),all_cache:Vec::new(),ts:0}
}

impl BFGSearcher {

    pub fn next_srs(&mut self, sri :Option<usize>) {
        // case: brute
        //////
        if sri.is_none() {
            // iterate through tmp_cache
            let mut l = self.tmp_cache.len();
            while l > 0 {
                let mut x = self.tmp_cache[0].clone();
                self.tmp_cache = self.tmp_cache[1..].to_vec();

                x.next_path.push(0);
                self.cache.push(x);
                l = self.tmp_cache.len();
            }
            return;
        }

        // case: greedy by choice
        let mut x = self.tmp_cache[sri.unwrap()].clone();
        x.next_path.push(1);
        self.cache.push(x);
        self.tmp_cache = Vec::new();
    }

    /*
    processes one element in cache
    */
    pub fn process(&mut self) -> bool {

        if self.cache.len() == 0 {
            return false;
        }

        let mut c = self.cache[0].clone();
        let l = self.next_srs_(&mut c).len();
        // case: no more elements, finished
        if l == 0 {
            self.all_cache.push(c);
        }
        
        self.cache = self.cache[1..].to_vec();
        true
    }

    /*
    loads next batch into memory, and prompt for outside class to yield decision
    */
    pub fn next_srs_(&mut self, s: &mut BFGSelectionRule) ->
        Vec<BFGSelectionRule> {

        let mut sol: Vec<BFGSelectionRule> = Vec::new();

        // consider all choices
        let ci = s.sr.choices_at_col_index(s.i);

        // iterate through and make SelectionRule sibling for each c in ci
        for c in ci.into_iter() {
            let mut s2 = s.clone();
            s2.sr.select_choice_at_col_index(s.i,c,true);
            s2.i += 1;
            sol.push(s2);
        }

        self.tmp_cache = sol.clone();
        return sol;
    }
}
