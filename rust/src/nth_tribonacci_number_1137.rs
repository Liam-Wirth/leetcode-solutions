use crate::Solution;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n<2 {
            return n
        }
        let mut n = n;
        let (mut a, mut b, mut c, mut d) = (0,1,1,0);
        while (n > 2){
            d = a+b+c;
            a=b;
            b=c;
            c=d;
            n-=1;
        }
        c
    }
}
 fn main() {
    unimplemented!();
}
