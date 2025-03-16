use crate::Solution;
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut left = 1;
        let mut right = *ranks.iter().min().unwrap() as i64 * cars as i64 * cars as i64; 

        // Use a closure (move keyword to take ownership)
        let can_repair = |time: i64| -> bool {
            let mut total = 0;
            for rank in &ranks { // Iterate over a *reference* 
                let repaired = (time as f64 / *rank as f64).sqrt() as i64; 
                total += repaired;
                if total >= cars as i64 {
                    return true;
                }
            }
            false
        };

        while left < right {
            let mid = (left + right) / 2;
            if can_repair(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

