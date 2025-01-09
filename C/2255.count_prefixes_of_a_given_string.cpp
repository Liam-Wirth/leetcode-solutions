#include <string>
#include <vector>
class Solution {
public:
  int countPrefixes(vector<std::string> &words, std::string s) {
    int count = 0;
    for (const std::string &word : words) {
      if (s.substr(0, word.size()) == word) {
        count++;
      }
    }
    return count;
  }
};
