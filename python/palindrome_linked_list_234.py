# Definition for singly-linked list.
 from typing import Optional


class ListNode:
     def __init__(self, val=0, next=None):
         self.val = val
         self.next = next
class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        vals = []
        walk = head
        while walk:
            vals.append(walk.val)
            walk = walk.next
        return vals == vals[::-1]

    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        # list with no, or 1 value is by definition a palindrome
        if not head or not head.next:
            return True 

        slow = fast = head

        #first we need to find the middle of the list using two pointers
        # by advancing the fast pointer twice as fast as the slow pointer, it holds that by the time 
        # the fast pointer hits the end of the list, the slow pointer will be halfway to the end of the list, thus, at the midpoint
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next

        # now we reverse the second half of the list:

        prev = None
        while slow:
            temp = slow.next
            slow.next = prev
            prev = slow
            slow = temp


        # now we are basically comparing two lists (first and second half), 
        left = head # first half
        right = prev # second half

        while right:
            if left.val != right.val:
                return False
            left = left.next
            right = right.next
        # if we dont return in the above, then it's a palindromic linked list and we can return true
        return True




