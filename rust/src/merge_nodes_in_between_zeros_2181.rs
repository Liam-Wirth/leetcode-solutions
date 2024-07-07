use crate::Solution;
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur_out = &mut dummy;
        let mut cur = head.unwrap().next;  // skip the first 0 node
        let mut running_val = 0;

        while let Some(mut node) = cur {
            if node.val != 0 {
                running_val += node.val;
            } else {
                if running_val != 0 {
                    let new_node = Box::new(ListNode::new(running_val));
                    cur_out.next = Some(new_node);
                    cur_out = cur_out.next.as_mut().unwrap();
                    running_val = 0;
                }
            }
            cur = node.next.take();
        }

        dummy.next
    }
}

