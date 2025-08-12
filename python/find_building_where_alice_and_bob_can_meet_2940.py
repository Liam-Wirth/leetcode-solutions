class Solution:
    def leftmostBuildingQueries(self, heights: List[int], queries: List[List[int]]) -> List[int]:
        n = len(heights)
        next_higher = [[] for _ in range(n)]
        stack = []
        
        # Build the reachability lists
        for i in range(n-1, -1, -1):
            while stack and heights[stack[-1]] <= heights[i]:
                stack.pop()
                
            if stack:
                next_higher[i].append(stack[-1])
                next_higher[i].extend(next_higher[stack[-1]])
                
            stack.append(i)
            
        # Handle queries
        answers = []
        for q in queries:
            a, b = q[0], q[1]
            
            # If same position, that's the answer
            if a == b:
                answers.append(a)
                continue
                
            # Ensure a is the smaller index
            if a > b:
                a, b = b, a
                
            # If b is higher than a and reachable, that's the answer
            if heights[b] > heights[a]:
                answers.append(b)
                continue
                
            # Find the leftmost building both can reach
            min_height = max(heights[a], heights[b])
            found = False
            
            # Check reachable buildings from both positions
            for pos in next_higher[a]:
                if (heights[pos] > min_height and 
                    (b < pos and heights[b] < heights[pos] or pos in next_higher[b])):
                    answers.append(pos)
                    found = True
                    break
                    
            if not found:
                answers.append(-1)
                
        return answers

