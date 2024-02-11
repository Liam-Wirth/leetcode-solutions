from typing import List
from collections import defaultdict
class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        ans = []
        hm = defaultdict(list)
        for word in strs:
          j = ''.join(sorted(word))
          hm[j].append(word)
        for (k,v) in hm.items():
            ans.append(v)
        return ans 
