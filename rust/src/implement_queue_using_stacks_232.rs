struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        return MyQueue {
            stack1: vec![],
            stack2: vec![],
        };
    }

    fn push(&mut self, x: i32) {
        self.stack2.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack1.len() == 0 {
            while self.stack2.len() > 0 {
                self.stack1.push(self.stack2.pop().unwrap());
            }
        }
        self.stack1.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.stack1.len() == 0 {
            while self.stack2.len() > 0 {
                self.stack1.push(self.stack2.pop().unwrap());
            }
        }
        *self.stack1.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack1.len() + self.stack2.len() == 0
    }
}

/*
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
