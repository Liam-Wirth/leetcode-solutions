import java.util.Arrays;
import java.util.Collections;

/*
 * @lc app=leetcode id=914 lang=java
 *
 * [914] X of a Kind in a Deck of Cards
 *
 * https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/description/
 *
 * algorithms
 * Easy (32.83%)
 * Likes:    1282
 * Dislikes: 310
 * Total Accepted:    87.6K
 * Total Submissions: 266.9K
 * Testcase Example:  '[1,2,3,4,4,3,2,1]'
 *
 * In a deck of cards, each card has an integer written on it.
 *
 * Return true if and only if you can choose X >= 2 such that it is possible to
 * split the entire deck into 1 or more groups of cards, where:
 *
 *
 * Each group has exactly X cards.
 * All the cards in each group have the same integer.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: deck = [1,2,3,4,4,3,2,1]
 * Output: true
 * Explanation: Possible partition [1,1],[2,2],[3,3],[4,4].
 *
 *
 * Example 2:
 *
 *
 * Input: deck = [1,1,1,2,2,2,3,3]
 * Output: false
 * Explanation: No possible partition.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= deck.length <= 10^4
 * 0 <= deck[i] < 10^4
 *
 *
 */

// @lc code=start
class Solution {
    public boolean hasGroupsSizeX(int[] deck) {
        Arrays.sort(deck);
        int tmp = deck.length;
        int count[] = new int[tmp];
        for (int i = 0; i < tmp; i++) {
           count[deck[i]]+=1;
        }
        int max = count[1];
        for (int i : count) {
            if(max!= i) return false;
        }
        return true;
}
}
// @lc code=end

