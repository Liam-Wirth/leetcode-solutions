use crate::Solution;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //I was initially thinking that this was array based, and I was going to have to dive through the tree, 
        //and mathematically find each left node (the next left node would be 2*i + 1 iirc), but this is actually
        //alot easier than that
        match root {
            Some(node) => {
                let temp = node.borrow();
                let mut sum = 0;
                if let Some(ref left) = temp.left {
                    let left_val = left.borrow().val;
                    if left.borrow().left.is_none() && left.borrow().right.is_none() {
                        sum += left_val;
                    }
                }
                sum + Solution::sum_of_left_leaves(temp.left.clone()) + Solution::sum_of_left_leaves(temp.right.clone())
            }
            None => 0,
        }
    }
}
