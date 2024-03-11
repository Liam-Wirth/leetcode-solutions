use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = std::collections::VecDeque::new();
        let mut even: i32 = root.unwrap().borrow().val % 2; //
        let stack = &mut Vec::with_capacity(16);
        Solution::dfs(root, 0, stack)
    }
    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, stack: &mut Vec<i32>) -> bool {
        match node {
            Some(node) => {
                let borrowed = node.borrow();
                // println!("{stack:?} => {depth} => {}", borrowed.val);
                if depth < stack.len() {
                    let prev_val = stack[depth];
                    if depth & 1 == 0 {
                        if !(borrowed.val & 1 == 1) {
                            return false;
                        }
                        if borrowed.val <= prev_val {
                            return false;
                        }
                    } else {
                        if !(borrowed.val & 1 == 0) {
                            return false;
                        }
                        if borrowed.val >= prev_val {
                            return false;
                        }
                    }
                    stack[depth] = borrowed.val;
                } else {
                    if depth & 1 == 1 {
                        if borrowed.val & 1 != 0 {
                            return false;
                        }
                    } else {
                        if borrowed.val & 1 != 1 {
                            return false;
                        }
                    }
                    stack.push(borrowed.val);
                }
                Solution::dfs(borrowed.left.clone(), depth + 1, stack)
                    & Solution::dfs(borrowed.right.clone(), depth + 1, stack)
            }
            _ => true,
        }
    }
}
