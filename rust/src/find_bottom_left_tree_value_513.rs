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
use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
use crate::Solution;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root.unwrap()); //unwrap is safe here because we know the root has to be some
            let mut out = 0;

            while let Some(node) = queue.pop_front() {
                out = node.borrow().val; 
                if let Some(right) = node.borrow().right.clone() {
                    queue.push_back(right);
                }
                if let Some(left) = node.borrow().left.clone() {
                    queue.push_back(left);
                }
            }
            out
        }
    }
