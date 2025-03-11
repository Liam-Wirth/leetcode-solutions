class Solution:
    class log:
        def __init__(self, t: bool, og: str, i: str, val: str):
            self.t = t  # True if alphanumeric, False if numeric
            self.og = og  # Original log string
            self.i = i  # Identifier (first part of the log)
            self.val = val  # Value part of the log (everything after the identifier)
            
    def reorderLogFiles(self, logs: List[str]) -> List[str]:
        alphas = []
        nums = []
        
        for l in logs:
            lg = l.split(" ", 1)  # Split only on the first space
            identifier = lg[0]  # The first part is the identifier
            value = lg[1] if len(lg) > 1 else ""  # The rest is the value
            is_alpha = value.replace(" ", "").isalpha()  # Check if the value is alphabetic
            
            cur = self.log(is_alpha, l, identifier, value)  # Create a log object
            if is_alpha:
                alphas.append(cur)  # Add to alphas if it's alphanumeric
            else:
                nums.append(l)  # Add to nums if it's numeric

        # Sort alphas by value, then by identifier
        alphas.sort(key=lambda x: (x.val, x.i))

        # Combine sorted alphas with nums
        sorted_logs = [log.og for log in alphas] + nums
        
        return sorted_logs


