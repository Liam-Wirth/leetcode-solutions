class Solution:
    def bstFromPreorder(self, preorder: List[int]) -> Optional[TreeNode]:
        stack = []
        N = len(preorder)
        root = TreeNode(preorder[0])
        stack.append(root) # the value at the back of the stack can replace a "walk" pointer because we will assume to be going leftwards the whole time
        for i in range(1,N):
            val = preorder[i]
            walk = stack[-1]
            if val < walk.val:
                walk.left = TreeNode(val)
                stack.append(walk.left)
            else:
                while stack and preorder[i] > stack[-1].val:
                    last = stack.pop()
                last.right = TreeNode(val)
                stack.append(last.right)
        return root
        
