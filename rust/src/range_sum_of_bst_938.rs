#![allow(unused_imports)]
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

impl Solution {
    pub fn traverse_and_add(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> i32 {
        if let Some(node) = root {
            let mut sum = 0;
            if node.borrow().val >= low && node.borrow().val <= high {
                sum += node.borrow().val;
            }
            sum += Solution::traverse_and_add(node.borrow().left.clone(), low, high);
            sum += Solution::traverse_and_add(node.borrow().right.clone(), low, high);

            sum
        } else {
            0
        }
    }

    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Solution::traverse_and_add(root, low, high)
    }
}fn main() {
}

