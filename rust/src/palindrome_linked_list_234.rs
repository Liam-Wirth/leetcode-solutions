use crate::Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut slow = &head;
        let mut fast = &head;

        // Find the middle of the list
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        // Reverse the second half
        let mut prev = None;
        let mut current = slow.clone();
        while let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        // Compare both halves
        let mut left = &head;
        let mut right = &prev;
        while right.is_some() {
            if left.as_ref().unwrap().val != right.as_ref().unwrap().val {
                return false;
            }
            left = &left.as_ref().unwrap().next;
            right = &right.as_ref().unwrap().next;
        }

        true
    }
}

   
