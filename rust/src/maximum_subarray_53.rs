impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
      // Implementation of Kadane's algorithm (in rust yayayaa)
      let mut best = i32::MIN;
      let mut curr = 0;

      for i in 0..nums.len() {
        curr = std::cmp::max(nums[i], curr + nums[i]);
        best = best.max(curr);
      }
      best
    }
}

