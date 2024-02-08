use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut hm: HashMap<char, usize> = HashMap::new();

        // Count the frequency of each character
        for ch in s.chars() {
            *hm.entry(ch).or_insert(0) += 1;
        }

        // Sort the characters by their frequencies in descending order
        let mut sorted_map: Vec<_> = hm.into_iter().collect();
        sorted_map.sort_by(|a, b| b.1.cmp(&a.1));

        // Construct the resulting string based on the sorted characters
        let mut out = String::new();
        for (ch, freq) in sorted_map {
            for _ in 0..freq {
                out.push(ch);
            }
        }

        out
    }
}
