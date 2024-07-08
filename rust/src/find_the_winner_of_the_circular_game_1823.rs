use crate::Solution;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}
// NOTE: the custom circular linked list solution does not work sadly, it's super close but is
// consistently off by one for some reason I cant figure out why, which is sad

// TODO: Revisit and maybe address above note (see fn find_the_winner_old)

impl Node {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

impl Solution {
    pub fn find_the_winner_old(n: i32, k: i32) -> i32 {
        let mut head = Node::new(1);
        let mut prev = Rc::clone(&head);

        // Create the circular linked list
        for i in 2..=n {
            let new_node = Node::new(i);
            prev.borrow_mut().next = Some(Rc::clone(&new_node));
            prev = new_node;
        }
        // Link the last node to the head to make it circular
        prev.borrow_mut().next = Some(Rc::clone(&head));

        let mut cur_node = Rc::clone(&head);

        // Iterate to remove nodes
        let mut num = n;
        while num > 1 {
            // Move k-1 steps to find the k-th node
            for _ in 0..(k) {
                let next = Rc::clone(cur_node.borrow().next.as_ref().unwrap());
                cur_node = next;
            }
            // Remove the k-th node
            let next_node = Rc::clone(cur_node.borrow().next.as_ref().unwrap());
            cur_node.borrow_mut().next = next_node.borrow().next.clone();
            cur_node = next_node;
            num -= 1;
        }

        let x = cur_node.borrow().val;
        x
    }
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends: Vec<i32> = (1..=n).collect();
        let mut cur_index: usize = 0;
        
        while friends.len() > 1
        {
            cur_index = (cur_index + (k as usize - 1)) % friends.len();
            friends.remove(cur_index);
        }
        friends[0]
    
    }
}
