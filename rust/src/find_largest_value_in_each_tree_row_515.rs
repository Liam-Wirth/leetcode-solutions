use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::bfs(root, &mut res);
        res
    }

    fn bfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let mut q = vec![root];
        while !q.is_empty() {
            let mut next_q = vec![];
            let mut max_val = i32::MIN;

            for node_opt in q.iter() {
                if let Some(node_rc) = node_opt {
                    let node = node_rc.borrow();
                    max_val = max_val.max(node.val);

                    if let Some(left_rc) = node.left.clone() {
                        next_q.push(Some(left_rc));
                    }
                    if let Some(right_rc) = node.right.clone() {
                        next_q.push(Some(right_rc));
                    }
                }
            }

            res.push(max_val);
            q = next_q;
        }
    }
}
