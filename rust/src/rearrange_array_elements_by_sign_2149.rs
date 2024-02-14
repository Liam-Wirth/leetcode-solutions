use crate::Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {

        let mut pos: Vec<i32> = Vec::new();
        let mut neg: Vec<i32> = Vec::new();

        for num in nums.iter() {
            if *num >= 0 {
                pos.push(*num);
            } else {
                neg.push(*num);
            }
        };
        let mut out : Vec<i32> = Vec::new();
    for (i , num) in pos.iter().enumerate() {
        out.push(*num);
        out.push(neg[i]);
    };
    out
}
}