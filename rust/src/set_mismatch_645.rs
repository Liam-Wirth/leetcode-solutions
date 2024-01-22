// when in doubt throw a hashset at it!!!
use std::collections::HashSet;
impl Solution {
    /* How to solve?
     * sort the vec
     * Loop through the vector, adding the values contained within the vector
     * to the hashset, keep looping untill we find a value that already exists in the hashmap (A
     * duplicate), we now know the index and location, we then that the vector is sorted, so     * all we need to do is replace the value at position N, with the value of position (n-1 +1)
     * then we break the loop, and return the vector (AWESOME STYLE!@!!11)
     */

    /*
     * sort the array
     * loop through the vector, with index i, and checking the nums as we go
     * if (i-1) !< (i)
     *(i-1)++ ?
     */
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut check: HashSet<i32> = HashSet::new();
        let mut out: Vec<i32> = Vec::new();
        for (i, num) in nums.iter_mut().enumerate() {
            if check.contains(num) {
                out.push(*num);
            }
            check.insert(*num);
        }
        let n: usize = nums.len();
        //NOTE: I could also just use i..=n to include n, but n+1 is a janky fix that works too.
        for i in 1..n + 1 {
            if !check.contains(&(i as i32)) {
                out.push(i as i32);
            }
        }
        out
    }
}
/*
class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        seen = set()
        cur_sum = 0
        for num in nums:
            if num in seen:
                dup = num
            else:
                cur_sum += num
                seen.add(num)
        target_sum = (len(nums) + 1) * len(nums) // 2
        return [dup, target_sum - cur_sum]
        */
//NOTE:credit to killer_whale for this second, much more clever solution:
//only included in the repo for the sake of learning :)
impl Solutions {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut cur_sum: i32 = 0;
        let mut out: Vec<i32> = Vec::new();
        for num in nums.iter() {
            if seen.contains(&num) {
               out.push(num)
            } else {
                cur_sum += num;
                seen.insert(num);
            } 
        }
        target_sum = (nums.len() + 1 ) * len(nums) / 2;
        //NOTE: Better solution, as we are able to do everything in one, pass, and obviously, the
        //missing number would be equivalent to the sum of everything we saw in the given array
        //subtracted fromm what the actual sum of the array would be
        out.push(target_sum-cur_sum);
        out
    }
    //TODO: Look into solving this using sum of squares! 
    //Thanks lebron!
    //https://leetcode.com/problems/set-mismatch/submissions/1153157349/?envType=daily-question&envId=2024-01-22
    //
}
