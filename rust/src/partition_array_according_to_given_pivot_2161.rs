use crate::Solution;
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut before = Vec::new();
        let mut after = Vec::new();
        let mut eq = 0;

        for i in nums {
            if i < pivot {
                before.push(i);
            } else if i == pivot {
                eq += 1
            } else {
                after.push(i);
            }
        }
        println!("{:?}", before);
        before.extend(vec![pivot;eq]);
        before.extend(after);
        before
    }
}
