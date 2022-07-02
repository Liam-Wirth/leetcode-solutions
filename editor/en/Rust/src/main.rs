fn main() {
    let balls: &str = "FUCK!";
   println!("in the outer scope, the variable balls is {}", balls);
   {
       let balls: u64 = 42;
       println!("in this inner scope, balls references a different datatype and is {}", balls);
   } 
   println!("now that I have exited the innerscope, balls has returned too {}", balls)
}
