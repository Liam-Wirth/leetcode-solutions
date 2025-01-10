use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        // Step 1: Build the combined max frequency requirements from words2
        let mut maxf = HashMap::new();
        for b in &words2 {
            let freq_b = Self::char_frequency(b);
            for (&ch, &count) in &freq_b {
                maxf.insert(ch, maxf.get(&ch).cloned().unwrap_or(0).max(count));
            }
        }

        // Step 2: Check each word in words1
        let mut result = Vec::new();
        for a in &words1 {
            let freq_a = Self::char_frequency(a);
            if maxf.iter().all(|(&ch, &count)| freq_a.get(&ch).cloned().unwrap_or(0) >= count) {
                result.push(a.clone());
            }
        }

        result
    }

    // Helper function to calculate character frequency
    fn char_frequency(word: &str) -> HashMap<char, usize> {
        let mut freq = HashMap::new();
        for ch in word.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }
        freq
    }
}

