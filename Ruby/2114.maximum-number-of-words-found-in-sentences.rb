# @lc code=start
# @param {String[]} sentences
# @return {Integer}
def most_words_found(sentences)
  out = 0;
  sentences.each { |sentence|
    tmp = sentence.split().length
    if tmp > out
     out = tmp;
    end
   }
   return out
end
# @lc code=end

