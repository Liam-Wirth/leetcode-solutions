#ermmm what the FLIP leetcode, why was there NO HECKIN RUST SOLUTION!!!!


 Definition for singly-linked list.
 class ListNode:
     def __init__(self, x):
         self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        hare = tortoise = head
        
        while hare and hare.next:
            tortoise, hare = tortoise.next, hare.next.next
            if hare == tortoise:
                return True
        
        return False
