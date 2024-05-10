use crate::Solution;
// TODO: Revisit lebob this one pls this solution doesnt work
//
// use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut left = 0.0;
        let mut right = 1.0;
        let mut result = vec![0, 0];

        while right - left > 1e-9 {
            let mid = (left + right) / 2.0;
            let mut count = 0;
            let mut max_fraction = 0.0;
            let mut numerator = 0;
            for i in 0..arr.len() {
                let val = arr[i] as f64;
                let mut low = 0;
                let mut high = arr.len() - 1;
                let mut ans = 0;
                while low <= high {
                    let mid = (low + high) / 2;
                    if val / arr[mid] as f64 <= mid {
                        ans = mid;
                        high = mid - 1;
                    } else {
                        low = mid + 1;
                    }
                }
                count += arr.len() - ans;
                if ans < arr.len() {
                    let fraction = val / arr[ans] as f64;
                    if fraction > max_fraction {
                        max_fraction = fraction;
                        numerator = arr[i];
                    }
                }
            }
            if count == k {
                result = vec![numerator, arr[arr.len() - 1]];
                break;
            } else if count < k {
                left = mid;
            } else {
                right = mid;
                result = vec![numerator, arr[arr.len() - 1]];
            }
        }

        result
    }
}

