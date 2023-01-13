pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum_left: i32 = 0;
    let sum: i32 = nums.iter().sum();
    for (i, &val) in nums.iter().enumerate() {
        if sum - sum_left - val == sum_left {
            return i as _;
        }
        sum_left += val;
    }
    -1
}
fn main() {}
