use crate::Solution;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1_parts: Vec<i32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
        let v2_parts: Vec<i32> = version2.split('.').map(|s| s.parse().unwrap()).collect();
        let len = v1_parts.len().max(v2_parts.len());

        for i in 0..len {
            let v1_num = if i < v1_parts.len() { v1_parts[i] } else { 0 };
            let v2_num = if i < v2_parts.len() { v2_parts[i] } else { 0 };

            if v1_num < v2_num {
                return -1;
            } else if v1_num > v2_num {
                return 1;
            }
        }

        0
    }
}


