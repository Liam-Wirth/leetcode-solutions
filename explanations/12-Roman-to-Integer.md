# Intuition
I initally tried to solve this problem using a hashmap, but that introduced problems involving the order in which the hashmap was iterated. After a lot of debugging and asking friends, I determined it would be easier to solve the algorithm using a vector of tuples. This vector of tuples is iterated over from the highest value, down to the lowest value, and compared to the number given. 
# Approach
Greedy

# Complexity
- Time complexity: 
   $$O(n)$$
  - the algorithm is iterating over the symbols, and for each iteration it performs a constant amount of operations that doesnt change.
- Space complexity:
   $$0(1)$$
  - This is O(1) because I am solving this problem using a vector of tuples. This vector of tuples is declared once, and doesn't get changed in size later.
# Code
```
  
impl Solution {
pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut out = String::new();
    let conv = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];
    for &(symbol, val) in conv.iter() {
        while num >= val {
            out.push_str(&symbol);
            num -= val;
        }
    }

    out
}
 
}

```
