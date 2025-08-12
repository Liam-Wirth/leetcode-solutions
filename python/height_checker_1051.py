class Solution:
    def heightChecker(self, heights: list[int]) -> int:
        def counting_sort(heights):
            counts = {}
            minv, maxv = min(heights), max(heights)

            for i in heights:
                if i in counts:
                    counts[i] += 1
                else:
                    counts[i] = 1

            idx = 0
            for val in range(minv, maxv + 1):
                while counts.get(val, 0) > 0:
                    heights[idx] = val
                    idx += 1
                    counts[val] -= 1
        sorteds = heights[:]    
        counting_sort(sorteds)
        ans = 0
        for i in range(len(sorteds)):
            if heights[i] != sorteds[i]:
                ans += 1
        return ans

