class Solution:
    def replaceElements(self, arr: List[int]) -> List[int]:
        out = [0] * len(arr)
        
        
        best = 0
        bestpos = 0
        
        
        def find_best(pos: int) -> (int, int):
            b = -1
            bpos = 0
            for i in range(pos, len(arr)):
                b = max(arr[i], b)
                if b == arr[i]:
                    bpos = i
            return (bpos, b)
            
        
        bestpos, best = find_best(0)
        for (i,c) in enumerate(arr):
            if i == bestpos:
                bestpos, best = find_best(i+1)
            out[i] = best
        return out
