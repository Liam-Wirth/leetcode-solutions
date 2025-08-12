use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut hm: HashMap<char, Vec<usize>> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            hm.entry(c).or_insert(Vec::new()).push(i);
        }

        let mut count = 0;
        for word in words {
            let mut point = 0;
            let mut is_subseq = true;

            for c in word.chars() {
                if let Some(indices) = hm.get(&c) {
                    match Self::bin_search(indices, &point) {
                        Some(idx) => point = indices[idx] + 1,
                        None => {
                            is_subseq = false;
                            break;
                        }
                    }
                } else {
                    is_subseq = false;
                    break;
                }
            }

            if is_subseq {
                count += 1;
            }
        }
        count
    }

fn bin_search(arr: &[usize], target: &usize) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = arr.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_val = arr[mid as usize];

        if mid_val == *target {
            return Some(mid as usize);
        } else if mid_val < *target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    if low < arr.len() as i32 {
        return Some(low as usize);
    } else {
        return None;
    }
}
}
