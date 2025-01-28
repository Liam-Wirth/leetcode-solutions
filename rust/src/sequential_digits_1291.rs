use crate::Solution;
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        //Dumb solution first (o(n), not at ALL scaleable);
        let mut out: Vec<i32> = Vec::new();
        let brute: Vec<i32> = vec![
        12, 23, 34, 45, 56, 67, 78, 89, 
        123, 234, 345, 456, 567, 678, 789, 
        1234, 2345, 3456, 4567, 5678, 6789, 
        12345, 23456, 34567, 45678, 56789, 
        123456, 234567, 345678, 456789, 
        1234567, 2345678, 3456789, 
        12345678, 23456789, 
        123456789
        ];
        for num in brute.iter(){
            if num >= &low && num <= &high {
                out.push(*num);
            } 
        }
        out
    }
}
