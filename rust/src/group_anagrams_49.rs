use crate::Solution;
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());
        let offset = 'a' as usize;
        for s in strs {
            let mut key: [u8; 26] = [0; 26];
            for char in s.chars() {
                key[char.to_ascii_lowercase() as usize - offset] += 1;
            }
            hm.entry(key)
                .and_modify(|v| v.push(s.clone()))
                .or_insert(vec![s]);
        }
        hm.values().cloned().collect::<Vec<Vec<String>>>()
    }
}
