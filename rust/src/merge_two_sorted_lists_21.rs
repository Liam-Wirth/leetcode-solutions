use crate::Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = &mut list1;
        //note: this line exists to ensure safety/avoid an error condition when we approach the end of the list we are iterating through
        while list2.is_some() {
            //remember the question mark operator is being used to help ensure memory safety, basicaly, if the code after the question mark throws an error
            // then the line will not be ran
            if ret.is_none() || list2.as_ref()?.val < ret.as_ref()?.val {
                //instead of inserting things into a new list, we are just swapping the values of the 
                std::mem::swap(ret, &mut list2);
            }
            ret = &mut ret.as_mut()?.next;
        }
        list1
    }
}
