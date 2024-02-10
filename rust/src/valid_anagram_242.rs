use crate::Solution;
use std::collections::HashMap;
//RACECAR S  hm (r,2),(a,2),(C,2),(E,1)
//CAR T      hm1 (r,1),(a,1),(c,1)
//cccar      hm2 (r,1),(a,1), (c, 3) v > hm1.v

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hm: HashMap<char, i32> = HashMap::new();
        let mut hm1: HashMap<char, i32> = HashMap::new();
        for ch in s.chars().into_iter(){
            *hm.entry(ch).or_insert(0) +=1;
        }
        for ch in t.chars().into_iter(){
            *hm1.entry(ch).or_insert(0) += 1;
        }
        //if is anagram && same length
        if hm.eq(&hm1) {
            return true;
        } 
    //else {
            //WHAAAAAT I don't need to call .iter()??? WHAAAAAT
    //        for (k, v) in &hm1 {
    //            if !hm.contains_key(k) || v > hm.get(k).unwrap() {
    //                return false
    //            } 
     //       }
    //    }
        false   
    }
}
