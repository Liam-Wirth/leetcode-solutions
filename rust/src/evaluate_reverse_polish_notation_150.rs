use crate::Solution;
impl Solution {
 pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        if tokens.len() == 1 {
            return tokens.pop().unwrap().parse::<i32>().unwrap();
        }
        let mut stack: Vec<i32> = Vec::new();

       for token in tokens.iter(){
           if let Ok(n) = token.parse::<i32>() {
             stack.push(n);
           }else{
               let op2 = stack.pop().unwrap();
               let op1 = stack.pop().unwrap();
            match token.as_str() {
                "+" => stack.push(op1 + op2),
                "-" => stack.push(op1 - op2),
                "*" => stack.push(op1 * op2),
                "/" => stack.push(op1 / op2),
                _   => panic!("this won't happen")
            };
        }
       }
        stack[0]
    }

}
