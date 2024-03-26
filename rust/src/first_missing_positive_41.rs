use crate::Solution;
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        //I misread the problem, I incorrectly assumed the array was unsorted, and contained values going from 0-n
        //my previous solution would have worked if that was the case

        //this is also stupid, but could work. Obviously I could just use a hashset but that's kinda boring

        // I'm confusing the fact that accessing values in a hashset/hashmap takes O(1) time

        //get rid of all the edge cases we don't care about
        let n = nums.len() as i32;
        for num in &mut nums {
            if *num <= 0 || *num > n {
                *num = n + 1; //our value has to be within the range 1..n
            } else {
                continue;
            }
        }

        for (index, &num) in nums.iter().enumerate() {
            let num_index = num.abs() - 1;
            if num_index < n as usize {
                nums[num_index as usize] = -nums[num_index as usize].abs();
            }
        }

        // Finding the first missing positive integer
        for (index, &num) in nums.iter().enumerate() {
            if num >= 0 {
                return (index + 1) as i32;
            }
        }
        n + 1
    }
}
