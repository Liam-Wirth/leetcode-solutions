class Solution:
    def finalValueAfterOperations(self, operations: List[str]) -> int:
        out = 0
        for op in operations:
            match op:
                case "++X":
                    out+=1
                case "X++":
                    out +=1
                case "--X":
                    out-=1
                case "X--":
                    out-=1
        return out

