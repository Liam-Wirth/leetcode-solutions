# TODO: Revisit this It was tricky for me to think about and I had to rely on a video solution
class Solution:
    def numSubmat(self, mat: List[List[int]]) -> int:
        m = len(mat)
        n = len(mat[0])
        
        counts = [[0] * n for _ in range(m)]
        
        for i in range(m):
            for j in range(n-1, -1, -1):
                if mat[i][j] == 1:
                    counts[i][j] += 1 + (counts[i][j+1] if j< n -1 else 0)
        ans = 0
        for i in range(m):
            for j in range(n):
                if mat[i][j] == 1:
                    minw = sys.maxsize
                    for k in range(i, m): # check downwards
                        minw = min(minw, counts[k][j])
                        ans += minw
        return ans
