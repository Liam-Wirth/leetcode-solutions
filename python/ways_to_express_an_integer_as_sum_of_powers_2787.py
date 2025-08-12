class Solution:
    def numberOfWays(self, n: int, x: int) -> int:
        MOD = 10**9 + 7
        
        # Create cache for memoization
        dp = {}
        
        def dfs(curr_sum, start_num):
            # Base cases
            if curr_sum == n:
                return 1
            if curr_sum > n:
                return 0
            
            # Check if we've already computed this state
            state = (curr_sum, start_num)
            if state in dp:
                return dp[state]
            
            ways = 0
            # Try adding next numbers powered to x
            i = start_num
            while True:
                power = pow(i, x)
                if curr_sum + power > n:
                    break
                # Include current number powered to x
                ways = (ways + dfs(curr_sum + power, i + 1)) % MOD
                i += 1
            
            dp[state] = ways
            return ways
        
        return dfs(0, 1)
