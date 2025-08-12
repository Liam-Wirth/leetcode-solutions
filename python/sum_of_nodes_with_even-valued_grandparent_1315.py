# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sumEvenGrandparent(self, root: Optional[TreeNode]) -> int:
        def dfs(node, par, grand) -> int:
            if not node:
                return 0
            sumv = 0
            if grand is not None and grand % 2 == 0:
                sumv += node.val
            sumv += dfs(node.left, node.val, par)
            sumv += dfs(node.right, node.val, par)
            
            return sumv
        out = dfs(root, None, None)
        return out
