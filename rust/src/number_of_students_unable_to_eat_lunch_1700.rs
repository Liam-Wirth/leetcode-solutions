use crate::Solution;
impl Solution {
 
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let (mut count, n) = (vec![0; 2], students.len());
        for s in students { count[s as usize] += 1 }
        for (i, &s) in sandwiches.iter().enumerate() {
            count[s as usize] -= 1;
            if count[s as usize] < 0 { return (n - i) as i32 }
        }; 0
    }
}
