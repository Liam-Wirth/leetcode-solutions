class Solution:
    def defangIPaddr(self, address: str) -> str:
        out = ""
        for i in range(0,len(address)):
            if address[i] == ".":
                out += "[.]"
            else:
                out += address[i]
        return out

