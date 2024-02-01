use crate::Solution;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let mut result = String::new();
        let mut carry: u8 = 0;
        while a.len() < b.len() {
            a.insert(0, '0');
        }
        while b.len() < a.len() {
            b.insert(0, '0');
        }
        let a = a.chars().collect::<Vec<_>>();
        let b = b.chars().collect::<Vec<_>>();
        for i in (0..a.len()).rev() {
            let mut bitstring: String = String::new();
            bitstring.push(a[i]);
            bitstring.push(b[i]);
            bitstring.push(if carry == 1 { '1' } else { '0' });
            match &bitstring[..] {
                "000" => {
                    result.push('0');
                    carry = 0;
                }
                "001" => {
                    result.push('1');
                    carry = 0;
                }
                "010" => {
                    result.push('1');
                    carry = 0;
                }
                "011" => {
                    result.push('0');
                    carry = 1;
                }
                "100" => {
                    result.push('1');
                    carry = 0;
                }
                "101" => {
                    result.push('0');
                    carry = 1;
                }
                "110" => {
                    result.push('0');
                    carry = 1;
                }
                "111" => {
                    result.push('1');
                    carry = 1;
                }
                _ => panic!("This won't happen"),
            }
        }
        if carry == 1 {
            result.push('1');
        }
        return result.chars().rev().collect();
    }
}
