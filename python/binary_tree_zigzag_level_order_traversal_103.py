# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
from collections import deque
from this import d
from typing import Deque
from typing import Optional


class Solution:
  #left to right = FIFO
  # right to left = FILO
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> list[list[int]]:
        if not root:
            return []
        ret = []
        q = deque([root])
        lvl = 0

        while q:
            curr = []
            for i in range(len(q)):
                if lvl % 2 == 0: # left to right FIFO
                    n = q.popleft() #FO (first out) (assures we process in the ORDER THEY ARE INSERTED)
                    curr.append(n.val)

                    if n.left:
                        q.append(n.left) # somehow still first in
                    if n.right:
                        q.append(n.right)
                else: # right to left FILO 
                    n = q.pop() # "LO" (last out), returning the RIGHTMOST element (last) in the list
                    curr.append(n.val)
                    if n.right:
                        q.appendleft(n.right) # first in, makes sure it would be the 'last out'
                    if n.left:
                        q.appendleft(n.left) # this ensures the left node does get processed after the right node
            ret.append(curr)
            lvl +=1
        return ret






