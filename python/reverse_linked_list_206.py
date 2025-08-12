# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        walk = head
        if not head:
            return head
        elif not head.next:
            return head
        
        seen = []
        while walk.next:
            seen.append(walk.val)
            walk = walk.next
        
        print(seen)
        
        seen = seen[::-1]
        
        out = walk
        for i in seen[:-1]: # iterate up to last value
            walk.next = ListNode(val=i)
            walk = walk.next
        walk.next = ListNode(head.val)
        return out
            
            
            
        
        
            
        
