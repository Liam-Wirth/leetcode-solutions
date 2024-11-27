use crate::Solution;
use std::collections::HashMap;
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut hm = HashMap::new();
        
        let mut out: Vec<char> = Vec::new();
        
        for ch in s.chars() {
            *hm.entry(ch).or_insert(0) += 1;
        }
        
        
        for ch in order.chars() {
            if let Some(temp) = hm.remove(&ch){
                for i in 0..temp {
                    out.push(ch);
                }
            }
        }


        let mut temp: Vec<(&char, &i32)> = hm.iter().collect();
        
        for i in temp {
            for j in 0..(*i.1){
                out.push(*i.0);
            }
        }
        
        
        out.iter().collect()
    }
}
