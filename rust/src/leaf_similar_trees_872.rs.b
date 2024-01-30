//TODO: Finish this 
use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        false
    }
    pub fn traverse_and_store(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
         if let Some(node) = root {
             if node.borrow().left.is_some() {
                out = Self::traverse_and_store(node.borrow().left.clone());
             } else {
                 while node.borrow().right.is_some() {
                     out.push(node.borrow().val);
                    node = node.borrow().right.clone();
                    }
                 }
             }
         }
        //returning empty array for now
        vec![]
    }
}
