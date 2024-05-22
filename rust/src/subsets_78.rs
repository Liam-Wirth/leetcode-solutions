// TODO: Revisit
use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out:Vec<Vec<i32>> = Vec::new();
        let mut subset: Vec<i32> = Vec::new();

        Self::balls(&nums, 0, &mut subset, &mut out);
        out
    }
    pub fn balls(nums: &Vec<i32>, i: i32, sub: &mut Vec<i32>, subsets: &mut Vec<Vec<i32>>) {
        subsets.push(sub.to_vec());
        for poo in i..nums.len() as i32{
            sub.push(nums[poo as usize]);
            Self::balls(nums, poo+1, sub, subsets);
            sub.remove(sub.len());

        }
    }
}
