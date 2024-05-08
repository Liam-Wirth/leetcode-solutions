use crate::Solution;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut out: Vec<String> = vec!["".to_string(); score.len()];
        let mut score_index_map: HashMap<i32, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        // Populate score_index_map and heap
        for (i, &s) in score.iter().enumerate() {
            score_index_map.insert(s, i);
            heap.push(s);
        }
        
        let medals = ["Gold Medal", "Silver Medal", "Bronze Medal"];
        let mut rank = 1;
        
        // Iterate through sorted scores
        while let Some(top_score) = heap.pop() {
            let index = score_index_map[&top_score];
            match rank {
                1..=3 => out[index] = medals[rank - 1].to_string(),
                _ => out[index] = rank.to_string(),
            }
            rank += 1;
        }
        
        out
    }
}
