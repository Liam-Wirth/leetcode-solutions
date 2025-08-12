use std::num::ParseIntError;
use crate::Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
       let mut reversed:i32 = 0;
       let mut num = x;

       while num != 0 {
            let dig = num %10;
            num /=10;

            // Overflow Check
            if reversed > i32::MAX / 10 || (reversed == i32::MAX /10 && dig > i32::MAX % 10) {
                return 0;
            }
            if reversed < i32::MIN / 10 || (reversed == i32::MIN/10 && dig < i32::MIN % 10) {
                return 0;
            }
            reversed = reversed*10 + dig;
       }
       reversed
    }

    pub fn dumbreverse(x: i32) -> i32 {
        // this might be a dumb solution I bet the actual solution is bitwise/has something to do with twos compliment? I dunno

        let mut tmp: Vec<char> = x.to_string().chars().collect();

        // Thought process in a software environment right, a signed value will have half the range of an unsigned
        // value, so all we gotta do is remove the sign to handle that "int overflow" issue temporarily, and then check that unsigned number to see if it's outside the range(either under or overflow)
        let base: i32 = 2;
        if tmp[0] == '-' {
            let mut tmp1: Vec<char> = tmp.iter().copied().rev().collect();
            tmp1.pop();
            let tmp1_str: String = tmp1.iter().collect();
            let tmp1_result: Result<u32, ParseIntError> = tmp1_str.trim().parse();

            match tmp1_result {
                Ok(tmp1_val) => {
                    if tmp1_val > base.pow(31) as u32 {
                        return 0;
                    } else {
                        return -1 * tmp1_val as i32;
                    }
                }
                Err(_e) => { // Pattern match on ParseIntError, you can name it `_e` to indicate it's not used
                    // Handle the error, for example, return 0 as in your original code for error cases
                    return 0;
                }
            }
        } else {
            let tmp1_str: String = tmp.iter().copied().rev().collect();
            let tmp1_result: Result<u32, ParseIntError> = tmp1_str.trim().parse();
            match tmp1_result {
                Ok(v) => {
                    if v > (base.pow(31) - 1) as u32 {
                        return 0;
                    } else {
                        return v as i32;
                    }
                }
                Err(_e) => { // Pattern match on ParseIntError
                    // Some parse int error
                    return 0;
                }
            }
        }
        0
    }
}

