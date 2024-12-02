class Solution:
    def isPrefixOfWord(self, sentence: str, searchWord: str) -> int:
        sent = sentence.split()
        for i in range(len(sent)):
            if sent[i].startswith(searchWord):
                return i + 1
        return -1
