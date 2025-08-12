# Definition for a binary tree node.
from random import vonmisesvariate
from typing_extensions import Deque


class TreeNode:
 def __init__(self, val=0, left=None, right=None):
     self.val = val
     self.left = left
     self.right = right
class Solution:
     def largestValues(self, root: Optional[TreeNode]) -> List[int]:
        out = []
        q = [root]
        visited = set()

        def bfs(node):
            if not node:
                return
            q = [node]
            visited.add(node)
            while q:
                next_q = []
                max_val = float('-inf')
                for node in q:
                    print(node.val)
                    max_val = max(max_val, node.val)
                    if node.left and node.left not in visited:
                        next_q.append(node.left)
                        visited.add(node.left)
                    if node.right and node.right not in visited:
                        next_q.append(node.right)
                        visited.add(node.right)
                out.append(max_val)
                q = next_q

            bfs(root)
            

