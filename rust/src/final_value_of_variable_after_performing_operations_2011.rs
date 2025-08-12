use crate::Solution;
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut out = 0;
        for op in operations {
            match op.as_str() {
                "++X" | "X++" => {
                    out +=1;
                },
                "--X" | "X--" => {
                    out -= 1;
                },
                _ => {

                }

            }
        }
        out
    }
}
