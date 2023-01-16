pub struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);
        let mut i = 0;
        //here I am comparing intervals and trying to find instances of overlap
        //I then store these instances of overlap temperarily in the answer vector.
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            answer.push(intervals[i].clone());
            i += 1;
        }
        //shadowing the variable with a mutable version of itself (like a boss)
        let mut new_interval = new_interval;
        //NOTE: using the new_interval variable to store wherever a merge would need to happen
        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            //NOTE:  the final merged start time will be the minimum of the current and the new
            new_interval[0] = std::cmp::min(new_interval[0], intervals[i][0]);
            //NOTE: the final merged end time will be the maximum of the current and the new
            new_interval[1] = std::cmp::max(new_interval[1], intervals[i][1]);
            i += 1;
        }
        //NOTE:pushing the merged intervals to the answer
        answer.push(new_interval);
        //NOTE: adding any unmerged intervals to the final answer
        //reusing I because I know it is already going to be the index of the last merge+1,
        //therefore I can just add everything found in the original array after that.
        while i < intervals.len() {
            answer.push(intervals[i].clone());
            i += 1;
        }
        answer
    }
}

fn main() {
    let intervals: Vec<Vec<i32>> = vec![[1, 3].to_vec(), [6, 9].to_vec()];
    let new_interval: Vec<i32> = vec![2, 5];
    Solution::insert(intervals, new_interval);
}
