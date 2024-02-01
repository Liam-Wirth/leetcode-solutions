impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut result:Vec<Vec<i32>> = Vec::new();
        for chunk in nums.chunks(3) {
            if chunk[2] - chunk[0]<= k {
               result.push(chunk.to_vec());
            } else {
                return
            }
            result
        }
    }
}
