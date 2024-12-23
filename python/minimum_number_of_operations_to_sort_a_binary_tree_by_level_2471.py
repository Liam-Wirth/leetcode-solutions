# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


from collections import deque


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def minimumOperations(self, root: Optional[TreeNode]) -> int:
        
        def min_swaps_to_sort(arr):
            n = len(arr)
            # Array of [value, original index] pairs
            arr_pos = [[v, i] for i, v in enumerate(arr)]
            # Sort by value
            arr_pos.sort()
            
            visited = [False] * n
            swaps = 0
            
            for i in range(n):
                if visited[i] or arr_pos[i][1] == i:
                    continue
                    
                # Find cycle size
                cycle_size = 0
                j = i
                while not visited[j]:
                    visited[j] = True
                    j = arr_pos[j][1]
                    cycle_size += 1
                    
                # Add minimum swaps needed for this cycle    
                swaps += cycle_size - 1
                
            return swaps
        
        def bfs(root):
            if not root:
                return []
            result = []
            q = deque([root])
            
            while q:
                level_size = len(q)
                current_level = []
                
                for _ in range(level_size):
                    node = q.popleft()
                    current_level.append(node.val)  # Note: now appending node.val instead of node
                    if node.left:
                        q.append(node.left)
                    if node.right:
                        q.append(node.right)
                
                result.append(current_level)
            return result
        
        levels = bfs(root)
        total_swaps = sum(min_swaps_to_sort(level) for level in levels)
        return total_swaps
