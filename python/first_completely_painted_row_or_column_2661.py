class Solution:
    def firstCompleteIndex(self, arr: List[int], mat: List[List[int]]) -> int:
        m, n = len(mat), len(mat[0])
        
        # Create mapping of value to position
        valpos = {}
        for i in range(m):
            for j in range(n):
                valpos[mat[i][j]] = (i, j)
        
        rowcount = [0] * m
        colcount = [0] * n

        # Process array elements
        for idx, num in enumerate(arr):
            row, col = valpos[num]
            # mark the cell
            rowcount[row] += 1
            colcount[col] += 1

            # Check if row is complete OR if column is complete
            # Note: column check should use 'm' not 'n'
            if rowcount[row] == n or colcount[col] == m:
                return idx
                
        return -1

