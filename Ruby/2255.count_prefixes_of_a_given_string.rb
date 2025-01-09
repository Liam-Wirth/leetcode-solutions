# @param {String[]} words
# @param {String} s
# @return {Integer}
    def count_prefixes(words, s)
        count = 0
        words.each do |word|
            count += 1 if s.start_with?(word)
        end
        count
    end

