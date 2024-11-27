# TODO: Revisit this problem
class Solution:

    def encode(self, strs: List[str]) -> str:
        # I'm gonna encode with my delimiter being a number + the length of the next string
        # stolen straight from the neetcode video because I am stoopit

        out = ""

        for i in strs:
            out += str(len(i)) + "!" + i
        print(out)
        return out

    def decode(self, s: str) -> List[str]:
        out = []
        i = 0
        while i < len(s):
            j = i
            while s[j] != "!":
                j += 1
            wlen = int(s[i:j])
            i = j + 1
            j = i + wlen
            out.append(s[i:j])
            i = j
        return out
